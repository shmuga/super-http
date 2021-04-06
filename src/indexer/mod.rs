pub mod fs;
pub mod handlers;
pub mod parser;
pub mod links_storage;

use actix::prelude::*;
use tantivy::schema::*;
use tantivy::Index;

use self::links_storage::LinksStorage;

pub struct HtmlIndexerInternal {
    pub is_indexing: bool,
    pub schema: Schema,
    pub index: Index,
    pub tags_field: Field,
    pub filename_field: Field,
    pub body_field: Field,
    pub title_field: Field,
    pub links_storage: LinksStorage,
}

pub struct HtmlIndexer(pub HtmlIndexerInternal);

impl HtmlIndexerInternal {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        let tags_field = schema_builder.add_text_field("tag", TEXT | STORED);
        let filename_field = schema_builder.add_text_field("filename", TEXT | STORED);
        let body_field = schema_builder.add_text_field("body", TEXT | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let schema = schema_builder.build();

        // TODO: move index to disk using path provided by Opts
        let index = Index::create_in_ram(schema.clone());

        HtmlIndexerInternal {
            is_indexing: true,
            schema,
            index,
            tags_field,
            filename_field,
            body_field,
            title_field,
            links_storage: LinksStorage::default()
        }
    }
}

impl Actor for HtmlIndexer {
    type Context = Context<Self>;
}
