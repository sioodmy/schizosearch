use std::collections::HashMap;

lazy_static! {
    static ref UNFREE: HashMap<&'static str, &'static str> = vec![
        ("en.wikipedia.org", "wikiless.org"),
        ("imgur.com", "rimgo.projectsegfau.lt")
    ]
    .into_iter()
    .collect();
}
pub fn libre(url: String) -> String {
    let mut new = url;
    for (unfree, libre) in UNFREE.clone() {
        new = new.replacen(unfree, libre, 1);
    }
    new
}
