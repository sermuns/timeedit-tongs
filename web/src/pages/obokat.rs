use dioxus::prelude::*;

use crate::constants::*;

use types::{CalendarResponse, Reservation};

#[component]
pub fn Obokat() -> Element {

    let free_rooms = use_resource(move || async move {
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


    rsx! {
        document::Title { "{OBOKAT_ROUTE_STR} | {PKG_NAME}" }
    }
}
