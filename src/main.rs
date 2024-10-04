#![allow(non_snake_case, unused)]

//! Run with:
//!
//! ```sh
//! dx build --features web --release
//! cargo run --features ssr
//! ```

mod homepage;
use homepage::Home;
mod projects;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    launch(|| {
        rsx! {
            head::Link { rel: "stylesheet", href: asset!("./public/output.css") }
            Router::<Route> {}
        }
    })
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}
