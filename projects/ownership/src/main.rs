fn main() {
    exp1();
    exp2();
    exp3();
    exp4();
    exp5();
    // exp6();
} 

fn exp1() {
    // 문자열 리터럴 저장은 크기가 고정. 
    // let s = "hello";
    // 그래서 String 타입을 사용하면 가변 문자열도 스택을 이용해 효율적으로 사용할 수 있음
    // string 타입의 from 메서드를 이용해서 문자열을 저장
    let s1 = String::from("hello");

    // string 메서드 -> 문자열 추가
    s1.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다

    println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다
}

fn exp2() {
    // string 타입은 포인터와 크기, 용량 등을 저장한 메타데이터이고, 데이터는 힙에 저장됨
    // 다른 언어에서 얕은 복사처럼 s2에 s1을 대입하면, 메타데이터가 복사되어 저장되고,
    // 실제 문자열 데이터를 공유한다.
    // 러스트는 얕은 복사는 일어나지 않고, 이를 s1이 s2로 이동되었다고 표현함
    let s1 = String::from("hello");
    let s2 = s1;

    // s1이 제거되면서 실제 데이터가 같이 삭제되고, s2에 오류나는 현상(중복 해제 에러)을 
    // 방지하기 위해서, 복사 후 s1은 더 이상 유효한 값을 가지고 있지 않다.
    try {
        println!("{}, world!", s1);
    } catch {
        println!("this is unsafe pointer")
    }

    //깊은 복사를 위해선, string의 clone 메서드를 활용해야 한다.
    let s1 = s2::clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn exp3() {
    // 정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장되기 때문입니다. 
    // 스택에 저장되니, 복사본을 빠르게 만들 수 있고, 
    // 따라서 굳이 y를 생성하고 나면 x를 무효화할 필요가 없습니다.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // 그냥 copy 가능한 타입의 일부 목록
    // 모든 정수형 타입 (예: u32)
    // true, false 값을 갖는 논리 자료형 bool
    // 모든 부동 소수점 타입 (예: f64)
    // 문자 타입 char
    // Copy 가능한 타입만으로 구성된 튜플 (예를 들어, (i32, i32)는 Copy 가능하지만 (i32, String)은 불가능합니다)
}

///////////////////////////////////////////////////////////////////////////
// 아래 함수 모음은 함수에 값을 전달하는 메커니즘이 어떻게 동작하는지 설명합니다.
fn exp4() {
    let s = String::from("hello");  // s가 스코프 안으로 들어옵니다

    takes_ownership(s);             // s의 값이 함수로 이동됩니다...
                                    // ... 따라서 여기서는 더 이상 유효하지 않습니다

    let x = 5;                      // x가 스코프 안으로 들어옵니다

    makes_copy(x);                  // x가 함수로 이동될 것입니다만,
                                    // i32는 Copy이므로 앞으로 계속 x를
                                    // 사용해도 좋습니다

} // 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 됩니다. 그러나 s의 값이 이동되었으므로
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
  // 메모리가 해제됩니다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.



fn exp4() {
    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
                                        // 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데,
                                        // 이 함수 또한 자신의 반환 값을 s3로
                                        // 이동시킵니다
} // 여기서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도
  // 일어나지 않습니다. s1은 스코프 밖으로 벗어나고 버려집니다.

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}
///////////////////////////////////////////////////////////////////////////////////

/// 위와 같은 소유권 상실을 방지하기 위해서, String 값의 참조자를 만들 수 있습니다. 
/// 참조자 (reference) 는 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주솟값에 해당하는,
/// 포인터와 같은 것입니다; 그 데이터는 다른 어떤 변수가 소유하고 있죠. 
/// 포인터와는 달리, 참조자는 살아있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장해 줍니다.

fn exp5() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 이처럼 참조자를 만드는 행위를 대여 (borrow) 라고 합니다. 
/// 현실에서도 여러분이 다른 사람이 소유하고 있는 뭔가를 빌리고, 
/// 용무가 끝나면 돌려주는 것처럼요. 여러분의 소유가 아니니까요.
/// 빌린 값을 수정하면 어떻게 될까요?
/// 이 코드는 작동하지 않습니다.
/// 가변 참조자 (mutable reference) 를 사용하는 식으로 살짝만 수정하면 에러를 없앨 수 있습니다
/// 가변 참조자는 한 가지 큰 제약사항이 있습니다: 
/// 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없습니다

fn exp6() {
    // let s = String::from("hello");
    let mut s = String::from("hello");

    change(&s);
}
// fn change(some_string: String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


/// 슬라이스 (slice) 는 컬렉션 (collection) 을 통째로 참조하는 것이 아닌, 
/// 컬렉션의 연속된 일련의 요소를 참조하도록 해줍니다. 
/// 슬라이스는 참조자의 일종으로서 소유권을 갖지 않습니다.

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 특정 대상의 불변 참조자가 이미 존재할 경우에는 가변 참조자를 만들 수 없다는 규칙이 있었죠. 
/// clear 함수는 String의 길이를 변경해야 하니 가변 참조자가 필요합니다.
/// clear 호출 이후 println!는 word의 참조자를 사용하므로, 
/// 이 불변 참조자는 이 지점까지 계속 활성화되어 있어야 합니다.
/// 러스트는 clear 내의 가변 참조자와 word의 불변 참조자가 같은 시점에 
/// 존재하는 것을 허용하지 않으므로 컴파일 에러가 발생합니다.
fn err_func() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 에러!

    println!("the first word is: {}", word);
}
