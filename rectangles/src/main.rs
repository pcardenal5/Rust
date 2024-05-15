fn area(width: u64, height: u64) -> u64 {
    width * height
}

fn main() {
    let width : u64 = 5;
    let height : u64 = 10;

    println!("The area of a rectangle of dimensions {}x{} is {}", height, width, area(height, width))
}
