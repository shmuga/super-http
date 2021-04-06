use crate::indexer::links_storage::Link;
use crate::indexer::{fs, parser, HtmlIndexer};
use crate::prelude::*;
use crate::ParsedFile;
use actix::prelude::*;
use actix::Message;
use log::info;
use tantivy::Document;

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct StartIndexing(pub String);

impl Handler<StartIndexing> for HtmlIndexer {
    type Result = Result<()>;

    fn handle(&mut self, message: StartIndexing, _: &mut Self::Context) -> Self::Result {
        info!("Indexing started");

        let path = message.0;
        let files = fs::walk(path.clone())?;
        let inter = &mut self.0;
        let mut index_writer = inter.index.writer(30_000_000)?;

        info!("Found {} files to be indexed", files.len());
        for file in files.iter() {
            let content = fs::file(&file)?;
            let ParsedFile {
                body,
                title,
                tags,
                headers,
                links,
            } = parser::parse_content(content)?;

            let mut doc = Document::new();
            let no_basedir = file.to_string().replace(&path, "");

            doc.add_text(inter.filename_field, no_basedir.clone());
            // TODO: replace multiple consecutive spaces in body
            doc.add_text(inter.body_field, body);
            doc.add_text(inter.title_field, title.clone());

            for tag in tags.iter() {
                doc.add_text(inter.tags_field, tag);
            }

            let file_link = Link::new(Some("/".to_string()), no_basedir.clone(), title.clone());
            let links: Vec<Link> = links
                .iter()
                .map(|(href, title)| Link::new(Some(no_basedir.clone()), href.to_owned(), title.to_owned()))
                .collect();

            inter.links_storage.update_file(file_link, links);

            for header in headers.iter() {
                doc.add_text(inter.title_field, header);
            }

            index_writer.add_document(doc);
        }

        println!("{:?}", inter.links_storage);
        index_writer.commit()?;
        self.0.is_indexing = false;

        Ok(())
    }
}
