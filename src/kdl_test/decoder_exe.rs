use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
use std::process::{self, Command, Output};

pub struct DecoderExe {
    decoder: PathBuf,
}

impl DecoderExe {
    pub fn new(decoder: PathBuf) -> Self {
        Self { decoder }
    }

    pub fn run(&self, input: &[u8]) -> Result<Output> {
        let mut child = Command::new(&self.decoder)
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn process: {}", self.decoder.display()))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input)?;
        }
        let output = child
            .wait_with_output()
            .with_context(|| format!("Failed to wait on process: {}", self.decoder.display()))?;
        Ok(output)
    }
}
