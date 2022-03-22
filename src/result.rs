pub mod result {
    #[derive(Debug)]
    pub struct MyError(String);

    pub fn returns_ok() -> Result<String, MyError> {
        Ok("great".to_owned())
    }

    pub fn returns_err() -> Result<String, MyError> {
        Err(MyError("fail".to_owned()))
    }
}