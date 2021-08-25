use crate::html::Render;
use iron::status;
use iron::{Handler, IronError, Request, Response};
use log::{error, info};

pub struct ZipcodeHandler {
    render: Render,
}

impl ZipcodeHandler {
    pub fn new(render: Render) -> ZipcodeHandler {
        ZipcodeHandler { render }
    }
}

impl Handler for ZipcodeHandler {
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        info!("{:?}", req);
        match self.render.render() {
            Ok(html) => Ok(Response::with((status::Ok, html))),
            Err(e) => {
                let msg = format!("{}", e);
                error!("{}", msg);
                Err(IronError::new(
                    std::io::Error::new(std::io::ErrorKind::Other, msg),
                    status::InternalServerError,
                ))
            }
        }
    }
}
