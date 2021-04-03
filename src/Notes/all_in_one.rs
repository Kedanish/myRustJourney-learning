//===========================================================================================================//
Comments:

// one line comment

/*
multiple
lines
of
Comments
*/


//===========================================================================================================//

//rust by examples: https://doc.rust-lang.org/rust-by-example/hello/print.html
// macros : https://doc.rust-lang.org/rust-by-example/macros.html

Printing is handled by a series of macros defined in std::fmt some of which include:
All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as format! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint!but a newline is appended.

std::fmt contains many traits which govern the display of text. The base form of two important ones are listed below:

    fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

//===========================================================================================================//
//source : https://doc.rust-lang.org/rust-by-example/primitives.html

Primitives:
    Rust provides access to a wide variety of primitives. A sample includes:

    Scalar Types:
        signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
        unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
        floating point: f32, f64
        char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
        bool either true or false
        and the unit type (), whose only possible value is an empty tuple: ()
            (Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.)

    Compound Types:
        arrays like [1, 2, 3]
        tuples like (1, true)

    Variables can always be type annotated.
    Numbers may additionally be annotated via a suffix or by default.
    Integers default to i32 and floats to f64.
    Note that Rust can also infer types from context.

//===========================================================================================================//

//source : https://doc.rust-lang.org/rust-by-example/primitives/literals.html

Literals and operators :
    Integers 1,
    floats 1.2,
    characters 'a',
    strings "abc",
    booleans true/false and
    the unit type ()
    can be expressed using literals.
    //
    Integers can, alternatively,be expressed using
    hexadecimal, octal or binary notation
    using these prefixes respectively: 0x, 0o or 0b.
    //
    Underscores can be inserted in numeric literals to improve readability,
    e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.
    //
    We need to tell the compiler the type of the literals we use.
    

    The operators available and their precedence in Rust are similar to other C-like languages.

//===========================================================================================================//
//Source : https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

Tuples :
    A tuple is a collection of values of different types.
    Tuples are constructed using parentheses (),
    and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members.
    Functions can use tuples to return multiple values, as tuples can hold any number of values.

//===========================================================================================================//
//source : https://doc.rust-lang.org/rust-by-example/primitives/array.html#arrays-and-slices

Arrays and Slices:
    Arrays:
        An array is a collection of objects of the same type T, stored in contiguous memory.
        Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; length].
    Slices:
        Slices are similar to arrays, but their length is not known at compile time.
        Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice.
        The word size is the same as usize, determined by the processor architecture eg 64 bits on an x86-64.
        Slices can be used to borrow a section of an array, and have the type signature &[T].

//===========================================================================================================//
// source : https://doc.rust-lang.org/rust-by-example/custom_types.html
// source : https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
// source : https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
Custom Types
    Rust custom data types are formed mainly through the two keywords:

        struct: define a structure
        enum: define an enumeration

    Constants can also be created via the const and static keywords.


Structures:
    There are three types of structures ("structs") that can be created using the struct keyword:

        Tuple structs, which are, basically, named tuples.
        The classic C structs
        Unit structs, which are field-less, are useful for generics.

Enums:
        The enum keyword allows the creation of a type which may be one of a few different variants.
         Any variant which is valid as a struct is also valid as an enum.

//===========================================================================================================//
//Source : https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
constants :
    Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

        const: An unchangeable value (the common case).

        static: A possibly mutable variable with 'static lifetime.
                The static lifetime is inferred and does not have to be specified.
                Accessing or modifying a mutable static variable is unsafe.

//===========================================================================================================//
// S : https://doc.rust-lang.org/rust-by-example/variable_bindings.html     

Variable Bindings:
    Rust provides type safety via static typing.

    Variable bindings can be type annotated when declared.
    (However, in most cases,
    the compiler will be able to infer the type of the variable from the context,
    heavily reducing the annotation burden.)

    Values (like literals) can be bound to variables, using the let binding.


    The compiler warns about unused variable bindings; these warnings can
    be silenced by prefixing the variable name with an underscore
//===========================================================================================================//
// S : https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
    Mutability:
        Variable bindings are immutable by default, but this can be overridden using the mut modifier.
//===========================================================================================================//]
    Scope and Shadowing:
// S :https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//
//===========================================================================================================//