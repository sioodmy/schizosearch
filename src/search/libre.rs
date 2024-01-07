use std::collections::HashMap;

lazy_static! {
    static ref UNFREE: HashMap<&'static str, &'static str> = vec![
        ("en.wikipedia.org", "wikiless.org"),
        ("imgur.com", "rimgo.projectsegfau.lt"),
        ("medium.com", "scribe.rip"),
        ("tekstowo.pl", "davilarek.github.io/TekstoLibre/?"),
        ("stackoverflow.com", "code.whatever.social")
    ]
    .into_iter()
    .collect();
}

/// Replace some unfree websites with libre frontends
pub fn libre(url: String) -> String {
    let mut new = url;
    for (unfree, libre) in UNFREE.clone() {
        new = new.replacen(unfree, libre, 1);
    }
    new
}
