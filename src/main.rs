use std::io::{self, Write};

const NEXT_LINE: &str = "\n-----------------------------------\n";
struct Lyric {
    lyric: String,
    day: String,
}

fn main() {
    let verse = loop {
        print!("몇 절까지 출력할지 1~12 사이의 숫자를 입력하세요: ");
        io::stdout().flush().unwrap();
        let mut verse_number = String::new();
        let success = io::stdin().read_line(&mut verse_number);
        if success.is_err() {
            continue;
        }
        let success = verse_number.trim().parse::<u8>();
        if let Ok(value) = success {
            if value > 0 && value <= 12 {
                break value;
            }
        }
        println!("에러: 1~12 사이의 숫자를 입력하세요\n");
    };
    print_lyrics(verse);
}

fn print_lyrics(verse: u8) {
    let lyrics: [Lyric; 12] = [
        create_lyric("A partridge in a pear tree.", "first"),
        create_lyric("Two turtle doves", "second"),
        create_lyric("Three French hens", "third"),
        create_lyric("Four calling birds", "fourth"),
        create_lyric("Five golden rings", "fifth"),
        create_lyric("Six geese a-laying", "sixth"),
        create_lyric("Seven swans a-swimming", "seventh"),
        create_lyric("Eight maids a-milking", "eighth"),
        create_lyric("Nine ladies dancing", "ninth"),
        create_lyric("Ten lords a-leaping", "tenth"),
        create_lyric("Eleven pipers piping", "eleventh"),
        create_lyric("Twelve drummers drumming", "twelfth"),
    ];

    println!("{NEXT_LINE}");

    for i in 0..verse {
        let day = &lyrics[i as usize].day;
        let first_lyric = format!("On the {day} day of Christmas\nMy true love gave to me");
        let mut a = vec![];
        for lyric in lyrics[..i as usize + 1].iter().rev() {
            a.push(lyric.lyric.clone());
        }
        if a.len() > 1 {
            let index = a.len() - 1;
            a[index] = String::from("And a partridge in a pear tree.");
        }

        println!("{first_lyric}");
        for el in a {
            println!("{el}");
        }
        println!("{NEXT_LINE}");
    }

    let mut i = 0;
    while (i as u8) < verse {
        i += 1;
    }
}

fn create_string(text: &str) -> String {
    String::from(text)
}

fn create_lyric(lyric: &str, day: &str) -> Lyric {
    Lyric {
        lyric: create_string(lyric),
        day: create_string(day),
    }
}
