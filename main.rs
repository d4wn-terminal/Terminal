mod blockchain;
mod twitter;
mod github;
mod database;
mod notifications;

use tokio;
use database::init_db;
use notifications::send_alert;

#[tokio::main]
async fn main() {
    println!("D4WN AI is starting...");

    // Инициализация базы данных
    init_db().await.unwrap();

    // Запуск мониторинга блокчейнов
    tokio::spawn(async {
        blockchain::monitor_ethereum().await;
        blockchain::monitor_solana().await;
    });

    // Запуск мониторинга Twitter и GitHub
    tokio::spawn(async {
        twitter::monitor_twitter().await;
        github::monitor_github().await;
    });

    // Ожидание событий
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
