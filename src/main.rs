use std::io;

fn main() {
    println!("Hello, world!");
    guessing_game()
}

fn guessing_game() {
    println!("숫자를 맞혀봅시다!");
    println!("정답이라고 생각하는 숫자를 입력하세요.");
    // 변수생성 - let, 가변 - mut, 표준 문자열 라이브러리 - String
    let mut guess = String::new();

    // 입출력 표준 라이브러리 io
    // use 사용없이 std::io::stdin() 가능
    io::stdin()
        // & 기호는 이 인수가 참조(reference) 타입이라는 점을 지시함
        // 참조도 변수와 마찬가지로 기본적으로는 변경할 수 없기 때문에 변경이 가능한 참조를 전달하기 위해
        // &mut guess로 포기하였습니다.
        // read_line은 io::Result 타입을 반환합니다. (enum으로 Ok와 Err를 가지고 있습니다.)
        .read_line(&mut guess)
        // expect는 Ok일 경우 값을 반환 Error일 경우 메시지를 반환합니다.
        // expect 메서드를 호출하지 않는다면 프로그램은 컴파일 되지만 경고 메시지가 나옵니다.
        .expect("입력한 값을 읽지 못했습니다.");

    // 자리 지정자를 이용해 값을 출력할 수 있습니다.
    print!("입력한 값: {}", guess);
}

#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        assert_eq!(1 + 1, 2);
    }
}
