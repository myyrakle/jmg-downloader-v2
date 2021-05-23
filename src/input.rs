use std::io::{self};

use url::Url;

pub fn input_link() -> String {
    let mut buffer = String::new();

    loop {
        println!("## 링크: ");
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("!! 입력 오류입니다. 다시 입력해주세요.");
            continue;
        } else if Url::parse(&buffer).is_err() {
            println!("!! 잘못된 입력 형식입니다. 다시 입력해주세요.");
            continue;
        } else {
            break buffer.trim().into();
        }
    }
}

pub fn input_delay() -> i32 {
    let mut buffer = String::new();
    loop {
        println!("## 반복 딜레이(초 단위): ");
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("!! 입력 오류입니다. 다시 입력해주세요.");
            continue;
        } else if let Ok(num) = buffer.trim().parse::<i32>() {
            break num;
        } else {
            println!("!! 잘못된 입력 형식입니다. 다시 입력해주세요.");
            continue;
        }
    }
}

pub fn input_random_delay() -> (i32, i32) {
    let mut buffer = String::new();

    loop {
        println!("## 추가 랜덤 딜레이(초 단위). 입력 예시 5 10 : ");
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("!! 입력 오류입니다. 다시 입력해주세요.");
            continue;
        }

        let mut parts = buffer.trim().split_whitespace().map(|s| s.parse::<i32>());
        match (parts.next(), parts.next()) {
            (Some(Ok(l)), Some(Ok(r))) => {
                break (l, r);
            }
            _ => {
                println!("!! 잘못된 입력 형식입니다. 다시 입력해주세요.");
                continue;
            }
        }
    }
}
