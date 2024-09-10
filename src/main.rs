mod inputs;
mod outputs;

use std::time::Duration;
use clap::Parser;
use outputs::{KafkaOutput, Output, OutputType};
use inputs::{load_data, InputType};
use crate::outputs::StdOutput;

#[derive(Parser)]
#[clap(name = "mp", version = "0.1.0", author = "Chien-De Li", about="Multi-threaded message producer")]
struct Config {
    #[clap(long = "input-file", help = "Input file path", value_parser(inputs::InputType::from_str), default_value = "Default", name="input-file")]
    input_type: InputType,
    #[clap(long, default_value = "std-out", help = "Output type")]
    output_type: OutputType,
    #[clap(long, required_if_eq("output_type", "kafka"), help = "Kafka bootstrap server")]
    bootstrap_server: Option<String>,
    #[clap(long, required_if_eq("output_type", "kafka"), help = "Kafka topic")]
    topic: Option<String>,
    #[clap(long, help = "Number of threads", required = true)]
    nthreads: i64,
    #[clap(long, help = "Duration in seconds", required = true)]
    duration_secs: u64,
}

fn run_single_thread_writer(data: Vec<String>, mut writer: Box<dyn Output>, duration: Duration, seq: i64) -> i64
{
    writer.init(seq);
    let start = std::time::Instant::now();
    while start.elapsed() < duration {
        let idx = rand::random::<usize>() % data.len();
        writer.write(&data[idx]);
        writer.count();
    }
    writer.summarize();
    writer.get_count()
}

fn main() {
    let config = Config::parse();

    let duration_secs = config.duration_secs;
    let nthreads = config.nthreads;

    let data = match config.input_type {
        InputType::Default => load_data(config.input_type),
        InputType::File(_) => load_data(config.input_type)
    };

    let handlers: Vec<_> = (0..nthreads).map(|seq| {
        let data = data.clone();
        let writer:Box<dyn Output + Send> = match config.output_type {
            OutputType::StdOut => Box::new(StdOutput::new()),
            OutputType::Kafka => Box::new(KafkaOutput::new(config.bootstrap_server.clone().unwrap(), config.topic.clone().unwrap()))
        };
        std::thread::spawn(move || {
            run_single_thread_writer(data, writer, Duration::from_secs(duration_secs), seq)
        })
    }).collect();

    let mut all_counter = 0i64;

    for handler in handlers {
        all_counter = all_counter + handler.join().unwrap();
    }

    let tps: f64 = all_counter as f64 / 5.0;
    println!("Total TPS: {}", tps);
}
