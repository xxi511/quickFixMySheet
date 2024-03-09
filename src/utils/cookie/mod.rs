use uuid::Uuid;
use cookie::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn make_dd_s_cookie() -> Cookie<'static> {
    let uuid = Uuid::new_v4().to_string();
    let logs = "logs=1";
    let id = format!("id={uuid}");
    let current = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't get time stamp")
        .as_millis();
    let created = format!("created={current}");
    let expire_time = current + 900000;
    let expire = format!("expire={expire_time}");
    let value = [logs, &id, &created, &expire].join("&");

    let cookie = Cookie::new("_dd_s", value);
    return cookie;
}

pub fn make_bob_app_cookie() -> Cookie<'static> {
    return Cookie::new("bob-app", "bob-app-use");
}

pub fn cookie_header_value(cookies: &Vec<Cookie<'_>>) -> String {
    let mut values = Vec::new();
    for cookie in cookies {
        let (name, value) = cookie.name_value();
        values.push(format!("{name}={value}"));
    }

    let dd_s_cookie = make_dd_s_cookie();
    let (name, value) = dd_s_cookie.name_value();
    values.push(format!("{name}={value}"));

    return values.join("; ");
}
