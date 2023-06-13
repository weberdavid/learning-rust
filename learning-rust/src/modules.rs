// containers, in which you can group related functions and data types
// used to organize code, like Java namespaces.
// can be created in single files

mod some_module {

    const CONSTANT: u32 = 42;

    // public function
    pub fn some_pub_func() {
        println!("hello, this is it: {}", CONSTANT)
    }

    // private function
    fn some_private_func() {
        println!("Password is: ...");
    }

    struct SomeStruct {
        potato: String,
    }
}

// you can also import from other files like this
mod submodule;

fn main() {
    some_module::some_pub_func();
    submodule::test_func();
}


