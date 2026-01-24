use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

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



async fn get_crypto_price(price_feed_id: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://hermes.pyth.network/api/latest_price_feeds?ids[]={}",
        price_feed_id
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await?;
    
    let pyth_data: Vec<PythPrice> = response.json().await?;
    
    if let Some(price_data) = pyth_data.first() {
        let price: f64 = price_data.price.price.parse()?;
        let expo: i32 = price_data.price.expo;
        let actual_price = price * 10_f64.powi(expo);
        
        println!("Asset ID: {}", price_data.id);
        println!("Price: ${:.2}", actual_price);
        println!("Confidence: {}", price_data.price.conf);
        println!("Publish Time: {}", price_data.price.publish_time);
        
        Ok(actual_price)
    } else {
        Err("No price data available".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Price feed IDs for various crypto assets
    // BTC/USD
    let btc_feed = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";
    // ETH/USD
    let eth_feed = "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace";
    // SOL/USD
    let sol_feed = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    
    println!("=== Bitcoin Price ===");
    get_crypto_price(btc_feed).await?;
    
    println!("\n=== Ethereum Price ===");
    get_crypto_price(eth_feed).await?;
    
    println!("\n=== Solana Price ===");
    get_crypto_price(sol_feed).await?;
    
    Ok(())
}