use iron::{Handler, IronError, Request, Response};
use iron::status;
use log::info;

pub struct IndexHandler {}

impl IndexHandler {
    pub fn new() -> IndexHandler {
        IndexHandler{}
    }
}

impl Handler for IndexHandler {
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        info!("{:?}", req);
        Ok(Response::with((status::Ok, "index")))
    }
}