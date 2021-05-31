use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use std::path::Path;

use headless_chrome::{Element, Tab};
use url::Url;

use crate::constants::*;

// 다음 요소가 있는지 체크
pub fn has_next(tab: &Tab) -> Result<bool, Box<dyn std::error::Error>> {
    let class = tab.find_elements(NEXT_BUTTON_SELECTOR)?[1]
        .call_js_fn("function() { return this; }", false)?;

    Ok(class.description != Some("span.disabled".into()))
}

// 다음으로 이동
pub fn do_next(tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
    tab.wait_for_elements(NEXT_BUTTON_SELECTOR)?[1].click()?;
    Ok(())
}

// 요소의 텍스트 파싱
pub fn get_content(element: Vec<Element>) -> Option<String> {
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

// 파일에 텍스트 이어쓰기
pub fn file_write(filename: &str, content: String) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&filename).exists() == false {
        File::create(&filename)?;
    }

    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

// 다운로드 로직
pub fn download(tab: &Tab) -> Result<(), Box<dyn std::error::Error>> {
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
