use anyhow::{Context, Result};
use rust_embed::Embed;
use std::borrow::Cow;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "test_cases/"]
struct Tests;

pub struct ValidTestCase {
    pub name: Cow<'static, str>,
    pub input: Cow<'static, [u8]>,
    pub expected: serde_json::Value,
}

pub struct InvalidTestCase {
    pub name: Cow<'static, str>,
    pub input: Cow<'static, [u8]>,
}

pub trait TestCase {
    fn name(&self) -> &str;
    fn input(&self) -> &[u8];
}
impl TestCase for ValidTestCase {
    fn name(&self) -> &str {
        &self.name
    }
    fn input(&self) -> &[u8] {
        &self.input
    }
}
impl TestCase for InvalidTestCase {
    fn name(&self) -> &str {
        &self.name
    }
    fn input(&self) -> &[u8] {
        &self.input
    }
}

pub fn load() -> Result<(Vec<ValidTestCase>, Vec<InvalidTestCase>)> {
    let mut valid_tests = Vec::new();
    let mut invalid_tests = Vec::new();
    for file in Tests::iter() {
        let path = PathBuf::from(file.as_ref());
        if path.extension() != Some("kdl".as_ref()) {
            continue;
        }

        let input = Tests::get(file.as_ref()).expect("File should exist").data;
        if let Some(std::path::Component::Normal(dir)) = path.components().next() {
            if dir == "valid" {
                let expected_file = path
                    .with_extension("json")
                    .to_string_lossy()
                    .into_owned();
                let expected_file_result = Tests::get(&expected_file).with_context(|| {
                    format!("Expected file does not exist: {}", expected_file)
                })?;
                let expected =
                    serde_json::from_slice(&expected_file_result.data).with_context(|| {
                        format!("File is invalid json: {}", expected_file)
                    })?;
                valid_tests.push(ValidTestCase {
                    name: file,
                    input,
                    expected,
                })
            } else if dir == "invalid" {
                invalid_tests.push(InvalidTestCase { name: file, input })
            } else {
                continue;
            }
        }
    }
    Ok((valid_tests, invalid_tests))
}
