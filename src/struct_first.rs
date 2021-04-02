#[derive(Debug)]
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
#[allow(dead_code)] //this thing make so much noise & i hate it
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
    fn can_hold(&self, rect: &Rectangle) {
        if self.height > rect.height && self.width > rect.width {
            println!("Rec2 can fit in rect3");
        } else {
            println!("Rec2 can't fit in rect3");
        }
    }
    fn can_hold_in_it(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
#[derive(Debug)]
struct Cord(i32, i32);
impl Cord {
    fn xy(&self) -> i32 {
        &self.0 * &self.1
    }
    fn equal(cord: i32) -> Cord {
        Cord(cord, cord)
    }
}
//
#[allow(unused_variables)]
pub fn run() {
    let cord = Cord(1, 3);
    println!("Cord x*y={}", cord.xy());
    let cord1 = Cord::equal(12);
    println!("Our Equal Cord :{:?}", cord1);
    // println!("{}", cord.1);
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
    rec3.can_hold(&rec2);
    if rec3.can_hold_in_it(&rec2) == true {
        println!("Rec2 can fit in rect3");
    } else {
        println!("Rec2 can't fit in rect3");
    }
    //
    let square = Rectangle::square(12);
    println!("{:?}", square.height);
}
