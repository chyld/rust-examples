use std::mem;

#[derive(Debug)]
struct Mega(i32);

#[derive(Debug)]
struct SuperMega(Mega);

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f64);

// A struct with two fields
struct Point {
    x: u8,
    y: f32,
}

// ? -> Debug
// o -> Octal
// x -> LowerHex
// X -> UpperHex
// p -> Pointer
// b -> Binary
// e -> LowerExp
// E -> UpperExp

fn main() {
    printing();
    tuples();
    arrays();
    structs();
}

fn structs() {
    let a = Nil;
    let b = Pair(3, 5.2);
    let c = Point {x: 4, y: 2.5};
}

fn printing() {
    println!("Hello, world!");
    println!("{0} + {1} = {2}", 1, 2, 3);
    println!("{x} and {y}", x=3, y=4);
    println!("Mega is {0:?}, SM is {1:?}", Mega(4), SuperMega(Mega(5)));
}

fn tuples() {
    let pair1 = ('X', 3);
    println!("{:?}", pair1);
    let (x, y) = pair1;
    let pair2 = (y, x);
    println!("{:?}", pair2);
}

fn arrays() {
    let a = [1,2,3];    // i32
    let b: [u64; 10];   // u64 - 10 random values
    let c: [u8; 5] = [7; 5];  // u8 - initialized to 7
    let d = [12; 20]; // i32 - 20 values initialized to 12
    println!("length a {0}, size {1}", a.len(), mem::size_of_val(&a));
    //println!("length b {0}, size {1}", b.len(), mem::size_of_val(&b));
    println!("length c {0}, size {1}", c.len(), mem::size_of_val(&c));
    println!("length d {0}, size {1}", d.len(), mem::size_of_val(&d));

    let e: [&str; 3] = ["dog", "cat", "horse"];
    println!("e is {:?}, len {}, size {}", e, e.len(), mem::size_of_val(&e));

    let f = &e[1..3];
    println!("f is {:?}", f);
}
