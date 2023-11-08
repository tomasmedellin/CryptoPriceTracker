# 🚀 Crypto Price Tracker 🚀

This Crypto Price Tracker is a personal project that I use daily to track the prices of my holdings. It is a lightweight and real-time terminal application that keeps you updated with the latest prices and market data for your favorite cryptocurrencies! 🎯

## What does it do? 🤔

This application fetches live data for cryptocurrencies including Monero (XMR), Bitcoin (BTC), Ethereum (ETH), Tether (USDT), and Litecoin (LTC). It presents you with the most recent prices, market capitalization, and percentage changes directly in your terminal. It's designed to be minimalistic and straightforward, making it an excellent choice for crypto enthusiasts who need quick updates without any frills. 💸

## Lightweight and Practical 🪶

Built in Rust 🦀, this application is as lightweight as it gets, ensuring that it runs smoothly even on low-specification machines. It's designed for practicality and speed, making it an ideal companion for small-scale applications or personal use. However, due to API rate limits, it's more suited for individual tracking rather than large-scale data analysis or trading.

## Credits and API Key 🗝️

This application uses the CoinMarketCap API to fetch real-time cryptocurrency data. You must obtain your own API key from [CoinMarketCap](https://pro.coinmarketcap.com/signup) to use this application.

## How to Install 🛠️

1. Make sure you have Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository to your local machine:

   \```bash
   git clone https://github.com/your-username/CryptoPriceTracker.git
   cd CryptoPriceTracker
   \```

3. Inside the `main.rs` file, replace the `api_key` placeholder with your own CoinMarketCap API key.

4. Build the project using Cargo (Rust's package manager and build system):

   \```bash
   cargo build --release
   \```

## How to Run 🏃‍♂️

After building the project, you can run it directly from the target folder:

\```bash
./target/release/monero_price_tracker
\```

Or simply use Cargo to run the project:

\```bash
cargo run --release
\```

## Structure 🏗️

- `main.rs`: This is the entry point of the application. It handles API requests, processes the JSON data, and outputs the results to the terminal.
- `Cargo.toml`: The manifest file for Rust's package manager, which includes all the dependencies and metadata for the project.

## Modifying the Tracker 📝

To add new cryptocurrencies or change the existing ones:

1. Update the `fetch_data` function to include the additional API endpoints.
2. Modify the `display_currency` function if you want to change the display format.
3. Adjust the `main` function loop to include your new functions as needed.

## License 📄

This project is released under the MIT License. See the `LICENSE` file in the repository for more details.

## That's All, Folks! 🎬

Enjoy staying on top of your crypto game! For any issues, suggestions, or contributions, feel free to open an issue or a pull request in the repository. Happy tracking! 🌟

Remember, the crypto market never sleeps, but you can always stay informed! 📈🌙

**Disclaimer**: This application is for educational purposes only. Always ensure you comply with the API's terms of service.


