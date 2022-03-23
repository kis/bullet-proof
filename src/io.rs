use std::fs::read_to_string;
use crate::errors::MyError;

// https://crates.io/crates/markdown

pub fn render_markdown(file: &str) -> Result<String, MyError> {
    let source = read_to_string(file)?;
    Ok(markdown::to_html(&source))
}