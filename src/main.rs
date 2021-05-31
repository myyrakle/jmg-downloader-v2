use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use headless_chrome::{util::Wait, Browser, LaunchOptionsBuilder};
use rand::Rng;

mod lib;
use lib::*;

fn jmg(
    link: String,
    delay: i32,
    random_delay: (i32, i32),
    open_browser: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 브라우저 생성 옵션
    let browser_option = LaunchOptionsBuilder::default()
        .headless(!open_browser)
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
        sleep(Duration::from_millis(2000));
        button.click()?;
    } else {
    }

    sleep(Duration::from_millis(5000));

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
            Wait::with_sleep(Duration::from_secs(second as u64));
            //sleep(Duration::from_secs(second as u64));
            do_next(&tab)?;
            download(&tab)?;
            break;
        } else {
            println!("@@ 다운로드 종료");
            break;
        }
    }

    Ok(())
}

fn main() {
    let open_browser = input_open_browser();
    let link = input_link();
    let delay = input_delay();
    let random_delay = input_random_delay();

    match jmg(link, delay, random_delay, open_browser) {
        Ok(()) => {
            println!("@@ 작업 성공");
        }
        Err(error) => {
            println!("@@ 실패. => {:?}", error);
        }
    }

    let _ = Command::new("pause").status();
}
