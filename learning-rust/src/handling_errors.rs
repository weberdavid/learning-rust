// Is done with the Rust type "Result" and its enums "Ok" and "Err"
// result is defined as follows:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Divide either returns an i32 or a Sring
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Cannot divide by zero".to_string())     // 3. Error raised
    } else {
        return Ok(x / y)
    }
}



fn main() {
    // Any call to divide, needs to account for all cases defined in Result
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }


    // this is a shortcut, to make it less verbose
    let result: Result<i32, &str> = Ok(42);
    let value = result.unwrap();
    println!("{}", value);
    // BUT: If called on Err, method will panic and program fails.


    // another option: ?
    // "?" propagates and error up the call stack.
    fn calculate(x: i32, y: i32) -> Result<i32, String> {
        let result = divide(x, y)?;         // 2. calling divide(10, 0) 
                                            // 4. error returned to caller Err("Cannot divide by zero")
        return Ok(result * 2)               // thus shortcutting this line
    }

    match calculate(10, 0) {                // 1. calling calculate(10, 0)
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error), // 5. Error displayed
    }

}

