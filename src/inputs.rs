use std::fs::File;
use std::io::BufRead;

#[derive(Clone)]
pub enum InputType {
    Default,
    File(String),
}

impl InputType {
    pub fn from_str(input: &str) -> Result<InputType, String> {
        match input {
            "Default" => Ok(InputType::Default),
            _ => Ok(InputType::File(input.to_string())),
        }
    }
}
pub fn generate_default_data() -> Vec<String> {
    vec![
        "Hello, world!".to_string(),
        "Hello, rust!".to_string(),
        "Hello, cargo!".to_string(),
    ]
}

pub fn load_data(source: InputType) -> Vec<String> {
    match source {
        InputType::Default => generate_default_data(),
        InputType::File(path) => {
            let file = File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            reader.lines().map(|line| line.unwrap()).collect()
        }
    }
}