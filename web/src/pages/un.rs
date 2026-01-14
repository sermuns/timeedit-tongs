use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::{NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;
use dioxus::CapturedError;
use dioxus::prelude::*;
use dioxus_sdk::time::use_debounce;
use fuzzy_matcher::FuzzyMatcher;
use fxhash::FxHashMap;
use serde::Deserialize;
use std::time::Duration;

use types::ObjectRecord;

use crate::Route;
use crate::constants::*;

#[derive(Debug, Deserialize)]
struct CalendarResponse {
    reservations: Vec<Reservation>,
}

#[derive(Debug, Deserialize)]
struct Reservation {
    id: String,
    startdate: NaiveDate,
    starttime: NaiveTime,
    enddate: NaiveDate,
    endtime: NaiveTime,
    columns: [String; 9],
}
impl Reservation {
    // TODO: https://crates.io/crates/chrono-tz#user-content-limiting-the-timezone-table-to-zones-of-interest

    // NOTE: hardcoded Stockholm timezone because i think TimeEdit API is in that??
    const TIME_ZONE: Tz = chrono_tz::Europe::Stockholm;

    fn get_start_utc(&self) -> DateTime<Utc> {
        let naive = NaiveDateTime::new(self.startdate, self.starttime);

        Self::TIME_ZONE
            .from_local_datetime(&naive)
            .unwrap()
            .with_timezone(&Utc)
    }

    fn get_end_utc(&self) -> DateTime<Utc> {
        let naive = NaiveDateTime::new(self.enddate, self.endtime);

        Self::TIME_ZONE
            .from_local_datetime(&naive)
            .unwrap()
            .with_timezone(&Utc)
    }
}

#[component]
pub fn Un(object: ReadSignal<Option<ObjectId>>) -> Element {
    let mut search_text = use_signal(String::new);

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
        results.truncate(MATCH_LIMIT);

        dioxus::Ok(results.into_iter().map(|(_, r)| r.clone()).collect())
    });

    let mut search_debounce = use_debounce(Duration::from_millis(100), move |query| {
        search_results.call(query);
    });

    let object_name = use_memo(move || {
        if let Some(object_id) = object() {
            RECORDS
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
                small { "ID: {object_id}" }
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
        }

    }
}

#[component]
fn ObjectSummary(object: ReadSignal<Option<ObjectId>>) -> Element {
    #[derive(Debug)]
    struct ActivityOccurences {
        past_occurrences: u32,
        total_occurrences: u32,
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
            let activity_type_string = reservation.columns[1].clone();
            if activity_type_string.is_empty() {
                continue; // NOTE: might be stupid to just skip?
            }
            let is_in_the_past = reservation.get_end_utc() < utc_now;

            let entry = activity_occurences_map
                .entry(activity_type_string)
                .or_insert(ActivityOccurences {
                    past_occurrences: 0,
                    total_occurrences: 0,
                });
            entry.total_occurrences += 1;

            if is_in_the_past {
                entry.past_occurrences += 1;
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
                            }
                        }
                        tbody {
                            for (activity_type, occurences) in activity_occurences_sorted {
                                tr {
                                    td { "{activity_type}" }
                                    td { "{occurences.past_occurrences} / {occurences.total_occurrences}" }
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
    search_results_value: Option<Result<ReadSignal<Vec<ObjectRecord>>, CapturedError>>,
) -> Element {
    if search_text().is_empty() {
        return rsx! { "" };
    }

    match search_results_value {
        Some(Ok(records)) => {
            rsx! {
                table {
                    id: "search-results",
                    for record in records() {
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
