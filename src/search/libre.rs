use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Clone)]
enum Alternative {
    Single(&'static str),
    Multiple(Vec<&'static str>),
}

//FIXME
macro_rules! libre{
    ($unfree:literal to $($libre:literal),+) => {
        ($unfree, Alternative::Multiple(vec![$($libre),+]))
    };
    ($unfree:literal to $libre:literal) => {
        ($unfree, Alternative::Single($libre))
    }
}

lazy_static! {
    static ref UNFREE: HashMap<&'static str, Alternative> = vec![
        libre!("medium.com" to "scripe.com"),
        libre!("en.wikipedia.org" to "wikiless.org"),
        libre!("imgur.com" to "rimgo.projectsegfau.lt"),
        libre!("tekstowo.pl" to "davilarek.github.io/TekstoLibre/?"),
        libre!("stackoverflow.com" to "code.whatever.social", "ao.vern.cc", "overflow.fascinated.cc", "ao.owo.si", "a.opnxng.com")
    ].into_iter().collect();
}
pub fn libre(url: String) -> String {
    let mut new = url;
    for (unfree, libre) in UNFREE.clone() {
        let instance = match libre {
            Alternative::Single(instance) => instance,
            Alternative::Multiple(instances) => instances.choose(&mut rand::thread_rng()).unwrap(),
        };
        new = new.replacen(unfree, instance, 1);
    }
    new
}
