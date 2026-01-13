use dioxus::CapturedError;
use dioxus::prelude::*;
use dioxus_sdk::time::use_debounce;
use fuzzy_matcher::FuzzyMatcher;
use std::fmt::Write;
use std::time::Duration;
#[cfg(debug_assertions)]
use web_sys::window;

use types::ObjectRecord;

use crate::Route;
use crate::constants::*;

#[component]
pub fn Ics(objects: String) -> Element {
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
        info!("took {:.0} ms", perf.now() - start);

        dioxus::Ok(results.into_iter().map(|(_, r)| r.clone()).collect())
    });

    let mut search_debounce = use_debounce(Duration::from_millis(100), move |query: String| {
        search_results.call(query);
    });

    let selected_ids: Signal<Vec<ObjectId>> =
        use_signal(|| objects.split(',').filter_map(|s| s.parse().ok()).collect());

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

    use_effect(move || {
        let objects = selected_ids()
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        navigator().replace(Route::Ics { objects });
    });

    rsx! {
        document::Title { "Prenumerationslänk | {PKG_NAME}" }

        SelectionsContainer {
            selected_ids,
            generated_url
        }

        div {
            id: "search-input-container",
            span {"🔎"}
            input {
                r#type: "text",
                placeholder: "Sök...",
                oninput: move |e| {
                    let query = e.value();
                    search_debounce.action(query.clone());
                    search_text.set(query);
                }
            }
        }

        SearchResults {
            search_text,
            search_results_value: search_results.value(),
            selected_ids,
        }

    }
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
                    id: "search-results",
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
    if selected_ids.is_empty() {
        return rsx! {
            div {
                class: "box",
                "Sök efter kurser och/eller studentgrupper i sökrutan. Kryssa sedan i vad som ska ingå i kalenderprenumerationen. När du är nöjd, kopiera länken och importera till valfri kalenderapp."
            }
        };
    }
    rsx! {
        div {
            id: "selections",
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
