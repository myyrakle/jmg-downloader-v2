use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use headless_chrome::{Browser, Element, LaunchOptionsBuilder, Tab};
use rand::Rng;
use url::Url;

mod lib;
use lib::*;

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

fn download(tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
    let query: HashMap<_, _> = Url::parse(&tab.get_url())?
        .query_pairs()
        .into_owned()
        .collect();
    let part = query.get("sortno").unwrap();

    println!("-- {} 화 다운로드 시작", part);

    let elements = tab.wait_for_elements(TEXT_SELECTOR)?;
    let content = get_content(elements).unwrap();
    let title = "foo.txt";

    file_write(title, content)?;

    println!("-- {} 화 다운로드 완료", part);

    Ok(())
}

fn has_next(tab: &Tab) -> Result<bool, Box<dyn std::error::Error>> {
    let class = tab.find_elements(NEXT_BUTTON_SELECTOR)?[1]
        .call_js_fn("function() { return this; }", false)?;

    Ok(class.description != Some("span.disabled".into()))
}

fn do_next(tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
    tab.find_elements(NEXT_BUTTON_SELECTOR)?[1].click()?;
    Ok(())
}

fn jmg(
    link: String,
    delay: i32,
    random_delay: (i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    // 브라우저 생성 옵션
    let browser_option = LaunchOptionsBuilder::default()
        .headless(true)
        .window_size(Some((1600, 1000)))
        .build()?;

    // 브라우저 생성
    let browser = Browser::new(browser_option)?;

    // 탭 생성
    let tab = browser.wait_for_initial_tab()?;

    // 최초 링크 이동
    tab.navigate_to(&link)?;

    // 최초 접근시 알림창 닫기
    if let Ok(button) = tab.wait_for_element(DONT_SEE_AGAIN_BUTTON_SELECTOR) {
        sleep(Duration::from_millis(1500));
        button.click()?;
    } else {
    }

    sleep(Duration::from_millis(3500));

    // 다음버튼 없을 경우 클릭해서 활성화
    if let Err(_error) = tab.find_element(NEXT_BUTTON_SELECTOR) {
        tab.wait_for_element(JUST_CENTOR_SELECTOR)?.click()?;
    }

    //최초 다운로드
    download(&tab)?;

    loop {
        if has_next(&tab)? {
            let mut rng = rand::thread_rng();
            let second = delay + rng.gen_range(random_delay.0..random_delay.1);
            println!("## {}초 딜레이중...", second);
            sleep(Duration::from_secs(second as u64));
            do_next(&tab)?;
            download(&tab)?;
        } else {
            println!("@@ 다운로드 종료");
            break;
        }
    }

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
            println!("@@ 실패. => {:?}", error);
        }
    }
}
