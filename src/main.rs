/* Main
  [ ] done
  [ ] refactor

  The following modules are included:

  [ ] shared: common functions and macros
  [ ] revshell: module for serving different types of reverse shell
  [ ] c2: command and control framework module
  [ ] cheatsheet: a handbook tool with basic commands
*/

use std::convert::Infallible;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use clap::Parser;
use colored::Colorize;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use json::JsonValue;
use tmux_interface::{NewSession, SendKeys, Tmux};
use tokio::runtime;
use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::greed::Cli;
use crate::revshell::*;
use crate::revshell::revshell_model::RevShells;
use crate::shared::logger::*;
use crate::shared::xmind::XMindJson;
use crate::shared::{utils, xmind};
use crate::tools::tools_model::*;
use crate::tools::*;
use crate::utils::format_duration;

mod cheatsheets;
mod greed;
mod revshell;
mod shared;
mod tools;

#[tokio::main]
async fn main() {
    let greed = Cli::parse();
    let download_mode = greed.download;
    let should_ip_be_local = greed.local;
    let start_time: Instant = Instant::now();
    let lport_tool = 8080;
    let lport_revshell = 8081;

    let lport_current = Arc::new(Mutex::new(4444));

    let _ = execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    );

    // 1. env configuration
    let lhost: IpAddr = if should_ip_be_local {
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    } else {
        utils::get_lhost()
    };

    logger_info!(format!(
        "{} set to {}",
        "LHOST".green().bold(),
        &lhost.to_string().green().bold()
    ));

    let rhost: Option<Ipv4Addr> = utils::get_rhost(&greed);

    // 2. get data from xmind file
    let xmind_json: JsonValue = xmind::get_content_from_xmind();
    logger_info!(format!(
        "XMind file {} loaded successfully.",
        "winner.xmind".green()
    ));

    // 3. Tools module
    let tools = Tools::from_root_json(&xmind_json);

    // 4. Revshell module
    let rev_shells = RevShells::from_root_json(&xmind_json);
    logger_info!("{}", format!("{} {} found: {}", rev_shells.revshells().len().to_string().green(),"revshels".green().bold(), rev_shells.to_string().green()));

    // __________________________________________________

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    // Tools Server
    let make_service_tools = make_service_fn(move |_conn: &AddrStream| {
        let tools = tools.clone();
        let service =
            service_fn(move |req| tools_server::listen(tools.clone(), req, download_mode));
        async move { Ok::<_, Infallible>(service) }
    });

    let tools_http_server =
        hyper::Server::bind(&SocketAddr::new(lhost, lport_tool)).serve(make_service_tools);

    // Revshell Server
    let make_service_revshell = make_service_fn(move |_conn: &AddrStream| {
        let revshells = rev_shells.clone();
        let lhost_clone = lhost.clone();
        let service =
            service_fn(move |req| revshell_server::listen(req, revshells.clone(), lhost_clone.to_string()));
        async move { Ok::<_, Infallible>(service) }
    });

    let revshell_http_server =
        hyper::Server::bind(&SocketAddr::new(lhost, lport_revshell)).serve(make_service_revshell);


    runtime.spawn(tools_http_server);
    runtime.spawn(revshell_http_server);
    logger_info!(format!(
        "servers started in {} {}{}",
        format_duration(&start_time.elapsed()),
        Icons::Rocket,
        Icons::Rocket
    ));


    let target_session = "example_1";

    // tmux new -d -s example_1 ; neww ; splitw -v
    Tmux::new()
        .add_command(NewSession::new().detached().session_name(target_session))
        .add_command(SendKeys::new().key("p").build())
        .output()
        .unwrap();

    // Tmux::with_command(HasSession::new().target_session(target_session)).add_command(SendKeys::key("d"));

    logger_info!(format!("session open at {}", target_session));




    print_welcome(
        &lhost.to_string(),
        &lport_tool,
        &lport_revshell,
        rhost.unwrap().to_string().as_str(),
    );

    // print!("query: ");
    let _ = io::stdout().flush();
    let lines = std::io::stdin().lines();
    for line in lines {
        let _ = execute!(
            std::io::stdout(),
            Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
        print_welcome(
            &lhost.to_string(),
            &lport_tool,
            &lport_revshell,
            rhost.unwrap().to_string().as_str()
        );

        let line_str = line.unwrap();

        if !line_str.is_empty() {
            let _ = cheatsheets::print_cheat_sheets(
                line_str.as_str(),
                lhost.to_string().as_str(),
                rhost.unwrap().to_string().as_str(),
                &*lport_current.lock().await.to_string()
            );
        }

        // print!("query: ");
        let _ = io::stdout().flush();
    }
}

fn print_welcome(lhost: &str, lport_tools: &u16, lport_revshell: &u16, rhost: &str) {
    println!(" ┌───────────────────────────────────────────────────────────┐   ");
    println!(
        " │  {}{}              tools: {:<28}│",
        Icons::Medal.to_string().bold().yellow(),
        "Winner".yellow().bold(),
        format!("http://{}:{}", lhost, lport_tools).blue()
    );
    println!(
        " │    {}       revshels: {:<28}│",
        "by n4zz4r1".white(),
        format!("http://{}:{}", lhost, lport_revshell).blue()
    );
    println!(
        " │                        {:<28}       │",
        format!("RHOST: {}", rhost).green()
    );

    println!(" └───────────────────────────────────────────────────────────┘   ");

    // {}{:<24}
}
