// BORROWING
// https://datawithrust.com/chapter_2/chapter_2_7.htm
// refers to the mechanism of allowing multiple references to access
// the same piece of data, without causing data races or memory unsafety issues


// In Rust, we can either 

fn main () {
    let mut x = 5;
    {
        let y = &mut x;    // mutable reference to x
        *y += 1;
    
    } // y goes out of scope, allowing x to be borrowed again

    let z = &x;     // immutable reference to x 

    println!("The value of x is: {}", z);
}