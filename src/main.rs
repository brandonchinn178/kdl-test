mod kdl_test;
use colored::Colorize;
use kdl_test::decoder_exe::DecoderExe;
use kdl_test::test_cases::{InvalidTestCase, TestCase, ValidTestCase};

use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use std::path::PathBuf;
use std::process::Output;

#[derive(Parser, Debug)]
#[command(name = "kdl-test")]
#[command(about = "An implementation-agnostic test suite for KDL", long_about = None)]
struct Args {
    /// Path to decoder executable
    #[arg(long, value_parser = validate_executable)]
    decoder: PathBuf,

    /// Test to skip (may be specified multiple times).
    /// e.g. `--skip valid/zero_space_before_slashdash_arg.kdl`
    #[arg(long)]
    skip: Vec<String>,
}

fn validate_executable(s: &str) -> Result<PathBuf> {
    which::which(s).with_context(|| format!("Could not find executable '{}'", s))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let decoder = DecoderExe::new(args.decoder);

    let (valid_tests, invalid_tests) = kdl_test::test_cases::load()?;
    let all_tests = valid_tests
        .iter()
        .map(|t| t as &dyn RunnableTestCase)
        .chain(invalid_tests.iter().map(|t| t as &dyn RunnableTestCase));

    let mut passes = 0;
    let mut failures = 0;
    let mut skipped = 0;
    for test in all_tests {
        print!("{}", test.name());

        if args.skip.contains(&test.name().to_string()) {
            println!(" {}", "SKIP".yellow());
            skipped += 1;
            continue;
        }

        let output = decoder.run(test.input())?;
        match test.get_result(output) {
            Ok(()) => {
                println!(" {}", "OK".green());
                passes += 1;
            }
            Err(e) => {
                println!(" {}\n{}", "FAIL".red(), e);
                failures += 1;
            }
        }
    }

    for _ in 0..80 {
        print!("{}", "-".yellow());
    }
    println!();
    println!("Tests passed: {}", passes);
    println!("Tests failed: {}", failures);
    if skipped > 0 {
        println!("Tests skipped: {}", skipped);
    }

    if failures > 0 {
        std::process::exit(1);
    }
    Ok(())
}

// Ok(()) = Test pass
// Err(e) = Test failed
type TestResult = Result<()>;

trait RunnableTestCase: TestCase {
    fn get_result(&self, output: Output) -> TestResult;
}
impl RunnableTestCase for ValidTestCase {
    fn get_result(&self, output: Output) -> TestResult {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Expected success, got:\n{}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let actual: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|_| anyhow!("Failed to decode output, got:\n{}", stdout))?;
        if actual != self.expected {
            bail!(
                "Expected:\n\
                 {}\n\
                 Got:\n\
                 {}",
                indented(json_pretty(&self.expected)),
                indented(json_pretty(&actual)),
            );
        }

        Ok(())
    }
}
impl RunnableTestCase for InvalidTestCase {
    fn get_result(&self, output: Output) -> TestResult {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            bail!("Expected failure, got:\n{}", stdout);
        }

        Ok(())
    }
}

fn json_pretty(v: &serde_json::Value) -> String {
    serde_json::to_string_pretty(v).expect("serde_json::Value should always serialize")
}

fn indented(s: String) -> String {
    s.lines()
        .map(|line| format!("    {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
