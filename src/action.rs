use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Action {
    Test {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    Hoge {
        #[arg(short, long)]
        fuga: String,
    }
}
