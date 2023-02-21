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

const TEST_UPPER: &str = "asdasdasd";

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
}