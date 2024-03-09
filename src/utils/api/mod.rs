use chrono::format::strftime::StrftimeItems;
use chrono::TimeDelta;
use cookie::Cookie;
use reqwest::{header, Client};
use std::collections::HashMap;
mod models;
use super::ToNaiveTime;
use rand::prelude::*;

pub async fn get_cookie_from_home(user_agent: &String) -> Cookie {
    let uri = "https://app.hibob.com/home";

    let client = Client::new();
    let response = client
        .get(uri)
        .header(header::USER_AGENT, user_agent)
        .header(header::HOST, "app.hibob.com")
        .send()
        .await
        .unwrap();

    if response.status() != 200 {
        panic!("Get hibob home failed, status code {}", response.status());
    }
    let set_cookie_header = response
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();

    let cookie = Cookie::parse(set_cookie_header).unwrap();
    return cookie.into_owned();
}

pub async fn login<'a>(
    config: &'a HashMap<String, String>,
    cookie: &'a Cookie<'a>,
) -> Vec<Cookie<'a>> {
    let uri = "https://app.hibob.com/api/login";
    let (cookie_name, cookie_value) = cookie.name_value();
    let client = Client::new();
    let mut json_body = config.to_owned();
    json_body.remove("user_agent");
    let response = client
        .post(uri)
        .header(header::COOKIE, format!("{cookie_name}={cookie_value}"))
        .header(header::USER_AGENT, config.get("user_agent").unwrap())
        .header(header::ORIGIN, "https://app.hibob.com")
        .json(&json_body)
        .send()
        .await
        .unwrap();

    if response.status() != 200 {
        panic!("login failed, status code: {}", response.status());
    }
    let mut cookies = Vec::new();
    for (key, value) in response.headers() {
        println!("{} = {:?}", key, value);
        if header::SET_COOKIE != key {
            continue;
        }
        let value_str = value.to_str().unwrap().to_owned();
        let login_cookie = Cookie::parse(value_str).unwrap();
        cookies.push(login_cookie);
    }
    if cookies.len() == 0 {
        panic!();
    } else {
        cookies.push(cookie.to_owned());
        return cookies;
    }
}

pub async fn get_user_id(config: &HashMap<String, String>, cookie_value: String) -> String {
    let uri = "https://app.hibob.com/api/user";

    let client = Client::new();
    let response = client
        .get(uri)
        .header(header::USER_AGENT, config.get("user_agent").unwrap())
        .header("Bob-Timezoneoffset", config.get("timezone_offset").unwrap())
        .header(header::COOKIE, cookie_value)
        .send()
        .await
        .unwrap();
    if response.status() != 200 {
        panic!("Fetch user info failed: status: {}", response.status())
    }

    let body: models::User = response.json().await.unwrap();
    return body.id;
}

pub async fn get_timesheet_id(
    config: &HashMap<String, String>,
    cookie_value: String,
    user_id: &String,
) -> i32 {
    let uri = format!("https://app.hibob.com/api/attendance/employees/{user_id}/sheets");
    let client = Client::new();

    let response = client
        .get(uri)
        .header("Bob-Timezoneoffset", config.get("timezone_offset").unwrap())
        .header(header::USER_AGENT, config.get("user_agent").unwrap())
        .header(header::COOKIE, cookie_value)
        .send()
        .await
        .unwrap();
    if response.status() != 200 {
        panic!("Get timesheet id failed, status: {}", response.status())
    }

    let month = config.get("timesheet_month").unwrap();
    let data: models::EmployeeSheets = response.json().await.unwrap();
    let target_sheet = data
        .employee_sheets
        .iter()
        .find(|&e| e.cycle_start_date.contains(month))
        .unwrap();
    return target_sheet.id;
}

pub async fn get_date_from_attendance(
    config: &HashMap<String, String>,
    cookie_value: String,
    sheet_id: &i32,
) -> Vec<String> {
    let uri = format!("https://app.hibob.com/api/attendance/my/sheets/{sheet_id}");

    let client = Client::new();
    let response = client
        .get(uri)
        .header("Bob-Timezoneoffset", config.get("timezone_offset").unwrap())
        .header(header::USER_AGENT, config.get("user_agent").unwrap())
        .header(header::COOKIE, cookie_value)
        .send()
        .await
        .unwrap();
    if response.status() != 200 {
        panic!("Get attendance failed, status: {}", response.status())
    }

    let data: models::Attendance = response.json().await.unwrap();
    let dates_needs_clock_in = data
        .attendance
        .iter()
        .filter(|&e| e.needs_clock_in())
        .map(|f| f.date.to_owned());

    return dates_needs_clock_in.collect();
}

pub async fn modify_entries(
    config: &HashMap<String, String>,
    cookie_value: String,
    user_id: &String,
    dates: &Vec<String>,
) -> Vec<String> {
    let mut failed_dates = Vec::new();
    for date in dates {
        let is_success = modify_entries_for_date(config, &cookie_value, user_id, date).await;
        if !is_success {
            failed_dates.push(date.to_owned());
        }
    }
    return failed_dates;
}

async fn modify_entries_for_date(
    config: &HashMap<String, String>,
    cookie_value: &String,
    user_id: &String,
    date: &String,
) -> bool {
    let uri = format!("https://app.hibob.com/api/attendance/employees/{user_id}/attendance/entries?forDate={date}");
    let client = Client::new();

    let mut request_body = HashMap::new();
    let times = get_clock_time(config, date);
    request_body.insert("start", times.0);
    request_body.insert("end", times.1);
    request_body.insert("offset", config.get("timezone_offset").unwrap().to_owned());

    let response = client
        .post(uri)
        .header(header::USER_AGENT, config.get("user_agent").unwrap())
        .header(header::COOKIE, cookie_value)
        .send()
        .await
        .unwrap();
    return response.status() == 200;
}

fn get_clock_time(config: &HashMap<String, String>, date: &String) -> (String, String) {
    let work_hours_in_minutes = config.get("work_hours").unwrap().parse::<i64>().unwrap() * 60;
    let mut rng = rand::thread_rng();

    let start_delta: i64 = rng.gen_range(-10..10);
    let start_time = config.get("start_time").unwrap().to_owned().to_navie_time();
    let new_start_time = start_time
        .overflowing_add_signed(TimeDelta::try_minutes(start_delta).unwrap())
        .0;

    let work_delta_in_minutes: i64 = rng.gen_range(0..15);
    let diff = TimeDelta::try_minutes(work_hours_in_minutes + work_delta_in_minutes).unwrap();
    let end_time = new_start_time.overflowing_add_signed(diff).0;

    let fmt = format!("{date}T%H:%M");
    let item = StrftimeItems::new(&fmt);
    let start_str = new_start_time.format_with_items(item.clone()).to_string();
    let end_str = end_time.format_with_items(item).to_string();
    return (start_str, end_str);
}
