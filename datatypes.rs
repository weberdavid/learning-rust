// interesting video: https://youtu.be/z-0-bbc80JM

fn main() {
    // INTEGERS
    // signed or unsigned (unsigned cannot be negative)
    // available in different bit sizes: 8, 16, 32, 64, 128
    // default: i32 and u32
    let signed: i32 = -12;
    let unsiged: u32 = 21;


    // FLOATS
    //f32 (single-precision) and f64 (double-precision)
    let single: f32 = 3.14;
    let double: f64 = 2.71828;
    let standard = 3.1234;       //f64


    // BOOLEANS
    let is_true: bool = true;
    let is_false: bool = false;


    // CHARS & STRINGS
    // works also for Unicode, not just ASCII
    let letter: char = 'A';
    let emoji: char = '😃';

    // additionally two main string types:
    // s1: String, heap-allocated string
    // s2: &str, string slice
    let s1 = String::from("hello");
    let s2 = "world".to_string();


    // COMPOUND TYPES
    // = group multiple values into one
    
    // TUPLES
    let tuple: (i32, f64, bool, char) = (32, 3.13, true, 'A');


    // ARRAYS
    // fixed-size collection, each value has same type
    let array: [i32; 5] = [1, 2, 3, 4, 5];


    // CUSTOM DATA TYPES
    // allows to create custom types, very powerful

    // STRUCTS
    // = create custom data types, that group related values together - either named or unnamed fields

    // Named fields
    struct Point {
        x: f64,
        y: f64,
    }

    // Tuple struct = Unnamed field
    struct Color(u8, u8, u8);

    let point = Point{ x: 1.0, y: 2.0 };
    let color = Color(255, 0, 0);


    // ENUMS
    // define a type which can have multiple variants, each with its own associated data
    enum Direction {
        North,
        South,
        West,
        East
    }
    let direction = Direction::North;

    // enums can be generic, so they can take type parameters
    // which allows to define an enum that can work with multiple types
    // T represents the type that could be present in the enum
    // using it this way, you can handle a value being present or absent
    enum Option<T> {
        Some(T),
        None,
    }



    //CUSTOM DATA TYPE BEHAVIOUR
    
    // impl
    // "implementation", used to define methods for a data type
    // similar to OOP
    struct Plane {
        name: String,
    }

    impl Plane {
        // function to create instance of plane with given name
        fn new(name: &str) -> Plane {
            return Plane {
                name: name.to_string()
            }
        }

        // fn to make instance do sth
        fn do_barrel_roll(&self) {
            println!("{} is doing a barrel roll!!", self.name);
        }
    }

    // create instance
    let plane = Plane::new("Falcon");
    plane.do_barrel_roll();



    // trait
    // define behaviour a data type should have
    // extend specific data types with additional behaviour
    trait CanVTOL {
        fn v_takeoff(&self);

        fn v_landing(&self);
    }

    // ensure every instance of Plane van takeoff & land virtually
    impl CanVTOL for Plane {
        fn v_takeoff(&self) {
            println!("Taking off vertically!")
        }

        fn v_landing(&self) {
            println!("Landing vertically!")
        }
    }

    let plane2 = Plane::new("Falcon2");
    plane2.v_takeoff();
    plane2.do_barrel_roll();
    plane2.v_landing();


    // WHEN TO USE WHICH?
    // impl VS trait

    // Use impl when:
    // - ýou want to define implementations that are closely tied to the implementation of the data type
    // - you want to define methods that take ownership of the data type or modify internal state
    // - you want to implement behaviour that is unique to a specific data type

    // Use trait when:
    // - you want to define generic behaviour that can be used with multiple data types
    // - you want to define behaviour that can be shared among different data types
    // - f.ex. define a Serialize trait that specifies how to serialise a data type into some format,
    //      which can be implemented by the different data types and use different serialization libraries


















}