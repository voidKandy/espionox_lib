use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Io {
    pub i: String,
    pub o: String,
}

impl Io {
    fn run_input(input: &str) -> String {
        let args: Vec<&str> = input.split(" ").collect();
        let out = Command::new(args[0])
            .args(&args[1..])
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute tmux command");
        String::from_utf8_lossy(&out.stdout).to_string()
    }

    pub fn new(input: &str) -> Io {
        Io {
            i: input.to_string(),
            o: Self::run_input(input),
        }
    }
}