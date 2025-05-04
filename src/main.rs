use std::io::stdin;

fn main() {
    loop {
        let mut input_value = String::new();
        let success = stdin().read_line(&mut input_value);
        if success.is_err() {
            println!("에러: 입력 값 받을 수 없음");
            return;
        }

        let success = input_value.trim().parse::<usize>();
        let input_value = match success {
            Ok(value) => value,
            Err(_) => {
                println!("에러: 입력 값 파싱 불가");
                continue;
            }
        };
        let mut a = 0; // 첫번째 수
        let mut b = 0; // 두번째 수
        let mut c: u8 = 0; // a 구해야 하는지 b 구해야 하는지 판단. a 구할 땐 0, b 구할 땐 1.
        for _ in 1..=input_value {
            if a == 0 {
                // 0으로 시작.
                b = 1;
            }
            if c == 0 {
                a += b;
                c += 1;
            } else {
                b += a;
                c -= 1;
            }
        }
        let result = if input_value % 2 == 0 { a } else { b };
        println!("result: {result}");
        break;
    }
}
