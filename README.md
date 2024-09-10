# Message Stress Test tool
## Usage
```
Usage: mst [OPTIONS] --nthreads <NTHREADS> --duration-secs <DURATION_SECS>

Options:
      --input-type <input-file>
          Input file path [default: Default]
      --output-type <OUTPUT_TYPE>
          Output type [default: std-out] [possible values: std-out, kafka]
      --bootstrap-server <BOOTSTRAP_SERVER>
          Kafka bootstrap server
      --topic <TOPIC>
          Kafka topic
      --nthreads <NTHREADS>
          Number of threads
      --duration-secs <DURATION_SECS>
          Duration in seconds
  -h, --help
          Print help
  -V, --version
          Print version
```
