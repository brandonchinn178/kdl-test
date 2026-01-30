use std::borrow::Cow;

use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "test_cases/"]
pub struct TestFiles;

impl TestFiles {
    pub fn iter_files() -> impl Iterator<Item = (Cow<'static, str>, EmbeddedFile)> {
        TestFiles::iter().map(|filepath| {
            let file = TestFiles::get(&filepath)
                .unwrap_or_else(|| panic!("Test file unexpectedly does not exist: {}", filepath));
            (filepath, file)
        })
    }
}
