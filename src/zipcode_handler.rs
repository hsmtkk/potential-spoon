use crate::html::Render;
use crate::zipcode_api::{Searcher};
use iron::status;
use iron::{Handler, IronError, Request, Response, Set};
use log::{error, info};

pub struct ZipcodeHandler {
    searcher: Searcher,
    render: Render,
}

impl ZipcodeHandler {
    pub fn new(searcher: Searcher, render: Render) -> ZipcodeHandler {
        ZipcodeHandler { searcher, render }
    }
}

impl Handler for ZipcodeHandler {
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        info!("{:?}", req);
        let query = req.url.query().unwrap();
        info!("query {}", query);
        let elems: Vec<&str> = query.split("=").collect();
        let zip_code = elems[1];
        let addresses = self.searcher.search(zip_code).unwrap();
        let html = self.render.render(addresses).unwrap();
        Ok(Response::with((html, status::Ok)))
    }
}
