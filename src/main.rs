use headless_chrome::Browser;
use std::io::{self};
use url::Url;

fn input_link() -> String {
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
            break;
        }
    }

    buffer.trim().into()
}

//#[allow(unused_parens)]
fn input_delay() -> i32 {
    let mut buffer = String::new();
    let mut delay = 0;

    loop {
        println!("## 반복 딜레이(초 단위): ");
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("!! 입력 오류입니다. 다시 입력해주세요.");
            continue;
        } else if let Ok(num) = buffer.trim().parse::<i32>() {
            delay = num;
            break;
        } else {
            println!("!! 잘못된 입력 형식입니다. 다시 입력해주세요.");
            continue;
        }
    }

    delay
}

fn input_random_delay() -> (i32, i32) {
    let mut buffer = String::new();
    let mut random_delay = (0, 0);

    loop {
        println!("## 추가 랜덤 딜레이(초 단위). 입력 예시 5 10 : ");
        if io::stdin().read_line(&mut buffer).is_err() {
            println!("!! 입력 오류입니다. 다시 입력해주세요.");
            continue;
        }

        let mut parts = buffer.trim().split_whitespace().map(|s| s.parse::<i32>());
        match (parts.next(), parts.next()) {
            (Some(Ok(l)), Some(Ok(r))) => {
                random_delay = (l, r);
                break;
            }
            _ => {
                println!("!! 잘못된 입력 형식입니다. 다시 입력해주세요.");
                continue;
            }
        }
    }

    random_delay
}

fn jmg(
    link: String,
    delay: i32,
    random_delay: (i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    Ok(())
}

fn main() {
    let link = input_link();
    let delay = input_delay();
    let random_delay = input_random_delay();

    println!("{}, {}, {:?}", link, delay, random_delay);

    jmg(link, delay, random_delay).ok();
}
