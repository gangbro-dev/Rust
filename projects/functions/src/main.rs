fn main() {
    println!("Hello, world!");
    
    another_function(5, 6);

    // 블록은 {}로 묶여 있는 표현식이다. 
    // 이는 리턴값이 존재할 수 있으므로. 대입이 가능하다. 
    // 마지막 ;를 쓰지 않은 줄의 값을 대입한다 
    let y = {
        let x = 3;
        x + 1
    };
    
    let x = five();
    println!("The main value of x is: {}", x);
    println!("The main value of y is: {}", y);
}

// 함수가 정의되어 있다면, 위치는 상관하지 않고 함수를 실행함
fn another_function(x: i32, y: i32) { // 인자는 ,를 이용해 구분. 타입을 지정할 것
    println!("The another value of x is: {}", x);
    println!("The another value of y is: {}", y);
}

// 반환값을 만들 때에는 화살표로 타입을 지정할 것.
fn five() -> i32 {
    5
}
