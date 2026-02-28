use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Serialize)]
struct PythPrice {
    id: String,
    price: Price,
    ema_price: Price,
}

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    price: String,
    conf: String,
    expo: i32,
    publish_time: i64,
}

async fn get_price(client: &reqwest::Client, label: &str, price_feed_id: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://hermes.pyth.network/api/latest_price_feeds?ids[]={}",
        price_feed_id
    );

    let response = client.get(&url).send().await?;
    let pyth_data: Vec<PythPrice> = response.json().await?;

    if let Some(price_data) = pyth_data.first() {
        let price: f64 = price_data.price.price.parse()?;
        let expo: i32 = price_data.price.expo;
        let actual_price = price * 10_f64.powi(expo);
        println!("{:<12}: ${:.2}", label, actual_price);
        Ok(actual_price)
    } else {
        Err("No price data available".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Crypto
    let btc_feed  = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";
    let eth_feed  = "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace";
    let sol_feed  = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

    // Commodities
    let gold_feed      = "0x765d2ba906dbc32ca17cc11f5310a89e9ee1f6420508c63861f2f8ba4ee34bb2";
    let silver_feed    = "0xf2fb02c32b055c805e7238d628e5e9dadef274376114eb1f012337cabe93871e";
    let crude_oil_feed = "0x2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f";

    let client = reqwest::Client::new();

    println!("Fetching prices every 1 second... Press Ctrl+C to stop.\n");

    loop {
        let now = chrono::Local::now();
        println!("--- {} ---", now.format("%H:%M:%S"));

        // Crypto
        if let Err(e) = get_price(&client, "BTC", btc_feed).await {
            eprintln!("BTC error: {}", e);
        }
        if let Err(e) = get_price(&client, "ETH", eth_feed).await {
            eprintln!("ETH error: {}", e);
        }
        if let Err(e) = get_price(&client, "SOL", sol_feed).await {
            eprintln!("SOL error: {}", e);
        }

        // Commodities
        if let Err(e) = get_price(&client, "Gold (XAU)", gold_feed).await {
            eprintln!("Gold error: {}", e);
        }
        if let Err(e) = get_price(&client, "Silver (XAG)", silver_feed).await {
            eprintln!("Silver error: {}", e);
        }
        if let Err(e) = get_price(&client, "Crude Oil", crude_oil_feed).await {
            eprintln!("Crude Oil error: {}", e);
        }

        println!();
        sleep(Duration::from_secs(1)).await;
    }
}