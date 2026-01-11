use dioxus::prelude::*;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::fmt::Write;
use std::sync::LazyLock;

use types::ObjectRecord;

static RECORDS: LazyLock<Vec<ObjectRecord>> = LazyLock::new(|| {
    wincode::deserialize::<Vec<ObjectRecord>>(include_bytes!("../assets/records.bin")).unwrap()
});
static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(SkimMatcherV2::default);
const LOGO: Asset = asset!("/assets/logo.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut search_text = use_signal(String::new);
    let mut selected_ids = use_signal(Vec::<u32>::new); // FIXME: should be Vec<&ObjectRecord>, will avoid later crap in rsx

    let mut search_results = use_action(|query: String| async move {
        let mut results: Vec<_> = RECORDS
            .iter()
            .filter_map(|r| {
                MATCHER
                    .fuzzy_match(&r.values, &query)
                    .map(|score| (score, r))
            })
            .collect();

        results.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by score
        dioxus::Ok(results.into_iter().map(|(_, r)| r).collect::<Vec<_>>())
    });

    let search_results_table = if !search_text().is_empty() {
        match search_results.value() {
            Some(Ok(records)) => rsx! {
                table {
                    id: "search-result",
                    for record in records() {
                        tr {
                            td {
                                label {
                                    for: "checkbox-{record.id}",
                                    "{record.values}"
                                }
                            }
                            td {
                                input {
                                    type: "checkbox",
                                    id: "checkbox-{record.id}",
                                    checked: selected_ids().contains(&record.id),
                                    onchange: move |_| {
                                        if let Some(pos) = selected_ids().into_iter().position(|id| id == record.id) {
                                            selected_ids.write().swap_remove(pos);
                                        } else {
                                            selected_ids.write().push(record.id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Some(Err(e)) => rsx! {
                span {
                    color: "red",
                    "fel: {e}",
                }
            },
            _ => rsx! {
                div { "..." }
            },
        }
    } else {
        rsx! {""}
    };

    let generated_url = use_memo(move || {
        const ID_LENGTH: usize = 6;
        let ids = selected_ids();

        let mut url = String::with_capacity(128 + ids.len() * ID_LENGTH);
        url.push_str(
            "https://cloud.timeedit.net/liu/web/schema/ri.ics?sid=3&p=20250101,20270101&objects=",
        );

        for (i, id) in ids.iter().enumerate() {
            if i > 0 {
                url.push(',');
            }
            write!(&mut url, "{id}").unwrap();
        }

        url
    });

    rsx! {
        document::Link { rel: "icon", href: LOGO }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }

        header {
            h1 {
                img { src: LOGO, height: 50 }
                { env!("CARGO_PKG_NAME") }
            }
            i { "för den som hatar TimeEdit..." }
        }

        main {

            input {
                placeholder: "Sök...",
                oninput: move |e| {
                    let value = e.value();
                    search_results.call(value.clone());
                    search_text.set(value);
                }
            }


            div {
                id: "selections",
                if selected_ids().is_empty() {
                   i { "Välj kurs och/eller studentgrupp från listan nedan. Använd sökrutan ovan för att filtera listan!" }
                } else {
                    table {
                        for (i, id) in selected_ids().into_iter().enumerate() {
                            if let Some(selection) = RECORDS.iter().find(|r| r.id == id) {
                                tr {
                                    td { span { "{selection.values}" } }
                                    td {
                                        button {
                                            onclick: move |_| { selected_ids.write().swap_remove(i); },
                                            "❌"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    { generated_url }
                }
            }

            {search_results_table}

        }

        footer {
            "Skapad av "
            a {
                href: "https://github.com/sermuns/",
                "Samuel \"sermuns\" Åkesson"
            }
        }
    }
}
