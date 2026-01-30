mod kdl_test;
use colored::Colorize;
use kdl_test::decoder_exe::DecoderExe;
use kdl_test::test_cases::TestCase;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kdl-test")]
#[command(about = "An implementation-agnostic test suite for KDL", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run tests
    Run(RunArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Run(args) => run_tests(args),
    }
}

/***** Run tests *****/

#[derive(Parser, Debug)]
struct RunArgs {
    /// Path to decoder executable
    #[arg(long, value_parser = validate_executable)]
    decoder: PathBuf,

    /// Test to skip (may be specified multiple times).
    /// e.g. `--skip valid/zero_space_before_slashdash_arg.kdl`
    #[arg(long)]
    skip: Vec<String>,

    /// Specific tests to run.
    /// e.g. `valid/arg_bare.kdl`
    tests: Vec<String>,
}

fn validate_executable(s: &str) -> Result<PathBuf> {
    which::which(s).with_context(|| format!("Could not find executable '{}'", s))
}

fn run_tests(args: RunArgs) -> Result<()> {
    let decoder = DecoderExe::new(args.decoder);

    let (valid_tests, invalid_tests) = kdl_test::test_cases::load()?;
    let all_tests = valid_tests
        .iter()
        .map(|t| t as &dyn TestCase)
        .chain(invalid_tests.iter().map(|t| t as &dyn TestCase));

    let mut passes = 0;
    let mut failures = 0;
    let mut skipped = 0;
    for test in all_tests {
        let test_name = &test.name().to_string();
        if !args.tests.is_empty() && !args.tests.contains(test_name) {
            // Don't even show in output
            continue;
        }

        print!("{}", test_name);

        if args.skip.contains(test_name) {
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
                let mut input = String::from_utf8_lossy(test.input());
                if !input.ends_with("\n") {
                    input += "\n";
                }
                println!(" {}\nInput:\n{}\n{}", "FAIL".red(), input, e);
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
