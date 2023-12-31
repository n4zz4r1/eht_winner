// /* Tools Server
//   [ ] done
//   [ ] refactor
// */
use crate::tools::tools_model::Tools;
use crate::*;
use hyper::http::response::Builder;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use tokio::fs;

pub async fn listen<T>(
    tools: Tools,
    req: Request<T>,
    _download_mode: bool,
) -> Result<Response<Body>, Infallible> {
    if &req.uri().to_string() == "/" {
        return return_list_of_tools(&tools).await;
    } else if &req.uri().to_string() == "/favicon.ico" {
        // ignore annoying favicon
        return Ok(Builder::new()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    }

    let parts: &Vec<&str> = &req.uri().path().split('/').collect();
    if parts.len() == 3 && tools.exists(parts[1], parts[2]) {
        let tool = tools.get_by_os_and_name(parts[1], parts[2]);

        if tool.labels().contains(&"url".to_string()) {
            get_from_url(tool).await
        } else {
            get_from_local_file(tool).await
        }
    } else {
        logger_trace!(format!(
            "tools > {}{} not found. Was it mapped on xmind?",
            Icons::Error.to_string(),
            &req.uri().path().to_string().bold()
        ));
        Ok(Builder::new()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap())
    }
}

async fn return_list_of_tools(tools: &Tools) -> Result<Response<Body>, Infallible> {
    let mut result_html = String::new();
    result_html.push_str("<!DOCTYPE html><html><head><!-- <link rel=\"icon\" href=\"favicon.ico\" type=\"image/x-icon\"> --></head><body style=\"background-color:black;color:white\">");
    for tool in tools.get_all_tools() {
        result_html.push_str(
            format!(
                "{} / <a style=\"color:white\" href=\"{}\">{}</a><br>",
                tool.os(),
                tool.link_name(),
                tool.title()
            )
            .as_str(),
        );
    }
    result_html.push_str("</html>");

    Ok(Builder::new()
        .status(StatusCode::OK)
        .body(Body::from(result_html))
        .unwrap())
}

async fn get_from_local_file(tool: &Tool) -> Result<Response<Body>, Infallible> {
    let path = format!("/opt/winner/scripts/{}/{}", tool.os(), tool.title());
    match fs::read(&path).await {
        Ok(file) => {
            logger_debug!(format!(
                "{} tools > LOCAL file {} downloaded.",
                Icons::Download.to_string().blue().bold(),
                tool.title().blue().bold()
            ));
            Ok(Builder::new()
                .status(StatusCode::OK)
                .body(Body::from(file))
                .unwrap())
        }
        Err(_) => {
            logger_trace!(format!(
                "tools > {}LOCAL file {} doesn't exists.",
                Icons::Error.to_string(),
                &path
            ));
            Ok(Builder::new()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    }
}

async fn get_from_url(tool: &Tool) -> Result<Response<Body>, Infallible> {
    match reqwest::get(tool.href()).await {
        Ok(response) => {
            let body = response.bytes().await.unwrap_or_default();
            // Get response body bytes
            logger_debug!(format!(
                "{} tools > URL file {} downloaded.",
                Icons::Download.to_string().blue().bold(),
                tool.title().blue().bold()
            ));
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(body))
                .unwrap())
        }
        Err(_) => {
            logger_warn!(format!(
                "tools > {} URL file {} seems to have an incorrect href. {}",
                Icons::Error.to_string(),
                tool.title(),
                tool.href()
            ));
            Ok(Builder::new()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap())
        }
    }
}
