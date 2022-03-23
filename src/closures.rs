pub fn make_adder(left: i32) -> impl Fn(i32) -> i32 {
    move |right: i32| {
        println!("{} + {} = {}", left, right, left + right);
        left + right
    }
}

pub struct DynamicBehavior<T> {
    closure: Box<dyn Fn(T) -> T>,
}

impl<T> DynamicBehavior<T> {
    pub fn new(closure: Box<dyn Fn(T) -> T>) -> Self {
        Self { closure }
    }
    
    pub fn run(&self, arg: T) -> T {
        (self.closure)(arg)
    }
}