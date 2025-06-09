use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct ARGs {
    #[arg(short, long)]
    pub program: Option<String>,
    #[arg(short, long)]
    pub repo: Option<String>,
}
