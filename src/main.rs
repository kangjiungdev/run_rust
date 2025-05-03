use std::io::{Write, stdin, stdout};

const ONE_OR_TWO_ERROR: &str = "에러: 1 또는 2를 입력해 주세요";

fn main() {
    let method = choose_one_or_two();
    let (converted_temperature, temperature_unit, success) = convert_temperature(method);
    if success {
        println!("결과: {temperature_unit} {converted_temperature}도");
    }
}

fn convert_temperature(method: u8) -> (f64, String, bool) {
    const CELSIUS_RANGE: &str = "−273 ~ (5.5 × 10¹²)";
    const FAHRENHEIT_RANGE: &str = "−459 ~ 10¹³";

    let temperature_unit: String;
    let message: String;
    if method == 1 {
        temperature_unit = str_type_to_string_type("섭씨");
        message =
            format!("올바른 {temperature_unit} 온도(정수)를 입력해 주세요(범위: {CELSIUS_RANGE})");
    } else {
        temperature_unit = str_type_to_string_type("화씨");
        message = format!(
            "올바른 {temperature_unit} 온도(정수)를 입력해 주세요(범위: {FAHRENHEIT_RANGE})"
        );
    };

    loop {
        let mut temperature = String::new();

        print!("{message}: ");

        stdout().flush().unwrap();
        let success = stdin().read_line(&mut temperature);
        let (ok, _) = is_ok(
            success,
            str_type_to_string_type("Error: couldn't get input value"),
        );
        if !ok {
            break (0.00, str_type_to_string_type(""), false);
        }

        let success = temperature.trim().parse::<i64>();
        let err_message = format!("에러: {message}");

        let (ok, value) = is_ok(success, err_message);
        if !ok {
            continue;
        }

        let value = value.unwrap(); // 유저가 입력한 온도

        if method == 1 {
            // 섭씨 -> 화씨 변환
            if !(-273..=5500000000000).contains(&value) {
                // 섭씨 범위 초과
                println!("{}", message);
                continue;
            }

            let result = value as f64 * (9.00 / 5.00) + 32.00;
            break (result, str_type_to_string_type("화씨"), true);
        } else {
            // 화씨 -> 섭씨 변환
            if !(-459..=10000000000000).contains(&value) {
                // 화씨 범위 초과
                println!("{}", message);
                continue;
            }

            let result = 5.00 / 9.00 * (value as f64 - 32.00);
            break (result, str_type_to_string_type("섭씨"), true);
        }
    }
}

fn str_type_to_string_type(text: &str) -> String {
    String::from(text)
}

// Choose to convert Celsius to Fahrenheit or Fahrenheit to Celsius
fn choose_one_or_two() -> u8 {
    loop {
        let mut method = String::new();
        print!("섭씨 -> 화씨 변환은 1번\n화씨 -> 섭씨 변환은 2번을 입력하세요: ");
        stdout().flush().unwrap();
        let success = stdin().read_line(&mut method);
        let (ok, _) = is_ok(
            success,
            str_type_to_string_type("Error: couldn't get input value"),
        );
        if !ok {
            continue;
        }

        let success = method.trim().parse::<u8>();
        let (ok, value) = is_ok(success, str_type_to_string_type(ONE_OR_TWO_ERROR));
        if !ok {
            continue;
        }
        let method = value.unwrap();
        if method != 1 && method != 2 {
            println!("{ONE_OR_TWO_ERROR}\n");
            continue;
        }
        break method;
    }
}

fn is_ok<T, E>(success: Result<T, E>, err_message: String) -> (bool, Option<T>) {
    match success {
        Ok(value) => (true, Some(value)),
        Err(_) => {
            println!("{err_message}\n");
            (false, None)
        }
    }
}
