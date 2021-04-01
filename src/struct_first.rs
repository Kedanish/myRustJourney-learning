#[allow(dead_code)]
//makin Rectangle Struction
struct Rectangle {
    width: u32,
    height: u32,
}

//makin fn sturct maker with same inner structure names
fn new_rec(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

//makin fn sturct maker with different inner structure names
#[allow(dead_code)] //this thing make so much noise i hate it
fn new_rec1(w: u32, h: u32) -> Rectangle {
    Rectangle {
        width: w,
        height: h,
    }
}

//
impl Rectangle {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
}
//
#[allow(unused_variables)]
pub fn run() {
    //normal
    let rec1 = Rectangle {
        width: 12,
        height: 13,
    };
    //lazy type
    let rec2 = new_rec(20, 40);
    println!("rec2.width: {}", rec2.width);
    println!("rec2.height: {}", rec2.height);

    println!("=======");

    let rec3 = new_rec1(25, 50);
    println!("rec3.width: {}", rec3.width);
    println!("rec3.height: {}", rec3.height);

    println!("=======");

    //impl
    println!("rec2.area: {}", rec2.area());
    println!("rec3.area: {}", rec3.area());
}
