// // // mod important;

// // // use std::mem;

// // // // mod s1st;
// // // // This function borrows a slice
// // // fn analyze_slice(ali: &[i32]) {
// // //     println!("first element of the slice: {}", ali[0]);
// // //     println!("the slice has {} elements", ali.len());
// // // }

// // // fn run() {
// // //     // Fixed-size array (type signature is superfluous)
// // //     let xs: [i32; 5] = [1, 2, 3, 4, 5];

// // //     // All elements can be initialized to the same value
// // //     let ys: [i32; 500] = [0; 500];

// // //     // Indexing starts at 0
// // //     println!("first element of the array: {}", xs[0]);
// // //     println!("second element of the array: {}", xs[1]);

// // //     // `len` returns the count of elements in the array
// // //     println!("number of elements in array: {}", xs.len());

// // //     // Arrays are stack allocated
// // //     println!("array occupies {} bytes", mem::size_of_val(&xs));

// // //     // Arrays can be automatically borrowed as slices
// // //     println!("borrow the whole array as a slice");
// // //     analyze_slice(&xs);

// // //     // Slices can point to a section of an array
// // //     // They are of the form [starting_index..ending_index]
// // //     // starting_index is the first position in the slice
// // //     // ending_index is one more than the last position in the slice
// // //     println!("borrow a section of the array as a slice");
// // //     analyze_slice(&ys[1..4]);

// // //     // Out of bound indexing causes compile error
// // //     // println!("{}", xs[5]);
// // // }
// // // // #[derive(Debug)]
// // // // struct Matrix(f32, f32, f32, f32);
// // // // fn reverse(pair: (i32, bool)) -> (bool, i32) {
// // // //     // `let` can be used to bind the members of a tuple to variables
// // // //     let (integer, boolean) = pair;

// // // //     (boolean, integer)
// // // // }
// // // // fn main() {
// // // // s1st::run();
// // // // important::run();

// // // // println!(
// // // //     "{} of {:b} people know binary, the other half doesn't",
// // // //     1, 3
// // // // );
// // // // println!("{number:>width$}", number = 1, width = 6);
// // // // println!("{number:>0width$}", number = 1, width = 5);
// // // // println!("{number:>5}", number = 1);
// // // // println!("{:5} {number:>5}", number = 1);

// // // // // All of these print "Hello x    !"
// // // // //println!("Hello {:5}!", "x");
// // // // //println!("Hello {:1$}!", "x", 5);
// // // // println!("Hello {1:0$}!", 5, "x");
// // // // println!("Hello {:width$}!", "x", width = 5);

// // // // assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");
// // // // assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
// // // // assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
// // // // assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");

// // // // assert_eq!(format!("Hello {:+}!", 5), "Hello +5!");
// // // // assert_eq!(format!("{:#x}!", 27), "0x1b!");
// // // // assert_eq!(format!("Hello {:05}!", 5), "Hello 00005!");
// // // // assert_eq!(format!("Hello {:05}!", -5), "Hello -0005!");
// // // // assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");

// // // // println!("Hello-{:5}!", "x");
// // // // println!("Hello-{:1$}!", "x", 5);

// // // // println!("Hello"); // => "Hello"
// // // // println!("Hello, {}!", "world"); // => "Hello, world!"
// // // // println!("The number is {}", 1); // => "The number is 1"
// // // // println!("{:?} {:?}", 3, 4); // => "(3, 4)"
// // // // println!("{value}", value = 4); // => "4"
// // // // println!("{} {}", 1, 2); // => "1 2"
// // // // println!("{:01}", 42); // => "0042" with leading zeros
// // // // println!("{0:1$}5", "i", 5);

// // // // println!("Hello-{:-<5}!", "x");
// // // // println!("Hello-{:-<5}!", "x");
// // // // println!("Hello-{:-^5}!", "x");
// // // // println!("Hello-{:>5}!", "x");

// // // // assert_eq!(format!("{:#010x}!", 27), "0x0000001b!");
// // // // println!("Hello {:5}!", "x");
// // // // println!("Hello {:<5}!", "x");

// // // // Variables can be type annotated.
// // // // let logical: bool = true;

// // // // let a_float: f64 = 1.0; // Regular annotation
// // // // let an_integer = 5i32; // Suffix annotation

// // // // // Or a default will be used.
// // // // let default_float = 3.0; // `f64`
// // // // let default_integer = 7; // `i32`

// // // // // A type can also be inferred from context
// // // // let mut inferred_type = 12; // Type i64 is inferred from another line
// // // // inferred_type = 4294967296i64;

// // // // // A mutable variable's value can be changed.
// // // // let mut mutable = 12; // Mutable `i32`
// // // // mutable = 21;

// // // // Error! The type of a variable can't be changed.
// // // // mutable = true;

// // // // Variables can be overwritten with shadowing.
// // // // let mutable = true;
// // // // println!("Hello-{:^7}!", "x");

// // // // // Integer addition
// // // // println!("1 + 2 = {}", 1u32 + 2);
// // // // // Integer subtraction
// // // // println!("1 - 2 = {}", 1i32 - 2);
// // // // // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
// // // // // Short-circuiting boolean logic
// // // // println!("true AND false is {}", true && false);
// // // // println!("true OR false is {}", true || false);
// // // // println!("NOT true is {}", !true);
// // // // // Bitwise operations
// // // // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
// // // // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
// // // // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
// // // // println!("1 << 5 is {}", 1u32 << 5);
// // // // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// // // // // Use underscores to improve readability!
// // // // println!("One million is written as {}", 1_000_000u32);
// // // // println!("{}", true && false == false);
// // // // println!("{}", 123_456);

// // // // // But long Tuples cannot be printed
// // // // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
// // // // println!("too long tuple: {:?}", too_long_tuple);

// // // // let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

// // // // // Tuples are printable
// // // // println!("tuple of tuples: {:?}", tuple_of_tuples);

// // // // let tuple = (1, "hello", 4.5, true);

// // // // let (a, b, c, d) = tuple;
// // // // println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
// // // // let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
// // // // println!("{:?}", matrix);

// // // // let pair = (1, true);
// // // // println!("pair is {:?}", pair);

// // // // important::go();
// // // // println!("the reversed pair is {:?}", reverse(pair));
// // // // }
// // // #[derive(Debug)]
// // // struct Person {
// // //     name: String,
// // //     age: u8,
// // // }

// // // // A unit struct
// // // struct Unit;

// // // // A tuple struct
// // // struct Pair(i32, f32);

// // // // A struct with two fields
// // // struct Point {
// // //     x: f32,
// // //     y: f32,
// // // }

// // // // Structs can be reused as fields of another struct
// // // #[allow(dead_code)]
// // // struct Rectangle {
// // //     // A rectangle can be specified by where the top left and bottom right
// // //     // corners are in space.
// // //     top_left: Point,
// // //     bottom_right: Point,
// // // }

// // // fn main() {
// // //     run();

// // //     // Create struct with field init shorthand
// // //     let name = String::from("Peter");
// // //     let age = 27;
// // //     let peter = Person { name, age };

// // //     // Print debug struct
// // //     println!("{:?}", peter);

// // //     // Instantiate a `Point`
// // //     let point: Point = Point { x: 10.3, y: 0.4 };

// // //     // Access the fields of the point
// // //     println!("point coordinates: ({}, {})", point.x, point.y);

// // //     // Make a new point by using struct update syntax to use the fields of our
// // //     // other one
// // //     let bottom_right = Point { x: 5.2, ..point };

// // //     // `bottom_right.y` will be the same as `point.y` because we used that field
// // //     // from `point`
// // //     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

// // //     // Destructure the point using a `let` binding
// // //     let Point {
// // //         x: top_edge,
// // //         y: left_edge,
// // //     } = point;

// // //     let _rectangle = Rectangle {
// // //         // struct instantiation is an expression too
// // //         top_left: Point {
// // //             x: left_edge,
// // //             y: top_edge,
// // //         },
// // //         bottom_right: bottom_right,
// // //     };

// // //     // Instantiate a unit struct
// // //     let _unit = Unit;

// // //     // Instantiate a tuple struct
// // //     let pair = Pair(1, 0.1);

// // //     // Access the fields of a tuple struct
// // //     println!("pair contains {:?} and {:?}", pair.0, pair.1);

// // //     // Destructure a tuple struct
// // //     let Pair(integer, decimal) = pair;

// // //     println!("pair contains {:?} and {:?}", integer, decimal);
// // // }
// // // fn main() {
// // //     // This structure cannot be printed either with `fmt::Display` or
// // //     // with `fmt::Debug`.
// // //     #[derive(Debug)]

// // //     struct UnPrintable(i32);

// // //     // The `derive` attribute automatically creates the implementation
// // //     // required to make this `struct` printable with `fmt::Debug`.
// // //     struct DebugPrintable(i32);
// // // }
// // // #[derive(Debug)]
// // // struct Person<'a> {
// // //     name: &'a str,
// // //     age: u8,
// // // }

// // // fn main() {
// // //     let name = "Peter";
// // //     let age = 27;
// // //     let peter = Person { name, age };

// // //     // Pretty print
// // //     println!("{:#?}", peter);
// // //     println!("{:?}", peter);
// // // }

// // // fn main() {
// // //     let num = 14;
// // //     if num % 3 == 0 {
// // //         println!("{} is even", num);
// // //     } else if num % 2 == 0 && num % 7 == 0 {
// // //         println!("{} is 14n like number", num);
// // //     } else {
// // //         println!("{} is odd", num);
// // //     }

// // //     let condition = 1;
// // //     let vd = if condition == 1 { 5 } else { 2 };
// // //     println!("{}", vd);
// // //     run();
// // // }

// // use rand::Rng;

// // fn main() {
// //     let mut rng = rand::thread_rng();

// //     let n1: u8 = rng.gen();
// //     let n2: u16 = rng.gen();
// //     println!("Random u8: {}", n1);
// //     println!("Random u16: {}", n2);
// //     println!("Random u32: {}", rng.gen::<u32>());
// //     println!("Random i32: {}", rng.gen::<i32>());
// //     println!("Random float: {}", rng.gen::<f64>());

// //     let mut rng = rand::thread_rng();
// //     println!("Integer: {}", rng.gen_range(0..10));
// //     println!("Float: {}", rng.gen_range(0.0..10.0));
// //     die();
// //     run();
// // }
// // use rand::distributions::{Distribution, Uniform};

// // fn die() {
// //     let mut rng = rand::thread_rng();
// //     let die = Uniform::from(1..7);

// //     loop {
// //         let throw = die.sample(&mut rng);
// //         println!("Roll the die: {}", throw);
// //         if throw == 6 {
// //             break;
// //         }
// //     }
// // }
// // #[derive(Debug)]
// // struct Person {
// //     name: String,
// //     age: u8,
// // }

// // // A unit struct
// // struct Unit;

// // // A tuple struct
// // struct Pair(i32, f32);

// // // A struct with two fields
// // struct Point {
// //     x: f32,
// //     y: f32,
// // }

// // // Structs can be reused as fields of another struct
// // #[allow(dead_code)]
// // struct Rectangle {
// //     // A rectangle can be specified by where the top left and bottom right
// //     // corners are in space.
// //     top_left: Point,
// //     bottom_right: Point,
// // }

// // fn run() {
// //     // Create struct with field init shorthand
// //     let name = String::from("Peter");
// //     let age = 27;
// //     let peter = Person { name, age };

// //     // Print debug struct
// //     println!("{:?}", peter);

// //     // Instantiate a `Point`
// //     let point: Point = Point { x: 10.3, y: 0.4 };

// //     // Access the fields of the point
// //     println!("point coordinates: ({}, {})", point.x, point.y);

// //     // Make a new point by using struct update syntax to use the fields of our
// //     // other one
// //     let bottom_right = Point { x: 5.2, ..point };

// //     // `bottom_right.y` will be the same as `point.y` because we used that field
// //     // from `point`
// //     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

// //     // Destructure the point using a `let` binding
// //     let Point {
// //         x: top_edge,
// //         y: left_edge,
// //     } = point;

// //     let _rectangle = Rectangle {
// //         // struct instantiation is an expression too
// //         top_left: Point {
// //             x: left_edge,
// //             y: top_edge,
// //         },
// //         bottom_right: bottom_right,
// //     };

// //     // Instantiate a unit struct
// //     let _unit = Unit;

// //     // Instantiate a tuple struct
// //     let pair = Pair(1, 0.1);

// //     // Access the fields of a tuple struct
// //     println!("pair contains {:?} and {:?}", pair.0, pair.1);

// //     // Destructure a tuple struct
// //     let Pair(integer, decimal) = pair;

// //     println!("pair contains {:?} and {:?}", integer, decimal);
// // }
// #[allow(dead_code)]
// fn analyze_slice(slice: &[i32]) {
//     println!("first element of the slice: {}", slice[0]);
//     println!(" {:?} ", slice);
// }
// #[allow(unused_variables)]
// fn main() {
//     // Fixed-size array (type signature is superfluous)
//     let xs = [1, 2, 3, 4, 5];

//     // All elements can be initialized to the same value
//     let mut ys: [i32; 500] = [1; 500];
//     ys[3] = 2;
//     println!("{:?}", ys);

// }

fn main() {
    struct Rectangle {
        top_right: i32,
        down_left: i32,
    }
    let mut large_rec = Rectangle {
        top_right: 0,
        down_left: 0,
    };
    println!("{} and , {}", large_rec.down_left, large_rec.top_right);
    loop {
        large_rec.top_right += 1;
        if large_rec.top_right < 10 {
            println!("{}", large_rec.top_right);
            large_rec.down_left += 1;
            // Skip the rest of this iteration
        } else {
            println!(
                "Large Rectangle top right position: {} ",
                large_rec.top_right
            );
            println!(
                "Large Rectangle down left position: {} ",
                large_rec.down_left
            );
            break ();
        }
    }
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
    fn new_rec(first: i32, secend: i32) -> Rectangle {
        Rectangle {
            top_right: first,
            down_left: secend,
        }
    }
    let rec1 = new_rec(12, 13);
    println!("{}", rec1.top_right);
}
