use std::io::{self, Write, stdin};

fn main() {
    print!("문자를 입력해 주세요: ");
    io::stdout().flush().unwrap();
    let mut text = String::new();
    let sussess = stdin().read_line(&mut text);
    if let Err(err) = sussess {
        eprintln!("Failed to read input: {}", err);
        return;
    }
    println!("{}", compress_text(text));
}

fn compress_text(text: String) -> String {
    let text: Vec<&str> = text.trim().split("").filter(|x| !x.is_empty()).collect();
    let mut previous_str = String::new();
    let mut equal_char_loop_number = 1;
    let mut result = vec![];
    for el in text {
        if el == previous_str {
            equal_char_loop_number += 1;
        } else {
            if !previous_str.is_empty() {
                result.push(CharStruct {
                    char: previous_str,
                    loop_number: equal_char_loop_number,
                });
                equal_char_loop_number = 1;
            }
            previous_str = el.to_string();
        }
    }
    result.push(CharStruct {
        char: previous_str,
        loop_number: equal_char_loop_number,
    });
    convert_text(result)
}

fn convert_text(result: Vec<CharStruct>) -> String {
    let mut converted_text = String::new();
    for el in result {
        if el.loop_number != 1 {
            converted_text = format!("{converted_text}{}{}", el.char, el.loop_number)
        } else {
            converted_text = format!("{}{}", converted_text, el.char)
        }
    }

    converted_text
}

struct CharStruct {
    char: String,
    loop_number: u32,
}
