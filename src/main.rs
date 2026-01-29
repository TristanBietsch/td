use chrono::{Local, Utc};
use clap::Parser;

/// A minimal date/time CLI tool
#[derive(Parser)]
#[command(name = "td", version, about, long_about = None)]
struct Args {
    /// Show date only
    #[arg(short = 'd', long = "date", conflicts_with = "time")]
    date: bool,

    /// Show time only
    #[arg(short = 't', long = "time", conflicts_with = "date")]
    time: bool,

    /// Use UTC instead of local time
    #[arg(short = 'u', long = "utc")]
    utc: bool,

    /// Custom strftime format
    #[arg(short = 'f', long = "format")]
    format: Option<String>,
}

fn main() {
    let args = Args::parse();

    let format = match &args.format {
        Some(f) => f.as_str(),
        None if args.date => "%Y-%m-%d",
        None if args.time => "%H:%M:%S",
        None => "%Y-%m-%dT%H:%M:%S",
    };

    let output = if args.utc {
        Utc::now().format(format)
    } else {
        Local::now().format(format)
    };

    println!("{output}");
}
