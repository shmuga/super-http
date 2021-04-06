use crate::indexer::HtmlIndexer;
use crate::prelude::*;
use actix::prelude::*;
use actix::Message;
use log::info;
use serde_json::{from_str, Value};
use tantivy::schema::Schema;
use tantivy::Index;
use tantivy::{collector::TopDocs, query::QueryParser, schema::Field};

#[derive(Message)]
#[rtype(result = "Result<Value>")]
pub struct SearchAll(pub String);

fn search(index: &Index, schema: &Schema, query: String, fields: &[Field]) -> Result<Value> {
    info!("Searching {} via fields {:?}", query, fields);
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, fields.to_vec());
    let query = query_parser.parse_query(&query)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    let top_docs: Value = top_docs
        .to_vec()
        .iter()
        .filter_map(|(_, addr)| searcher.doc(*addr).ok())
        .map(|d| schema.to_json(&d))
        // FIXME: do we really need this double convertion?
        .filter_map(|j| from_str::<Value>(&j).ok())
        .collect();

    Ok(top_docs)
}

impl Handler<SearchAll> for HtmlIndexer {
    type Result = Result<Value>;

    fn handle(&mut self, msg: SearchAll, _ctx: &mut Self::Context) -> Self::Result {
        search(
            &self.0.index,
            &self.0.schema,
            msg.0,
            &[
                self.0.tags_field,
                self.0.title_field,
                self.0.body_field,
                self.0.filename_field,
            ],
        )
    }
}
