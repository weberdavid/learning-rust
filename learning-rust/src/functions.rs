// FUNCTIONS, MODULES & ERROR HANDLING

fn main() {

    // return multiple things
    let (result, sth) = divide_numbers(6, 4);
    println!("{}, {}", result, sth);

    // work with optional parameters
    let new_result = multiply_numbers(2, Some(4));
    println!("{}", new_result);

    let new_result2 = multiply_numbers(2, None);
    println!("{}", new_result2);

}

fn divide_numbers(a: i32, b: i32) -> (i32, bool) {
    if b == 0 {
        return (0, false);
    }

    let result = a / b;
    return (result, true);
}


// Option: optional parameter, expects a type in <> or None
// Some: is used to put a value to an optional parameter

fn multiply_numbers(a: i32, b: Option<i32>) -> i32 {
    let result = if let Some(b) = b {a*b} else {a};
    return result
}