// course info https://cloud.timeedit.net/liu/web/schema/objects/{OBJECT_ID}/o.json?fr=t gives JSON
//  schema genrerator: https://cloud.timeedit.net/liu/web/schema/sid=3

use dioxus::prelude::*;
use serde::Deserialize;

const STATIC_ASSETS: Asset = asset!("/assets/static");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut response_contents = use_signal(|| "wtf".to_string());

    let fetch = move |_| async move {
        info!("did");
        const YEAR: &str = "2026";
        let object_id_string = "153444";
        let response = reqwest::get(format!("https://cloud.timeedit.net/liu/web/schema/ri.ics?part=t&sid=3&p={YEAR}0101%2C{YEAR}1231&objects={object_id_string}")).await.unwrap().text().await.unwrap();
        response_contents.set(response);
    };

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/logo.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }
        input {
            oninput: fetch
        }
        div {
            {response_contents}
        }
    }
}
