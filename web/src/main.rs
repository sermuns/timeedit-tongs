use dioxus::{CapturedError, prelude::*};
use dioxus_sdk::time::use_debounce;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::fmt::Write;
use std::sync::LazyLock;
use std::time::Duration;

#[cfg(debug_assertions)]
use web_sys::window;

use types::ObjectRecord;

static RECORDS: LazyLock<Vec<ObjectRecord>> = LazyLock::new(|| {
    wincode::deserialize::<Vec<ObjectRecord>>(include_bytes!("../assets/records.bin")).unwrap()
});
const MATCH_LIMIT: usize = 20;
static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(|| {
    SkimMatcherV2::default()
        .element_limit(MATCH_LIMIT)
        .ignore_case()
});
const LOGO: Asset = asset!("/assets/logo.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn SearchResults(
    search_text: Signal<String>,
    search_results_value: Option<Result<ReadSignal<Vec<ObjectRecord>>, CapturedError>>,
    selected_ids: Signal<Vec<u32>>,
) -> Element {
    if search_text().is_empty() {
        return rsx! { "" };
    }

    match search_results_value {
        Some(Ok(records)) => {
            let rows = records.iter().map(|record| {
                let id = record.id;

                rsx! {
                    tr {
                        td {
                            label {
                                r#for: "checkbox-{id}",
                                "{record.values}"
                            }
                        }
                        td {
                            input {
                                r#type: "checkbox",
                                id: "checkbox-{id}",
                                checked: selected_ids().contains(&id),
                                onchange: move |_| {
                                    if let Some(pos) = selected_ids().iter().position(|x| *x == id) {
                                        selected_ids.write().swap_remove(pos);
                                    } else {
                                        selected_ids.write().push(id);
                                    }
                                }
                            }
                        }
                    }
                }
            });

            rsx! {
                table {
                    id: "search-result",
                    {rows}
                }
            }
        }
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
}

#[component]
fn SelectionsContainer(selected_ids: Signal<Vec<u32>>, generated_url: Memo<String>) -> Element {
    rsx! {
        div {
            id: "selections",
            if selected_ids().is_empty() {
               i {
                    "Sök efter kurs och/eller studentgrupp i sökrutan ovan. Kryssa i vad som ska ingå i din kalenderprenumeration. När du är nöjd, kopiera länken och importera till valfri kalenderapp."
                }
            } else {
                div {
                    id: "generated-url",
                    code { {generated_url} }
                    button {
                        "onclick": "navigator.clipboard.writeText(\"{generated_url}\")",
                        "📋"
                    }
                }
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
            }
        }
    }
}

#[component]
fn App() -> Element {
    let mut search_text = use_signal(String::new);

    let mut search_results = use_action(|query: String| async move {
        #[cfg(debug_assertions)]
        let perf = window().unwrap().performance().unwrap();
        #[cfg(debug_assertions)]
        let start = perf.now();

        let mut results: Vec<_> = RECORDS
            .iter()
            .filter_map(|r| {
                MATCHER
                    .fuzzy_match(&r.values, &query)
                    .map(|score| (score, r))
            })
            .collect();

        results.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by score
        results.truncate(MATCH_LIMIT);

        #[cfg(debug_assertions)]
        info!("took {:.3} ms", perf.now() - start);

        dioxus::Ok(results.into_iter().map(|(_, r)| r.clone()).collect())
    });

    let mut search_debounce = use_debounce(Duration::from_millis(100), move |query: String| {
        search_results.call(query);
    });

    let selected_ids = use_signal(Vec::<u32>::new);

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
                r#type: "text",
                placeholder: "Sök...",
                oninput: move |e| {
                    let query = e.value();
                    search_debounce.action(query.clone());
                    search_text.set(query);
                }
            }

            SelectionsContainer {
                selected_ids,
                generated_url
            }

            SearchResults {
                search_text,
                search_results_value: search_results.value(),
                selected_ids,
            }

        }

        footer {
            "Skapad av "
            a {
                href: "https://github.com/sermuns/timeedit-tongs",
                "Samuel \"sermuns\" Åkesson"
            }
        }
    }
}
