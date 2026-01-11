// FROM OBJECT ID TO string -> course info https://cloud.timeedit.net/liu/web/schema/objects/{OBJECT_ID}/o.json?fr=t gives JSON
// get everything in JSON https://cloud.timeedit.net/liu/web/schema/objects//o.json?fr=f&types=205&sid=3 gives JSON
//  schema genrerator: https://cloud.timeedit.net/liu/web/schema/sid=3

use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ObjectSearchResponse {}

#[derive(Deserialize, Debug)]
struct Object {}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let fetch = use_resource(|| async {
        const YEAR: &str = "2026";
        let object_id_string = "153444";
        let response = reqwest::get(format!(
            "https://cloud.timeedit.net/liu/web/schema/objects//o.json?fr=f&types=205&sid=3"
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
        response
    });

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/logo.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }
        div {
            {fetch}
        }
    }
}
