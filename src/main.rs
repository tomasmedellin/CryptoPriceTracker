// Import necessary crates
use reqwest;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
use colored::*;
use std::thread;
use std::time::Duration;

// Define the structure of the API response
#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Data,
}

// Define the data structure for multiple cryptocurrencies
#[derive(Deserialize, Debug)]
struct Data {
    // Each cryptocurrency is a field within `Data`
    #[serde(rename = "XMR")]
    xmr: CurrencyData,
    #[serde(rename = "BTC")]
    btc: CurrencyData,
    #[serde(rename = "ETH")]
    eth: CurrencyData,
    #[serde(rename = "USDT")]
    usdt: CurrencyData,
    #[serde(rename = "LTC")]
    ltc: CurrencyData,
}

// Define the data structure for individual cryptocurrency data
#[derive(Deserialize, Debug)]
struct CurrencyData {
    // Contains the quote for the USD value
    #[serde(rename = "quote")]
    quote: Quote,
}

// Define the quote structure which contains the actual currency data
#[derive(Deserialize, Debug)]
struct Quote {
    #[serde(rename = "USD")]
    usd: Currency,
}

// Define the structure for the currency information
#[derive(Deserialize, Debug)]
struct Currency {
    price: f64,
    market_cap: f64,
    #[serde(rename = "percent_change_1h")]
    percent_change_1h: f64,
    #[serde(rename = "percent_change_24h")]
    percent_change_24h: f64,
    #[serde(rename = "percent_change_7d")]
    percent_change_7d: f64,
}

// Fetch data from the API for the specified cryptocurrencies
async fn fetch_data(api_key: &str) -> Result<Data, Box<dyn Error>> {
    // Construct the URL with the API key and the symbols for the cryptocurrencies
    let url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol=XMR,BTC,ETH,USDT,LTC&CMC_PRO_API_KEY={}", api_key);

    // Send the request to the API
    let response = reqwest::Client::new()
        .get(url)
        .header("Accepts", "application/json")
        .send()
        .await?;

    // Parse the JSON response into our ApiResponse struct
    let response_body = response.json::<ApiResponse>().await?;
    Ok(response_body.data)
}

// Clear the terminal screen
fn clear_screen() {
    // Escape sequence to clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

// Display the currency data in the terminal
fn display_currency(currency: &CurrencyData, name: &str, color: &str) {
    println!("{} Data:", name);
    println!("Price: {}", format!("${:.2}", currency.quote.usd.price).color(color));
    println!("Market Cap: {}", format!("${:.2}", currency.quote.usd.market_cap).color(color));

    // Calculate the percentage changes and color them based on positive or negative change
    let change_1h = format!("{:.2}%", currency.quote.usd.percent_change_1h);
    let change_24h = format!("{:.2}%", currency.quote.usd.percent_change_24h);
    let change_7d = format!("{:.2}%", currency.quote.usd.percent_change_7d);

    // Determine the color based on whether the change is positive or negative
    let change_1h_color = if currency.quote.usd.percent_change_1h > 0.0 { "green" } else { "red" };
    let change_24h_color = if currency.quote.usd.percent_change_24h > 0.0 { "green" } else { "red" };
    let change_7d_color = if currency.quote.usd.percent_change_7d > 0.0 { "green" } else { "red" };

    // Print the changes with the appropriate color
    println!("Change (1h): {}", change_1h.color(change_1h_color));
    println!("Change (24h): {}", change_24h.color(change_24h_color));
    println!("Change (7d): {}", change_7d.color(change_7d_color));
    println!();
}

// The main async function to run our application
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your actual API key from CoinMarketCap
    let api_key = "API KEY HERE";

    // Continuously run in a loop
    loop {
        // Clear the screen for each new set of data
        clear_screen();
        // Fetch new data
        let data = fetch_data(api_key).await?;

        // Display the data for each cryptocurrency
        display_currency(&data.xmr, "Monero (XMR)", "orange");
        display_currency(&data.btc, "Bitcoin (BTC)", "yellow");
        display_currency(&data.eth, "Ethereum (ETH)", "blue");
        display_currency(&data.usdt, "Tether (USDT)", "green");
        display_currency(&data.ltc, "Litecoin (LTC)", "magenta");

        // Sleep for 60 seconds before fetching the data again
        thread::sleep(Duration::from_secs(60));
    }
}
