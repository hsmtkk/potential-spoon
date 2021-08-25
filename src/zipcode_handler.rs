use iron::{Handler, IronError, Request, Response};
use iron::status;

pub struct ZipcodeHandler {}

impl ZipcodeHandler {
    pub fn new() -> ZipcodeHandler {
        ZipcodeHandler{}
    }
}

impl Handler for ZipcodeHandler {
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        Ok(Response::with((status::Ok, "zipcode")))
    }
}