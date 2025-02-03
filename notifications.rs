use reqwest::Client;

const TELEGRAM_BOT_TOKEN: &str = "YOUR_TELEGRAM_BOT_TOKEN";
const TELEGRAM_CHAT_ID: &str = "YOUR_TELEGRAM_CHAT_ID";

pub async fn send_alert(message: &str) {
    let client = Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID, message
    );

    let _ = client.get(&url).send().await.unwrap();
}
