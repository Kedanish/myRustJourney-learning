//rust by examples: https://doc.rust-lang.org/rust-by-example/hello/print.html
// macros : https://doc.rust-lang.org/rust-by-example/macros.html
//ctr + mouse
//
//#[derive(Debug)]
//#[allow(dead_code)]
// #[allow(unused_variables)]
// #[allow(unused_imports)]

/*

Printing is handled by a series of macros defined in std::fmt some of which include:
All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.

*/
/*

std::fmt contains many traits which govern the display of text. The base form of two important ones are listed below:

    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
*/
