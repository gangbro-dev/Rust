fn main() {
    //
    // 3.1 변수와 가변성
    //

    // 변수는 기본적으로 불변. mut 키워드를 통해서 변할 수 있음을 명시(동일 타입 내)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000; // 상수명은 대문자로 표현 _로 단위를 자를 수 있음

    // shadowing
    let x = 5;
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    
    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len();

    //
    // 3.2 데이터 타입들
    //


    // 데이터타입을 명시할 것. 타입에 따라서 메서드가 달라짐
    // 정수형 : u32(unsigned), i32(signed)
    // let guess = "42".parse().expect("Not a number!"); -> 에러!
    let guess: u32 = "42".parse().expect("Not a number!");
    

    // 실수형 : f64(기본, 64비트 부동소수점), f32
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // 기본 사칙연산들
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    // Boolean 타입
    let t = true;

    let f: bool = false; // with explicit type annotation


    // 문자 타입. 작은 따옴표로 묶기
    // 유니코드 문자를 기반으로 하므로 왠만한 모든 문자를 다룰 수 있음
    // 단, Unicode를 위한 개념은 아니므로 직관에 따른 문자와 실제 저장된 값이 다를 수 있음.(8장에서 자세히)

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    // 복합 타입. 기본적으로 튜플과 배열이 있음

    // 튜플(서로 다른 타입의 요소도 가능)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 패턴 매칭을 통해 단일 요소 분리하기
    let (x, y, z) = tup;

    println!("The value of y is: {}", y); // 6.4
    
    // index를 통해 접근하기
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;


    // 배열(모두 같은 타입의 요소)
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];

    let index = 10;

    //let element = a[index]; // 메모리 색인 에러 발생!

    // println!("The value of element is: {}", element); 
}
