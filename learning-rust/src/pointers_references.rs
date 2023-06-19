// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6];

    // reference a value in that list
    println!("Bevore changing: {}", &numbers[2]);

    // modify the list
    numbers[2] = 12;
    println!("After the change: {}", &numbers[2]);




    // dereference operator: *
    // used to access the value that a reference or pointer points to
    let x = &7;
    assert_eq!(*x, 7);

    // here it panicks, because left does not equal right
    let y = &9;
    assert_eq!(*y, 11);




    // Copying and Cloning Traits
    // Copy is a bitwise copy of some types that implement it natively
    // such as scalars, integers, floats, characters

    let a: i32 = 12;
    let b = a;          // a is copied to b

    // Clone trait is a more advanced Copy.
    // allows for explicit duplication of an object with more complex logic
    // than a simple bitwise copy
    // typically used for complex types and custom structs

    #[derive(Clone)] //derives clone trait for the struct
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20};
    let p2 = p1.clone();        // cloning p1
    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);














}