#[derive(Debug)]
struct Rectangle{
    width : u64,
    height : u64
}

impl Rectangle{
    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_hold(&self, rectancle: &Rectangle) -> bool {
        if (self.height > rectancle.height) && (self.width > rectancle.width){
            return true
        }
        return false
    }
}

fn main() {
    let rectangle = Rectangle{
        width : 10, 
        height: 5
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of {:#?} is {}", rectangle, rectangle.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
