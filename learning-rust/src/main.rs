fn main() {
    println!("Hello, world! Happy to start learning");


    // variables
    let x = 5; //immutable variable

    let mut y = 4; //mutable variable
    y = 7 ;          // changing the variable

    //const cannot be changed, it is immutable
    const Y2K: i32 = 2000; //const variables require known type

    //reference to a fixed location in memory
    //similar to global variable
    //can be mutated with some conditions (unsafe code blocks)
    static mut POTATOES: u32 = 0; //mutable static variable

    //using unsafe is not recommend, should be avoided if possible
    unsafe {
        POTATOES = 1;
    }




    // if else
    let z = 5;

    if(z < 0){
        println!("Negative");
    } else if(x > 0) {
        println!("Positive");
    } else {
        println!("Zero");
    }

    // match

    match z {
        n if n < 0 => println!("Negative"),
        n if n > 0 => println!("Positive"),
        _ => println!("Zero"), // match needs all possible outcomes to be handled
    }



    // Loops
    



















}


