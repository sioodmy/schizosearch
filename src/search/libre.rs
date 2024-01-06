use std::collections::HashMap;
use crate::search::libre::Alternative::{Single, Multiple};
use rand::seq::SliceRandom; 

#[derive(Clone)]
enum Alternative {
    Single(&'static str),
    Multiple(Vec<&'static str>)
}

lazy_static! {
    static ref UNFREE: HashMap<&'static str, Alternative> = vec![
        ("en.wikipedia.org", Single("wikiless.org")),
        ("imgur.com", Single("rimgo.projectsegfau.lt")),
        ("medium.com", Single("scribe.rip")),
        ("tekstowo.pl", Single("davilarek.github.io/TekstoLibre/?")),
        ("stackoverflow.com", Multiple(vec!["code.whatever.social", "ao.vern.cc", "overflow.fascinated.cc", "ao.owo.si", "a.opnxng.com"]))
    ]
    .into_iter()
    .collect();
}
pub fn libre(url: String) -> String {
    let mut new = url;
    for (unfree, libre) in UNFREE.clone() {
        let instance = match libre {
            Alternative::Single(instance)=> instance,
            Alternative::Multiple(instances) =>
            instances.choose(&mut rand::thread_rng()).unwrap()
        };
        new = new.replacen(unfree, instance, 1);
    }
    new
}
