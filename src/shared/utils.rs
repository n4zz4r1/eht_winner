/* Utils module from Shared
  [ ] done
  [ ] refactor
*/
use std::net::IpAddr;
use std::process::exit;
use std::time::Duration;
use clap::Command;

use local_ip_address::{list_afinet_netifas, local_ip};

use crate::*;

pub fn format_duration(duration: &Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    let micro = duration.subsec_micros();

    format!("{:01}s.{:03}ms.{:02}μs", seconds, millis, micro)
}

pub fn get_lhost() -> IpAddr {
    let network_interfaces = list_afinet_netifas();
    if let Ok(network_interfaces) = network_interfaces {
        for (name, ip) in network_interfaces.iter() {
            if ip.is_ipv4() && name.contains("tun") {
                return *ip;
            }
        }
        logger_warn!(format!("no `tun` interface found"));
        local_ip().unwrap()
    } else {
        logger_error!(format!("whoops, local ip not found"));
        exit(1)
    }
}

pub fn get_rhost(greed: &Cli) -> Option<Ipv4Addr> {
    match &greed.rhost {
        None => {
            logger_warn!(
                format!("no {} found. consider using --rhost [IP]", "RHOST".bold()).yellow()
            );
            None
        }
        Some(rhost_p) => match &rhost_p.parse::<Ipv4Addr>() {
            Ok(ipv4) => {
                logger_info!(format!(
                    "{} set to {}",
                    "RHOST".green().bold(),
                    &rhost_p.green().bold()
                ));
                Some(*ipv4)
            }
            Err(_) => {
                logger_warn!(format!(
                    "{} not set, as it is malformed: `{}`",
                    "RHOST", &rhost_p
                ).yellow());
                None
            }
        },
    }
}

// pub fn cls() {
//     Command::new("clear").expe
// }