use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self};
use std::path::Path;

use headless_chrome::{Browser, Element};
use url::Url;

fn get_content(element: Vec<Element>) -> Option<String> {
    let texts: Option<Vec<String>> = element
        .into_iter()
        .map(|element| {
            element
                .call_js_fn("function() { return this.textContent;}", false)
                .ok()
                .map(|e| e.value)
                .flatten()
                .map(|e| e.as_str().unwrap_or("").to_string())
        })
        .collect();

    Some(texts?.join("\n").to_string())
}

fn file_write(filename: &str, content: String) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&filename).exists() == false {
        File::create(&filename)?;
    }

    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

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
            break buffer.trim().into();
        }
    }
}

fn input_delay() -> i32 {
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

fn input_random_delay() -> (i32, i32) {
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

fn jmg(
    link: String,
    _delay: i32,
    _random_delay: (i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    //tab.navigate_to(&link)?;
    tab.navigate_to(
        "https://docs.rs/headless_chrome/0.9.0/headless_chrome/browser/tab/struct.Tab.html",
    )?;
    tab.wait_for_element(".pc-guide-action > a")?.click()?;

    // driver.findElementByCssSelector("").click()

    let elements = tab.wait_for_elements("p")?;
    let content = get_content(elements).unwrap();
    let title = "foo";
    let part = 1;

    file_write("foo", content)?;

    // val ps = driver.findElementsByTagName("p")
    // for(e in ps) {
    //     text.append(e.text)
    //     text.append("\n");
    // }

    // var title = ""

    // driver.findElementByCssSelector(".viewScroll-Center").click();
    // if(title=="") {
    //     title = driver.findElementByCssSelector(".view-title > p").text

    //     title = arrayOf("\\", "/", ":", "*", "?", "\"", "<", ">", "|").fold(title) {
    //         title, character -> title.replace(character, " ")
    //     }

    //     println("제목은 [${title}]이요")
    // }

    Ok(())
}

fn main() {
    let link = input_link();
    let delay = input_delay();
    let random_delay = input_random_delay();

    match jmg(link, delay, random_delay) {
        Ok(()) => {
            println!("@@ 작업 성공");
        }
        Err(error) => {
            println!("@@ 실패, {:?}", error);
        }
    }
}
