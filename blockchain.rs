use web3::transports::Http;
use web3::Web3;

pub async fn monitor_ethereum() {
    let http = Http::new("https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY").unwrap();
    let web3 = Web3::new(http);

    match web3.eth().block_number().await {
        Ok(block) => println!("Ethereum latest block: {:?}", block),
        Err(e) => println!("Error fetching block: {:?}", e),
    }
}

pub async fn monitor_solana() {
    let client = reqwest::Client::new();
    let url = "https://api.mainnet-beta.solana.com";
    let body = r#"{"jsonrpc":"2.0","id":1,"method":"getRecentBlockhash"}"#;

    let res = client.post(url).body(body).send().await.unwrap();
    let json: serde_json::Value = res.json().await.unwrap();

    println!("Solana latest block: {:?}", json);
}
