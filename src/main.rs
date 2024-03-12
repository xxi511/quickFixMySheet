use config::Config;
use std::collections::HashMap;
use tokio;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Read config");
    let settings = read_config();

    println!("Login");
    let login_cookie = utils::api::get_cookie_from_home(&settings.get("user_agent").unwrap()).await;
    let mut cookies = utils::api::login(&settings, &login_cookie).await;
    cookies.push(utils::cookie::make_bob_app_cookie());
    let user_id =
        utils::api::get_user_id(&settings, utils::cookie::cookie_header_value(&cookies)).await;
    
    println!("Get timesheet");
    let timesheet_id = utils::api::get_timesheet_id(
        &settings,
        utils::cookie::cookie_header_value(&cookies),
        &user_id,
    )
    .await;
    let dates = utils::api::get_date_from_attendance(
        &settings,
        utils::cookie::cookie_header_value(&cookies),
        &timesheet_id,
    )
    .await;
    println!("Find working dates: {:?}", dates);

    let failed_dates = utils::api::modify_entries(
        &settings,
        utils::cookie::cookie_header_value(&cookies),
        &user_id,
        &dates,
    )
    .await;

    println!("Modified finish, please submit it after reviewing on the web");
    if !failed_dates.is_empty() {
        println!("The following dates are modified failed {:?}", failed_dates)
    }
    Ok(())
}

fn read_config() -> HashMap<String, String> {
    let settings = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    return settings;
}
