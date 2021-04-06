use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::hash::Hasher;
use url::{ParseError, Url};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub href: String,
    pub is_external: bool,
}

impl Link {
    pub fn new(base: Option<String>, href: String, title: String) -> Self {
        let base = base.unwrap_or("/".to_string());
        let host = format!("https://localhost{}", base);

        match Url::parse(&href) {
            Err(ParseError::RelativeUrlWithoutBase) => Link {
                title,
                href: Url::parse(&host)
                    .and_then(|u| u.join(&href))
                    .map(|u| u.path().to_string())
                    .unwrap_or("PARSING_ERROR".to_string()),
                is_external: false,
            },
            _ => Link {
                title,
                href,
                is_external: true,
            },
        }
    }
}

impl Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.href.hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinksStorage {
    pub files: HashSet<Link>,
    forward: HashMap<String, HashSet<Link>>,
    backward: HashMap<String, HashSet<Link>>,
}

impl Default for LinksStorage {
    fn default() -> Self {
        LinksStorage {
            files: HashSet::new(),
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
}

fn derive_filename(from: &Link) -> Option<String> {
    if let Ok(parsed) = Url::parse(&from.href) {
        if let Some(_) = parsed.host() {
            None
        } else {
            Some(parsed.path().to_owned())
        }
    } else {
        None
    }
}

impl LinksStorage {
    pub fn get_backward(&self, source: Link) -> HashSet<Link> {
        self.backward
            .get(&source.href)
            .unwrap_or(&HashSet::<Link>::new())
            .clone()
    }

    pub fn get_forward(&self, source: Link) -> HashSet<Link> {
        self.forward
            .get(&source.href)
            .unwrap_or(&HashSet::<Link>::new())
            .clone()
    }

    pub fn update_file(&mut self, source: Link, links: Vec<Link>) {
        let forward = self
            .forward
            .get(&source.href)
            .unwrap_or(&HashSet::<Link>::new())
            .clone();

        let to_add: HashSet<_> = links.iter().cloned().collect();

        // for each one missing in to_add -> remove backlinks
        for removed_link in forward.difference(&to_add.clone()) {
            if !removed_link.is_external {
                if let Some(backlink) = self.backward.get_mut(&removed_link.href) {
                    backlink.take(&removed_link);
                }
            }
        }

        // for each one new link in to_add -> add backlinks
        for added_link in to_add.clone().difference(&forward) {
            if !added_link.is_external {
                match self.backward.get_mut(&added_link.href) {
                    Some(backlink) => {
                        backlink.insert(source.clone());
                    }
                    None => {
                        let mut set = HashSet::new();
                        set.insert(source.clone());
                        self.backward.insert(added_link.href.clone(), set);
                    }
                }
            }
        }

        self.files.insert(source.clone());
        self.forward.insert(source.href.clone(), to_add.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn link(href: &str) -> Link {
        Link::new(None, href.to_string(), format!("title-{}", href))
    }

    fn set(links: Vec<Link>) -> HashSet<Link> {
        links.iter().cloned().collect::<HashSet<Link>>()
    }

    #[test]
    fn test_create_link() {
        let external = &link("https://google.com.ua");
        assert_eq!(external.is_external, true);

        let internal1 = link("./starting/with/dot");
        assert_eq!(internal1.href, "/starting/with/dot".to_string());
        assert_eq!(internal1.is_external, false);

        let internal2 = link("starting/without/dot");
        assert_eq!(internal2.href, "/starting/without/dot".to_string());
    }

    #[test]
    fn test_create_link_base() {
        let internal = Link::new(
            Some("/some/path/file.html".to_string()),
            "../auth/mod".to_string(),
            "Title".to_string(),
        );
        assert_eq!(internal.href, "/some/auth/mod".to_string());
    }

    #[test]
    fn test_update_file() {
        let mut ls = LinksStorage::default();
        let index = link("/todo/index");

        let a = link("./todo/archive");
        let b = link("todo/today");
        let c = link("todo/tomorrow");

        ls.update_file(index.clone(), vec![a.clone(), b.clone(), c.clone()]);

        assert_eq!(
            ls.get_forward(index.clone()),
            set(vec![a.clone(), b.clone(), c.clone()])
        );
        assert_eq!(ls.get_backward(a), set(vec![index.clone()]));
        assert_eq!(ls.get_backward(b), set(vec![index.clone()]));
        assert_eq!(ls.get_backward(c), set(vec![index.clone()]));
    }

    #[test]
    fn test_different_relative_links() {
        let mut ls = LinksStorage::default();
        let index = link("/todo/index");

        let a = link("./todo/archive");
        let b = link("todo/archive");

        ls.update_file(index.clone(), vec![a.clone(), b.clone()]);

        assert_eq!(ls.get_forward(index.clone()).len(), 2);
        assert_eq!(ls.backward.len(), 1);
    }
}
