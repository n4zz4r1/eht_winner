use clap::Parser;

// ref.: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser)]
#[command(name = "Winner")]
#[command(author = "n4zz4r1 <nazzari_red@pm.me>")]
#[command(author, version, about)]
pub struct Cli {
    /// Download all tools at once before serve.
    #[arg(short, long, default_value_t = false)]
    pub download: bool,
    /// Provided rhost ip
    #[arg(long)]
    pub rhost: Option<String>,
    /// Provided lport ip for revshell
    #[arg(long)]
    pub lport: Option<u16>,
    /// Flag to use local ip address.
    #[arg(short, long, default_value_t = false)]
    pub local: bool,
}
