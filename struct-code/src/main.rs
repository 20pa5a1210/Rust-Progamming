struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    println!("Hello, world!");
    struct User {
        acitive: bool,
        username: String,
    }
    fn build_user(username: String) -> User {
        User {
            acitive: true,
            username,
        }
    }

    let new = build_user(String::from("20PA5A1210"));

    let rect2 = Rectangle {
        width: 30,
        height: 5,
    };

    println!("{} {}", new.acitive, new.username);
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
    println!(
        "The area of the rectangle 2 is {} square pixels.",
        new_area(&rect2)
    );
}

fn area(dimensions: (u64, u64)) -> u64 {
    dimensions.0 * dimensions.1
}

fn new_area(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.height
}
