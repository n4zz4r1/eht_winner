/* Revshell Server
  [ ] done
  [ ] refactor
*/
// todo: do

use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn listen(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("TODO".into()))
}
