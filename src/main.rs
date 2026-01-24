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

#[derive(Debug, Deserialize, Serialize)]
struct PriceFeedMetadata {
    id: String,
    attributes: FeedAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
struct FeedAttributes {
    symbol: String,
    asset_type: String,
    base: String,
    quote_currency: String,
    description: String,
}



async fn list_all_price_feeds() -> Result<Vec<PriceFeedMetadata>, Box<dyn Error>> {
    let url = "https://hermes.pyth.network/v2/price_feeds";
    
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    let feeds: Vec<PriceFeedMetadata> = response.json().await?;
    
    println!("Total price feeds available: {}\n", feeds.len());
    
    Ok(feeds)
}

async fn list_crypto_feeds() -> Result<(), Box<dyn Error>> {
    let all_feeds = list_all_price_feeds().await?;
    
    // Filter for crypto assets
    let crypto_feeds: Vec<&PriceFeedMetadata> = all_feeds
        .iter()
        .filter(|f| f.attributes.asset_type == "Crypto")
        .collect();
    
    println!("=== CRYPTO PRICE FEEDS ({}) ===\n", crypto_feeds.len());
    
    for feed in crypto_feeds.iter().take(50) {  // Show first 50
        println!("Symbol: {}", feed.attributes.symbol);
        println!("Base: {}", feed.attributes.base);
        println!("Quote: {}", feed.attributes.quote_currency);
        println!("Description: {}", feed.attributes.description);
        println!("ID: {}", feed.id);
        println!("---");
    }
    
    Ok(())
}

async fn search_price_feed(symbol: &str) -> Result<(), Box<dyn Error>> {
    let all_feeds = list_all_price_feeds().await?;
    
    let matches: Vec<&PriceFeedMetadata> = all_feeds
        .iter()
        .filter(|f| {
            f.attributes.symbol.to_lowercase().contains(&symbol.to_lowercase()) ||
            f.attributes.base.to_lowercase().contains(&symbol.to_lowercase())
        })
        .collect();
    
    if matches.is_empty() {
        println!("No price feeds found for: {}", symbol);
    } else {
        println!("=== Found {} matches for '{}' ===\n", matches.len(), symbol);
        for feed in matches {
            println!("Symbol: {}", feed.attributes.symbol);
            println!("Type: {}", feed.attributes.asset_type);
            println!("Description: {}", feed.attributes.description);
            println!("ID: {}", feed.id);
            println!("---");
        }
    }
    
    Ok(())
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
    // Example 1: List all crypto price feeds (first 50)
    println!("====================================");
    println!("LISTING CRYPTO PRICE FEEDS");
    println!("====================================\n");
    list_crypto_feeds().await?;
    
    // Example 2: Search for specific crypto
    println!("\n====================================");
    println!("SEARCHING FOR 'DOGE'");
    println!("====================================\n");
    search_price_feed("DOGE").await?;
    
    // Example 3: Get specific prices
    println!("\n====================================");
    println!("GETTING SPECIFIC PRICES");
    println!("====================================\n");
    
    let btc_feed = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";
    let eth_feed = "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace";
    let sol_feed = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    
    println!("=== Bitcoin Price ===");
    get_crypto_price(btc_feed).await?;
    
    println!("\n=== Ethereum Price ===");
    get_crypto_price(eth_feed).await?;
    
    println!("\n=== Solana Price ===");
    get_crypto_price(sol_feed).await?;
    
    Ok(())
}