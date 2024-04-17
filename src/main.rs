use std::time::Duration;
use tokio::sync::Mutex;
use std::sync::Arc;
use structopt::StructOpt;
pub mod alloc;
#[cfg(target_family = "unix")]
mod daemon;
mod error;
mod proxy;
mod update;
mod util;

use clap::{Args, Parser, Subcommand};
use std::{net::SocketAddr, path::PathBuf};

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Opt {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run server
    Run(BootArgs),
    /// Start server daemon
    #[cfg(target_family = "unix")]
    Start(BootArgs),
    /// Restart server daemon
    #[cfg(target_family = "unix")]
    Restart(BootArgs),
    /// Stop server daemon
    #[cfg(target_family = "unix")]
    Stop,
    /// Show the server daemon process
    #[cfg(target_family = "unix")]
    Status,
    /// Show the server daemon log
    #[cfg(target_family = "unix")]
    Log,
    /// Update the application
    Update,
}

#[derive(Args, Clone, Debug)]
pub struct BootArgs {
    /// Debug mode
    #[clap(short = 'L', long, global = true, env = "VPROXY_DEBUG")]
    debug: bool,
    /// Bind address
    #[clap(short = 'B', long, default_value = "0.0.0.0:8100")]
    bind: SocketAddr,
    /// Basic auth username
    #[clap(short = 'u', long)]
    auth_user: Option<String>,
    /// Basic auth password
    #[clap(short = 'p', long)]
    auth_pass: Option<String>,
    /// TLS certificate file
    #[clap(short = 'C', long)]
    tls_cert: Option<PathBuf>,
    /// TLS private key file
    #[clap(short = 'K', long)]
    tls_key: Option<PathBuf>,
    /// Ipv6 subnet, e.g. 2001:db8::/32
    #[clap(short = 'i', long)]
    ipv6_subnet: Option<cidr::Ipv6Cidr>,
    /// Fallback address
    #[clap(short = 'f', long)]
    fallback: Option<std::net::IpAddr>,
    /// Proxy type, e.g. http, https, socks5
    #[clap(short = 't', long, default_value = "http")]
    typed: ProxyType,
    /// Session ID to IP mapping with TTL
    #[clap(long)]
    session_ttl: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

impl std::str::FromStr for ProxyType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "socks5" => Ok(Self::Socks5),
            _ => Err("".to_string()),
        }
    }
}

fn main() -> crate::Result<()> {
    let opt = Opt::parse();

    // Define your own session TTL
    let session_ttl = opt.session_ttl.unwrap_or(3600); // Use provided TTL or default to 1 hour

    // Create SessionManager with the provided TTL
    let session_manager = Arc::new(Mutex::new(SessionManager::new(Duration::from_secs(session_ttl))));

    match opt.commands {
        Commands::Run(mut args) => {
            let session_manager = SessionManager::new();  // Initialize SessionManager
            args.session_manager = Some(session_manager); // Pass SessionManager to BootArgs
            proxy::run(args)?; // Run the proxy with BootArgs
        },
        #[cfg(target_family = "unix")]
        Commands::Start(args) => daemon::start(args)?,
        #[cfg(target_family = "unix")]
        Commands::Restart(args) => daemon::restart(args)?,
        #[cfg(target_family = "unix")]
        Commands::Stop => daemon::stop()?,
        #[cfg(target_family = "unix")]
        Commands::Status => daemon::status(),
        #[cfg(target_family = "unix")]
        Commands::Log => daemon::log()?,
        Commands::Update => update::update()?,
    };
    
    Ok(())
}