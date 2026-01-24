# Pyth Network Crypto Price Fetcher in Rust

A Rust application to fetch real-time cryptocurrency prices using the Pyth Network Hermes API.

## Features

- ✅ Fetch real-time crypto prices
- ✅ List all available crypto price feeds
- ✅ Search for specific cryptocurrencies
- ✅ Support for 500+ crypto assets
- ✅ Easy to integrate and extend

## Prerequisites

- Rust (1.70 or higher)
- Cargo

## Installation

1. Create a new Rust project:
```bash
cargo new price-checker
cd price-checker
```

2. Add dependencies to `Cargo.toml`:
```toml
[package]
name = "price-checker"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
```

3. Copy the code from the artifact to `src/main.rs`

4. Run the application:
```bash
cargo run
```

## Usage

### 1. List All Crypto Price Feeds

```rust
list_crypto_feeds().await?;
```

This will display the first 50 available crypto price feeds with their symbols, descriptions, and IDs.

### 2. Search for Specific Crypto

```rust
search_price_feed("BTC").await?;
search_price_feed("ETH").await?;
search_price_feed("DOGE").await?;
```

Search by symbol or asset name to find the price feed ID you need.

### 3. Get Current Price

```rust
let btc_feed_id = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43";
get_crypto_price(btc_feed_id).await?;
```

## Common Price Feed IDs

| Asset | Symbol | Price Feed ID |
|-------|--------|---------------|
| Bitcoin | BTC/USD | `0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43` |
| Ethereum | ETH/USD | `0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace` |
| Solana | SOL/USD | `0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d` |
| Cardano | ADA/USD | `0x2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d` |
| Dogecoin | DOGE/USD | `0xdcef50dd0a4cd2dcc17e45df1676dcb336a11a61c69df7a0299b0150c672d25c` |

## API Endpoints

### List All Price Feeds
```
GET https://hermes.pyth.network/v2/price_feeds
```

### Get Latest Price
```
GET https://hermes.pyth.network/api/latest_price_feeds?ids[]={PRICE_FEED_ID}
```

## Response Format

Price data includes:
- **price**: Raw price value (as string)
- **expo**: Exponent to apply to price
- **conf**: Confidence interval
- **publish_time**: Unix timestamp of price update

**Actual Price Calculation:**
```
actual_price = price × 10^expo
```

Example:
- price = "9876543210"
- expo = -8
- actual_price = 9876543210 × 10^(-8) = 98.76543210

## Example Output

```
====================================
LISTING CRYPTO PRICE FEEDS
====================================

Total price feeds available: 537

=== CRYPTO PRICE FEEDS (422) ===

Symbol: BTC/USD
Base: BTC
Quote: USD
Description: Bitcoin vs US Dollar
ID: 0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43
---

====================================
GETTING SPECIFIC PRICES
====================================

=== Bitcoin Price ===
Asset ID: 0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43
Price: $98765.43
Confidence: 12345678
Publish Time: 1706112000
```

## Advanced Usage

### Filter by Asset Type

```rust
let feeds = list_all_price_feeds().await?;
let equity_feeds: Vec<_> = feeds.iter()
    .filter(|f| f.attributes.asset_type == "Equity")
    .collect();
```

Available asset types:
- `Crypto`
- `Equity`
- `FX` (Foreign Exchange)
- `Metal`
- `Commodity`

### Multiple Price Feeds

```rust
let feed_ids = vec![
    "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43", // BTC
    "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace", // ETH
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d", // SOL
];

for feed_id in feed_ids {
    match get_crypto_price(feed_id).await {
        Ok(price) => println!("Price: ${:.2}", price),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Error Handling

The code includes proper error handling. Common errors:
- Network connectivity issues
- Invalid price feed ID
- API rate limiting
- JSON parsing errors

All functions return `Result<T, Box<dyn Error>>` for easy error propagation.

## Resources

- [Pyth Network Documentation](https://docs.pyth.network/)
- [Price Feed IDs](https://pyth.network/developers/price-feed-ids)
- [Hermes API Documentation](https://hermes.pyth.network/docs)
- [Pyth Network Website](https://pyth.network/)

## License

This is example code for educational purposes. Check Pyth Network's terms of service for production use.

## Contributing

Feel free to submit issues, fork the repository, and create pull requests for any improvements.

## Support

For issues related to Pyth Network API, visit their [Discord](https://discord.gg/pythnetwork) or [documentation](https://docs.pyth.network/).