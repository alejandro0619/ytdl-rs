// Author: alejandro0619
// github: github.com/alejandro0619/
// contact me: spaghetticodedev@gmail.com

use clap::Parser;

use ytdlrs_lib::api::{
    client::APIClientBuilder, downloader::DownloaderBuilder, search::SearchVideo,
};
// TODO:
// Improve the error managment

// CLI
#[derive(Debug, Parser)]
#[clap(
    author = "github.com/alejandro0619",
    version,
    about = "Simple CLI app to download videos from Youtube"
)]
struct Args {
    #[clap(short, long)]
    search: Option<bool>,
    #[clap(short, long)]
    query: String,
    #[clap(short, long)]
    format: String,
}

#[tokio::main]
async fn main() {
    let cli_args = Args::parse();

    println!(
        "Search?: {:?} \n Query: {} \n Format: {}",
        cli_args.search, cli_args.query, cli_args.format
    );
}
