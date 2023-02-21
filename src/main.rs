// An attribute to hide warnings for unused code.
#![allow(dead_code)]


enum DerpEnum {
    Yes,
    No,
    Maybe
}

struct Test {
    derp: String,
    age: u32,
    date: u64,
    ok: DerpEnum
}

impl Test {
    fn new()->Test {
        return Test{derp: String::from("derpine"), age: 23, date: 23, ok: DerpEnum::Maybe}
    }

    fn testDerp(&self) -> () {
        println!("Test {0}, {1}", self.derp, self.date);
    }
}

fn testtest(t: Test) -> () {
    t.testDerp();
}

const TEST_UPPER: &str = "asdasdasd";

fn derp(a: i64) -> bool {
    return false
}

fn fizzbuzz(mut n: i32) -> () {
    n = 4;

    println!("fizzbuzz {0}", n)
}

fn main() {
    println!("Hello modafoca");

    let logical: bool = true;
    let poop: i32 = 2;

    println!("{0} {1}", logical, poop);

    let t: Test = Test{derp: String::from("derp"), age: 2, date: 3, ok: DerpEnum::Yes};

    println!("{0} {1} {2}", t.derp, t.age, t.date);

    let mut _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    _immutable_binding += 1;

    println!("1000 as a u16 is: {}", 1000 as u16);

    let mut a: i32 = 3;

    fizzbuzz(a);

    t.testDerp();

    let d: Test = Test::new();

    testtest(d);
    
    d.testDerp();

    // a = 4;

    println!("{0}", a);
}