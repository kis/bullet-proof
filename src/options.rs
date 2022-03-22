pub mod options {
    pub fn returns_some() -> Option<String> {
        Some("my_str".to_owned())
    }

    pub fn returns_none() -> Option<String> {
        None
    }
}