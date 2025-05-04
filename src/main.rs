use std::io::{Write, stdin, stdout};

fn main() {
    loop {
        let mut input_value = String::new();
        print!("피보나치 수열: ");
        stdout().flush().unwrap();
        let success = stdin().read_line(&mut input_value);
        if success.is_err() {
            println!("에러: 입력 값 받을 수 없음");
            return;
        }

        let success = input_value.trim().parse::<usize>();
        let input_value = match success {
            Ok(value) => value,
            Err(_) => {
                println!("에러: 입력 값 파싱 불가\n");
                continue;
            }
        };
        let result = a(input_value);
        println!("result: {result}");
        break;
    }
}

fn a(number: usize) -> usize {
    if number == 0 || number == 1 {
        return number;
    }
    a(number - 1) + a(number - 2)
}
