use crate::html::Render;
use crate::zipcode_api::Address;
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
        let addresses = vec![Address::new(
            "7830060",
            "39",
            "高知県",
            "南国市",
            "蛍が丘",
        )];
        match self.render.render(addresses) {
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
