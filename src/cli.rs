#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub sub_command: CliSubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum CliSubCommand {
    Add {
        #[clap(short, long)]
        name: String,

        #[clap(long)]
        key: String,
    },

    Get {
        #[clap(short)]
        balance: bool,
    },

    Send {
        #[clap(short)]
        value: f64,

        #[clap(short)]
        account: String,

        #[clap(short)]
        to: String,
    },
}
