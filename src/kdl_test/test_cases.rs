use std::borrow::Cow;
use std::path::PathBuf;
use std::process::Output;

use anyhow::{Context, Result, anyhow, bail};

use crate::kdl_test::test_files::TestFiles;

pub struct ValidTestCase {
    pub name: Cow<'static, str>,
    pub input: Cow<'static, [u8]>,
    pub expected: serde_json::Value,
}

pub struct InvalidTestCase {
    pub name: Cow<'static, str>,
    pub input: Cow<'static, [u8]>,
}

pub fn load() -> Result<(Vec<ValidTestCase>, Vec<InvalidTestCase>)> {
    let mut valid_tests = Vec::new();
    let mut invalid_tests = Vec::new();
    for (filepath, file) in TestFiles::iter_files() {
        let path = PathBuf::from(filepath.as_ref());
        if path.extension() != Some("kdl".as_ref()) {
            continue;
        }

        let input = file.data;
        if let Some(std::path::Component::Normal(dir)) = path.components().next() {
            if dir == "valid" {
                let expected_file = path.with_extension("json").to_string_lossy().into_owned();
                let expected_file_result = TestFiles::get(&expected_file)
                    .with_context(|| format!("Expected file does not exist: {}", expected_file))?;
                let expected = serde_json::from_slice(&expected_file_result.data)
                    .with_context(|| format!("File is invalid json: {}", expected_file))?;
                valid_tests.push(ValidTestCase {
                    name: filepath,
                    input,
                    expected,
                })
            } else if dir == "invalid" {
                invalid_tests.push(InvalidTestCase {
                    name: filepath,
                    input,
                })
            } else {
                continue;
            }
        }
    }
    Ok((valid_tests, invalid_tests))
}

// Ok(()) = Test pass
// Err(e) = Test failed
type TestResult = Result<()>;

pub trait TestCase {
    fn name(&self) -> &str;
    fn input(&self) -> &[u8];
    fn get_result(&self, output: Output) -> TestResult;
}

impl TestCase for ValidTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn input(&self) -> &[u8] {
        &self.input
    }

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

impl TestCase for InvalidTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn input(&self) -> &[u8] {
        &self.input
    }

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
