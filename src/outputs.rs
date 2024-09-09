use std::time::Duration;
use clap::ValueEnum;
use futures::executor::block_on;
use rdkafka::ClientConfig;

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputType {
    StdOut,
    Kafka,
}

pub trait Output {
    fn init(&mut self, seq: i64);
    fn write(&mut self, data: &str);
    fn summarize(&self);
    fn count(&mut self);
    fn get_count(&self) -> i64;
}

pub struct StdOutput {
    count: i64,
}

impl Output for StdOutput {
    fn init(&mut self, seq: i64) {
        println!("StdOutWriter initialized with seq: {}", seq);
    }

    fn write(&mut self, data: &str) {
        println!("{}", data);
    }

    fn summarize(&self) {
        // show count of written data
        println!("StdOutWriter closed, count: {}", self.count);
    }

    fn count(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i64 {
        self.count
    }
}

impl StdOutput {
    pub(crate) fn new() -> StdOutput {
        StdOutput { count: 0 }
    }
}
pub struct KafkaOutput {
    count: i64,
    topic: String,
    seq: i64,
    producer: rdkafka::producer::FutureProducer
}

impl Output for KafkaOutput {
    fn init(&mut self, seq: i64) {
        println!("KafkaWriter initialized with seq: {}", seq);
    }

    fn write(&mut self, data: &str) {
        // produce data to kafka
        let future = self.produce(data);
        block_on(future);
    }

    fn summarize(&self) {
        // show count of written data
        println!("KafkaWriter closed, count: {}", self.count);
    }

    fn count(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> i64 {
        self.count
    }
}

impl KafkaOutput {
    pub fn new(bootstrap_servers: String, topic: String) -> KafkaOutput {
        KafkaOutput {
            count: 0,
            topic,
            seq: 0,
            producer: ClientConfig::new()
                .set("bootstrap.servers", bootstrap_servers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error"),
        }
    }

    async fn produce(&self, data: &str) {
        // produce data to kafka
        let _ = self.producer.send(
            rdkafka::producer::FutureRecord::to(&self.topic)
                .payload(data)
                .key("key"),
            Duration::from_secs(0),
        ).await;
    }
}