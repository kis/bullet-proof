pub mod library {
    pub fn greet(target: String) {
        println!("Hello, {}", target);
    }
    
    pub fn print_str(msg: &str) {
        println!("{}", msg);
    }
    
    pub fn needs_string<T: ToString>(almost_string: T) {
        let real_string = almost_string.to_string();
        println!("{}", real_string);
    }
}