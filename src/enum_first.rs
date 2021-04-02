// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
#[derive(Debug)]
#[allow(dead_code)]
enum BookGenre {
    Action(String),
    Drama(String),
    Romance(String),
    Comedy(String),
}
#[derive(Debug)]

enum UserInputIfYouWant {
    Num(i32),
    Add,
    Subtract,
}
type Opperate = UserInputIfYouWant;
fn genre_shower(book: BookGenre) {
    match book {
        BookGenre::Action(string) => println!("{:?}", string),
        BookGenre::Drama(string) => println!("{:?}", string),
        BookGenre::Romance(string) => println!("{:?}", string),
        BookGenre::Comedy(string) => println!("{:?}", string),
    }
}
pub fn run() {
    let book1 = BookGenre::Action(String::from("hello"));
    genre_shower(book1);
    let ifplus = Opperate::Num(32);
    println!("{}", ifplus);
}
