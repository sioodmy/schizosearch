use askama::Template;
use axum::response::Html;
use rand::seq::SliceRandom;

lazy_static! {
    static ref SPLASHES: Vec<&'static str> = include_str!("../splashes").lines().collect();
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage {
    splash: &'static str,
}

impl HomePage {
    fn new() -> Self {
        if let Some(splash) = SPLASHES.choose(&mut rand::thread_rng()) {
            HomePage { splash }
        } else {
            // shouldnt ever happen
            panic!("Couldn't get splash");
        }
    }
}

pub async fn homepage() -> Html<String> {
    let homepage = HomePage::new();
    Html(homepage.render().unwrap())
}
