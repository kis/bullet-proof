pub mod io {
    use std::{fs::read_to_string, error::Error};

    // https://crates.io/crates/markdown

    pub fn render_markdown(file: &str) -> Result<String, Box<dyn Error>> {
        let source = read_to_string(file)?;
        Ok(markdown::to_html(&source))
    }
}