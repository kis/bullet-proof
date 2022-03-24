pub struct Worker {
    pub data: Vec<u32>,
}

impl Worker {
    pub fn do_work(&mut self) -> Option<u32> {
        self.data.pop()
    }
}