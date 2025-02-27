use clap::Parser;
mod keypair;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(long)]
    key: String,
}

fn main() {
    let args = Args::parse();
    keypair::store_keypair(&args.name, &args.key);
}
