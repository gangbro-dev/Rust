#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/// impl 뒤에 연관함수들을 적을 수 있음. 연관함수는 타입에 연관된 함수로써 .으로 호출가능함.
/// impl은 꼭 하나일 필요는 없다.
impl Rectangle {
    // 첫 매개변수로 &self를 받아옴 -> 내부 값을 읽어 이용만 함. 변화시키지 않음
    // 값을 변화시키려면 &mut self을 사용함
    // self를 직접 받고 이 인스턴스를 스코프 내에서 만료시켜 다음에 사용하지 못하도록 할 수 있음.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 필드와 메서드를 같은 이름으로 정의할 수 있음. 추후 사용에서 ()를 붙여 메서드인지 알 수 있음.
    fn width(&self) -> bool {
        self.width > 0
    }
    // 다른 인스턴스를 인수로 받을 때, other로 명시하여야 함
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // self가 없는 형태의 함수를 정의할 수 있음. 이는 메서드는 아니지만, 연관함수이긴 하다.
    // 주로 생성자로 활용됨.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
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
    let square1 = Rectangle::square(30);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );
    
    if rect1.width() { // width 메서드의 사용
        println!("The rectangle has a nonzero width; it is {}", rect1.width); // width 필드의 사용
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
/* 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/