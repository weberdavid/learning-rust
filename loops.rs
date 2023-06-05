fn main() {
    // start with one, end with 9, 10 is excluded
    for n in 1..10 {
        println!("N={}", n)
    }


    // iterate through a list of things
    let languages = ["Python", "Rust", "Scala"];
    for name in languages.iter() {
        println!("I love {}!", name);
    }


    // loop: equals to "while True" - very similar
    let mut count = 0;
    loop {
        count += 1;

        if count < 10 {
            // continue doing stuff
            println!("I am doing stuff...");
            continue;
        }

        if count==10 {
            println!("Done");
            // break the loop
            break;
        }
    }

    //while version:
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("Do stuff...");

        if count==10 {
            println!("Done");
            break;
        }
    }
}














