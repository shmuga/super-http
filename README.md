# Super http

This is a basic http server that serves static content and also indexes all the files.
Includes following features:

- Full text index using [tantivy search index](https://docs.rs/tantivy/0.14.0/tantivy/query/struct.QueryParser.html)
- Forwards/Backwards links
- Provides api for the search
- Injects a search page at /search path


## TODO
- Graph of indexed documents
- Add watch mode for new files
- Connections graph for the specific document
- Introduce a better way to inject graphs/search into templates
- Move index into the filesystem
- Refactor search page
