use chrono::Utc;
use dioxus::CapturedError;
use dioxus::prelude::*;
use dioxus_sdk::time::use_debounce;
use fxhash::FxHashMap;
use std::time::Duration;

use types::{CalendarResponse, Reservation};

use crate::Route;
use crate::constants::*;
use crate::search::fuzzy_search_object_records;

#[component]
pub fn Un(object: ReadSignal<Option<ObjectId>>) -> Element {
    let mut search_text = use_signal(String::new);

    let mut search_results =
        use_action(|query: String| async move { fuzzy_search_object_records(&query) });

    let mut search_debounce = use_debounce(Duration::from_millis(100), move |query| {
        search_results.call(query);
    });

    let object_name = use_memo(move || {
        if let Some(object_id) = object() {
            OBJECT_RECORDS
                .iter()
                .find(|r| r.id == object_id)
                .map(|o| &o.values)
        } else {
            None
        }
    });

    rsx! {
        document::Title { "{UN_ROUTE_STR} | {PKG_NAME}" }

        if let Some(object_id) = object()  {
            if let Some(object_name) = object_name() {
                h2 { "{object_name}" }
                ObjectSummary { object }
            } else {
                div {
                    class: "box error",
                    "Objekt med ID {object_id} finns inte i {PKG_NAME}:s databas!"
                }
            }
        } else {
            div {
                class: "box",
                "Sök efter kurs/studentgrupp och välj för att få sammanfattning över passerade moment i kursen."
            }
        }

        div {
            id: "search-input-container",
            span {"🔍"}
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
        }

    }
}

#[component]
fn ObjectSummary(object: ReadSignal<Option<ObjectId>>) -> Element {
    #[derive(Debug)]
    struct ActivityOccurences {
        past_occurrences: u32,
        total_occurrences: u32,
        next_occurence: Option<Reservation>,
    }

    let activity_occurences = use_resource(move || async move {
        let calendar_response: CalendarResponse = reqwest::get(format!(
            "https://cloud.timeedit.net/liu/web/schema/ri.json?sid=3&p=20250101,20270101&objects={}",
            object().unwrap(),
        ))
        .await.with_context(|| "Fel vid hämtning av data från TimeEdit för objekt med ID {object_id}")?.json().await.with_context(|| "Fel vid tolkning av hämtad data från TimeEdit för objekt med ID {object_id}")?;

        let mut activity_occurences_map: FxHashMap<String, ActivityOccurences> =
            FxHashMap::default();

        let utc_now = Utc::now();
        for reservation in calendar_response.reservations {
            let activity_type_string = reservation.teaching_activity.clone();
            if activity_type_string.is_empty() {
                continue; // NOTE: might be stupid to just skip?
            }
            let is_in_the_past = reservation.end_utc() < utc_now;

            let entry = activity_occurences_map
                .entry(activity_type_string)
                .or_insert(ActivityOccurences {
                    past_occurrences: 0,
                    total_occurrences: 0,
                    next_occurence: None,
                });
            entry.total_occurrences += 1;

            if is_in_the_past {
                entry.past_occurrences += 1;
            } else if entry.next_occurence.is_none() {
                entry.next_occurence = Some(reservation);
            }
        }

        dioxus::Ok(activity_occurences_map)
    });

    if let Some(response) = &*activity_occurences.read() {
        match response {
            Ok(activity_occurences_map) => {
                let mut activity_occurences_sorted: Vec<(&String, &ActivityOccurences)> =
                    activity_occurences_map.iter().collect();
                activity_occurences_sorted.sort_by(|a, b| a.0.cmp(b.0));

                rsx! {
                    table {
                        thead {
                            tr {
                                th { "Typ" }
                                th { "Passerade / totalt" }
                                th { "Nästa tillfälle" }
                            }
                        }
                        tbody {
                            for (activity_type, occurences) in activity_occurences_sorted {
                                tr {
                                    td { "{activity_type}" }
                                    td { "{occurences.past_occurrences} / {occurences.total_occurrences}" }
                                    td {
                                        if let Some(occurence) = &occurences.next_occurence {
                                            a {
                                                href: occurence.link(),
                                                target: "_blank",
                                                display: "block",
                                                text_align: "right",
                                                {occurence.start_localized_format()}
                                            }
                                        } else {
                                            "Inga fler tillfällen"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => rsx! {
                span {
                    color: "red",
                    "fel: {err}",
                }
            },
        }
    } else {
        rsx! {
            div {
                "Laddar..."
            }
        }
    }
}

#[component]
fn SearchResults(
    search_text: Signal<String>,
    search_results_value: Option<Result<ReadSignal<Vec<usize>>, CapturedError>>,
) -> Element {
    if search_text().is_empty() {
        return rsx! { "" };
    }

    match search_results_value {
        Some(Ok(indices)) => {
            let records = indices.iter().flat_map(|idx| OBJECT_RECORDS.get(*idx));
            rsx! {
                table {
                    id: "search-results",
                    for record in records {
                        tr {
                            td {
                                Link {
                                    to: Route::Un {
                                        object: Some(record.id)
                                    },
                                    "{record.values}"
                                }
                            }
                        }
                    }
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
            span { "..." }
        },
    }
}
