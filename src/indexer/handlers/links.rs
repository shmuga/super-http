use url::Url;
use crate::indexer::links_storage::Link;
use crate::indexer::HtmlIndexer;
use crate::prelude::*;
use actix::prelude::*;
use actix::Message;
use serde_json::Value;

#[derive(Message)]
#[rtype(result = "Result<Value>")]
pub struct GetForward(pub String);

impl Handler<GetForward> for HtmlIndexer {
    type Result = Result<Value>;

    fn handle(&mut self, msg: GetForward, _ctx: &mut Self::Context) -> Self::Result {
        let path = Url::parse(&msg.0)
                    .map(|u| u.path().to_string())
                    .unwrap_or_else(|_| "/".to_string());

        let links = self
            .0
            .links_storage
            .get_forward(Link::new(None, path, String::new()));

        // FIXME: remove unwrap
        Ok(serde_json::to_value(links).unwrap())
    }
}

#[derive(Message)]
#[rtype(result = "Result<Value>")]
pub struct GetBackward(pub String);

impl Handler<GetBackward> for HtmlIndexer {
    type Result = Result<Value>;

    fn handle(&mut self, msg: GetBackward, _ctx: &mut Self::Context) -> Self::Result {
        let path = Url::parse(&msg.0)
                    .map(|u| u.path().to_string())
                    .unwrap_or_else(|_| "/".to_string());

        let links = self
            .0
            .links_storage
            .get_backward(Link::new(None, path, String::new()));

        // FIXME: remove unwrap
        Ok(serde_json::to_value(links).unwrap())
    }
}
