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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;

use clap::Parser;
use colored::Colorize;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use json::JsonValue;
use tokio::runtime;
use tokio::time::{Instant};

use crate::greed::Cli;
use crate::revshell::*;
use crate::shared::logger::*;
use crate::shared::xmind::XMindJson;
use crate::shared::{utils, xmind};
use crate::tools::tools_model::*;
use crate::tools::*;
use crate::utils::format_duration;

mod greed;
mod revshell;
mod shared;
mod tools;
mod cheatsheets;

use std::io::{self, Write};
#[tokio::main]
async fn main() {
    let greed = Cli::parse();
    let download_mode = greed.download;
    let should_ip_be_local = greed.local;
    let start_time: Instant = Instant::now();
    let lport_tool = 8080;
    let lport_revshell = 8081;

    execute!(
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

    // __________________________________________________

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    // A `MakeService` that produces a `Service` to handle each connection.
    let make_service_tools = make_service_fn(move |_conn: &AddrStream| {
        let tools = tools.clone();
        let service =
            service_fn(move |req| tools_server::listen(tools.clone(), req, download_mode));
        async move { Ok::<_, Infallible>(service) }
    });

    let tools_http_server =
        hyper::Server::bind(&SocketAddr::new(lhost, lport_tool)).serve(make_service_tools);
    runtime.spawn(tools_http_server);

    let revshell_make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(revshell_server::listen))
    });
    let revshell_http_server =
        hyper::Server::bind(&SocketAddr::new(lhost, lport_revshell)).serve(revshell_make_svc);
    runtime.spawn(revshell_http_server);

    // done
    logger_info!(format!(
        "{} Tools server started at {}",
        Icons::Rocket,
        format!("http://{}:{}", lhost, lport_tool).blue()
    ));
    logger_info!(format!(
        "{} Revshell server started at {}",
        Icons::Rocket,
        format!("http://{}:{}", lhost, lport_revshell).blue()
    ));

    logger_info!(format!(
        "servers started in {} {}{}",
        format_duration(&start_time.elapsed()),
        Icons::Rocket,
        Icons::Rocket
    ));

    print_welcome(&lhost.to_string(), &lport_tool, &lport_revshell, &rhost.unwrap().to_string().as_str());

    print!("query: ");
    let _ = io::stdout().flush();
    let lines = std::io::stdin().lines();
    for line in lines {

        execute!(
            std::io::stdout(),
            Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
        print_welcome(&lhost.to_string(), &lport_tool, &lport_revshell, &rhost.unwrap().to_string().as_str());
        let line_str = line.unwrap();

        if !line_str.is_empty() {
            let _ = cheatsheets::print_cheat_sheets(line_str.as_str(), lhost.to_string().as_str(), rhost.unwrap().to_string().as_str());
        }

        print!("query: ");
        let _ = io::stdout().flush();
    }
}


fn print_welcome(lhost: &str, lport_tools: &u16, lport_revshell: &u16, rhost: &str) {
    println!(" ┌───────────────────────────────────────────────────────────┐   ");
    println!(
        " │  {}{}              tools: {:<20}       │",
        Icons::Medal.to_string().bold().yellow(),
        "Winner".yellow().bold(),
        format!("http://{}:{}", lhost, lport_tools).blue()
    );
    println!(" │    {}       revshels: {:<20}       │", "by n4zz4r1".white(), format!("http://{}:{}", lhost, lport_revshell).blue());
    println!(" │                        {:<20}               │", format!("RHOST: {}", rhost).green());

    println!(" └───────────────────────────────────────────────────────────┘   ");

    // {}{:<24}
}
