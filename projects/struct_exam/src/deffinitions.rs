/// 구조체는 여러 개의 연관된 값을 가질 수 있고, 구성 요소들은 각각 다른 타입이 될 수 있습니다. 
/// 그리고 여기에 더해서, 구조체는 각각의 구성 요소에 이름을 붙일 수 있습니다. 
/// 따라서 각 요소가 더 명확한 의미를 갖게 되고, 
/// 특정 요소에 접근할 때 순서에 의존할 필요도 사라지게 되어 튜플보다 유연하게 사용할 수 있습니다.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


/// 튜플과 유사한 형태의 튜플 구조체 (tuple structs) 도 지원합니다. 
/// 튜플 구조체는 구조체 자체에는 이름을 지어 의미를 주지만 이를 구성하는 필드에는 
/// 이름을 붙이지 않고 타입만 적어 넣은 형태입니다. 튜플 구조체는 튜플 전체에 이름을 
/// 지어주거나 특정 튜플을 다른 튜플과 구분하고는 싶은데, 
/// 그렇다고 각 필드명을 일일이 정해 일반적인 구조체 형태로 만들면 너무 장황하거나 
/// 불필요할 경우 유용합니다.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/// 필드가 아예 없는 구조체를 정의할 수도 있습니다. 
/// 이는 ‘튜플 타입’에서 다룬 유닛 타입 ()와 비슷하게 동작하므로 
/// 유사 유닛 구조체 (unit-like structs) 라 지칭합니다. 
/// 유사 유닛 구조체는 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에 
/// 어떤 데이터를 저장할 필요는 없을 경우 유용합니다.
struct AlwaysEqual;

fn struct_tuple_exam() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Color 타입과 Point 타입은 둘 다 i32 값 3개로 이루어진 타입이지만,
    // Color 타입을 매개변수로 받는 함수에 Point 타입을 인수로 넘겨주는 건 불가능합니다.
    let mut some_color: Color = Color(1, 2, 3);
    let some_point = Point(4, 5, 6);

    //some_color = some_point; // error
}
fn main() {
    let user1:User = struct_exam();

    let user2:User = build_user_shorthand(
        String::from("mainuser@example.com"), 
        String::from("mainusername1234"));

    let subject = AlwaysEqual;
}

fn struct_exam() -> User {
    // 정의한 구조체를 사용하려면 해당 구조체의 각 필드에 대한 구체적인 값을 정하여 구조체의 
    // 인스턴스 (instance) 를 생성해야 합니다. 인스턴스를 생성하려면 먼저 구조체의 이름을 
    // 적고, 중괄호를 열고, 그 안에 필드의 이름 (key) 과 해당 필드에 저장할 값을 키: 값 쌍의 
    // 형태로 추가해야 합니다. 이때 필드의 순서는 구조체를 정의했을 때와 동일하지 않아도 됩니다. 
    // 바꿔 말하면, 구조체 정의는 해당 타입에 대한 일반 양식 같은 것이며, 인스턴스는 그 양식에 
    // 실제 값을 넣은 것으로 생각하시면 됩니다. 예를 들면 어떤 특정 사용자를 아래처럼 선언할 
    // 수 있습니다.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 가변 인스턴스라면 같은 방식으로 특정 필드의 값을 변경할 수도 있습니다.
    user1.email = String::from("anotheremail@example.com");

    // email에는 새로운 값을 설정했지만, 나머지 값들에는 위에서 만들었던 user1의 값과 
    // 동일합니다. 구조체 업데이트 문법을 사용하면 아래처럼 더 적은 코드로 같은 효과를
    // 낼 수 있습니다. .. 문법은 따로 명시된 필드를 제외한 나머지 필드를 주어진 인스턴스의 
    // 필드 값으로 설정합니다.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    user2
}


/// build_user 함수가 사용자 이메일과 이름을 전달받고, active, sign_in_count를 
/// 각각 true, 1로 설정한 User 인스턴스를 반환하는 모습을 보여줍니다.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/// 변수명과 구조체 필드명이 같을 땐, 필드 초기화 축약법 (field init shorthand) 을 
/// 사용해서 더 적은 타이핑으로 같은 기능을 구현할 수 있습니다.
/// email: email처럼 작성하는 대신, 변수명과 필드명이 같다는 점을 이용해 email로만 
/// 작성한 모습입니다. 물론, 함수는 이전과 같이 잘 작동합니다.
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

