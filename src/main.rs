use cookie::{self, Cookie};
use reqwest::{
    header::{HOST, SET_COOKIE, USER_AGENT},
    Client,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (cookie_name, cookie_value) = get_cookie_from_home().await;
    Ok(())
}

async fn get_cookie_from_home() -> (String, String) {
    let uri = "https://app.hibob.com/home";
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Safari/605.1.15";

    let client = Client::new();
    let response = client
        .get(uri)
        .header(USER_AGENT, user_agent)
        .header(HOST, "app.hibob.com")
        .send()
        .await
        .unwrap();

    if response.status() != 200 {
        panic!("Get hibob home failed, status code {}", response.status());
    }
    let set_cookie_header = response
        .headers()
        .get(SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();

    let cookie = Cookie::parse(set_cookie_header).unwrap();
    let (name, value) = cookie.name_value();

    return (name.to_string(), value.to_string());
}

fn login(account: String, password: String) {
    let uri = "https://app.hibob.com/api/login";
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Safari/605.1.15";
}