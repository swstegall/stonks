extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate alphavantage;
extern crate structopt;

use alphavantage::{Client, time_series::Entry};
use dotenv::dotenv;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    stock: String,
}

fn query_stock(client: &Client, stock_name: &str) {
    let time_series = client.get_time_series_daily(stock_name).unwrap();
    let entry = time_series.entries.last().unwrap();
    print_stock_data(stock_name, entry);
}

fn print_stock_data(stock_name: &str, entry: &Entry) {
    println!();
    println!("{}", stock_name);
    println!("Date:   {:?}", entry.date);
    println!("Open:   {:?}", entry.open);
    println!("High:   {:?}", entry.high);
    println!("Low:    {:?}", entry.low);
    println!("Close:  {:?}", entry.close);
    println!("Volume: {:?}", entry.volume);
    println!();
}

fn main() {
    dotenv().ok();
    let api_token = dotenv!("API_TOKEN");
    let args = Cli::from_args();
    let client = Client::new(api_token);
    let stock_name = &args.stock;
    query_stock(&client, stock_name);
}