mod utils;
mod lib;
mod options;
mod result;
mod io;
mod errors;
mod closures;
mod loops;

use std::collections::HashMap;
use std::str::FromStr;
use std::net::Ipv4Addr;
use std::time::Instant;
use std::fmt::Display;
use tokio::task::JoinHandle;

use utils::Light;
use utils::house_light::HouseLight;
use utils::traffic_light::TrafficLight;

use lib::library::{greet, print_str, needs_string};

use options::options::{returns_some, returns_none};
use result::result::{returns_ok, returns_err};
use io::render_markdown;
use errors::MyError;
use closures::{make_adder, DynamicBehavior};
use loops::Worker;

fn print_state(light: &impl Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}

fn maps() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("{}", map.get("key1").unwrap_or(&""));
    println!("{}", map.get("key2").unwrap_or(&""));

    for prop in map.keys() {
        println!("key {} in map, value {}", prop, map.get(prop).unwrap());
    }
}

fn structs() {
    let mut traffic_light = TrafficLight::new();
    println!("{}", traffic_light);
    println!("{}", traffic_light.get_state());
    println!("{:?}", traffic_light.get_state());
    traffic_light.turn_green();
    println!("{}", traffic_light.get_state());
    println!("{:?}", traffic_light.get_state());
    traffic_light.turn_yellow();
    println!("{}", traffic_light.get_state());
    println!("{:?}", traffic_light.get_state());
    println!("{}", traffic_light.get_name());

    print_state(&traffic_light);
}

fn traits() {
    let house_light = HouseLight::new();
    println!("{}", house_light.get_state());
    println!("{}", house_light.get_name());

    print_state(&house_light);
}

fn strings() {
    let string_slice = "String slice assigned to variable";
    let real_string = "Genuine string".to_owned();
    print_str(string_slice);
    print_str("Literal slice");
    print_str(&real_string);

    let ip_address = Ipv4Addr::from_str("127.0.0.1").unwrap();
    let string_proper = "String proper".to_owned();
    let string_slice = "string slice";
    needs_string(string_slice);
    print_str("Literal slice");
    needs_string(string_proper);
    needs_string(ip_address);

    // unwrap syntax
    let default_string = "Default value".to_owned();
    let unwrap_or = returns_none().unwrap_or(default_string);
    println!("returns_none().unwrap_or(...): {:?}", unwrap_or);

    let unwrap_or_else = returns_none()
        .unwrap_or_else(|| format!("Default value from a function at time {:?}", Instant
        ::now()));
    println!(
        "returns_none().unwrap_or_else(|| {{...}}): {:?}",
        unwrap_or_else
    );

    let default_unwrap = returns_none().unwrap_or_default();
    println!("returns_none().unwrap_or_default(...): {:?}", default_unwrap);
}

fn opts() {
    let some = returns_some();
    println!("{:?}", some);

    let none = returns_none();
    println!("{:?}", none);
}

fn res() {
    let ok = returns_ok();
    println!("{:?}", ok);

    let err = returns_err();
    println!("{:?}", err);
}

fn io() -> Result<(), MyError> {
    let html = render_markdown("./README.md")?;
    println!("{}", html);
    Ok(())
}

fn compose<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |i: T| f(g(i))
}

fn looping() {
    let mut obj = Worker {
        data: vec![1, 2, 3, 4],
    };

    while let Some(data) = obj.do_work() {
        println!("loop {}", data)
    }

    let arr = vec![2, 4, 6];
    let doubled: Vec<_> = arr.iter().map(|num| num * 2).collect();
    println!("doubled {:?}", doubled);
}

async fn async_run() -> String {
    "I'm async".to_owned()
}

fn async_print<T: Display + Send + 'static>(msg: T) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        println!("task {}", msg);
    })
}

#[tokio::main]
async fn main() {
    greet("World".to_owned());
    maps();
    structs();
    traits();
    strings();
    opts();
    res();
    io();

    let plus_two = make_adder(2);
    plus_two(32);
    let plus_three = make_adder(3);
    let times_two = |i: i32| i * 2;
    let compose_them = compose(plus_three, times_two);
    println!("{} * 2 + 3 = {}", 10, compose_them(10));

    let square = DynamicBehavior::new(Box::new(|num: i64| num * num));
    println!("{} squared to {}", 5, square.run(5));

    looping();

    let msg = async_run().await;
    println!("async {}", msg);

    let async_closure = || async {
        "Async closure".to_owned()
    };
    let closure_msg = async_closure().await;
    println!("async closure {}", closure_msg);

    async_print("Hello");
}
