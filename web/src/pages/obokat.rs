use dioxus::prelude::*;
use futures::StreamExt;
use fuzzy_matcher::FuzzyMatcher;
use fxhash::{FxHashMap, FxHashSet};
use std::sync::LazyLock;

use types::{CalendarResponse, ObjectRecord, ObjectType};

use crate::constants::*;

static ROOM_OBJECTS: LazyLock<Vec<ObjectRecord>> = LazyLock::new(|| {
    OBJECT_RECORDS
        .iter()
        .filter(|o| o.r#type == ObjectType::Room)
        .cloned()
        .collect::<Vec<_>>()
});

static ALL_ROOM_OBJECT_IDS_STRING: LazyLock<String> = LazyLock::new(|| {
    let mut s = String::with_capacity(ROOM_OBJECTS.len() * (6 + 1)); // id + comma
    for (i, o) in ROOM_OBJECTS.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str(&o.id.to_string());
    }
    s
});

#[component]
pub fn Obokat() -> Element {
    let mut unbooked_room_indices = use_resource(move || async move {
        info!("fetching...");
        let url = format!(
            "https://cloud.timeedit.net/liu/web/schema/r.json?sid=3&p=0.m,1.m&objects={}",
            *ALL_ROOM_OBJECT_IDS_STRING,
        );

        let responses: CalendarResponse = reqwest::get(url).await?.json().await?;
        info!("fetch done!");

        let reservations = responses.reservations;

        info!("matching..");

        // NOTE: maybe completely needless optimizaton... fucking overengineered piece of shit
        let mut reservation_room_to_room_objs_idx: FxHashMap<String, usize> = FxHashMap::default();

        let booked_room_indices: FxHashSet<usize> = reservations
            .into_iter()
            .filter_map(|r| {
                if let Some(&stored_idx) = reservation_room_to_room_objs_idx.get(&r.room) {
                    return Some(stored_idx);
                }

                let new_idx = ROOM_OBJECTS
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, room_object)| {
                        MATCHER
                            .fuzzy_match(&room_object.values, &r.room)
                            .map(|score| (idx, score))
                    })
                    .max_by_key(|(_, score)| *score)
                    .map(|(idx, _)| idx);

                if let Some(idx) = new_idx {
                    reservation_room_to_room_objs_idx.insert(r.room, idx);
                }

                new_idx
            })
            .collect();

        let unbooked_room_indices: Vec<usize> = (0..ROOM_OBJECTS.len())
            .filter(|idx| !booked_room_indices.contains(idx))
            .collect();
        info!("match done!");

        dioxus::Ok(unbooked_room_indices)
    });

    rsx! {
        document::Title { "{OBOKAT_ROUTE_STR} | {PKG_NAME}" }
        div {
            class: "box",
            "🚧 WORK-IN-PROGRESS: detta funkar nog inte, ännu... 🚧"
        }
        match &*unbooked_room_indices.read() {
            Some(Ok(indices)) => rsx!{
                div {
                    id: "obokat-result",
                    for room in indices.iter().map(|idx| &ROOM_OBJECTS[*idx]) {
                        span { "{room.values}" }
                    }
                }
            },
            Some(Err(err)) => rsx!{
                div {
                    class: "box error",
                    "Fel {err:?}"
                }
            },
            None => rsx!{
                div {
                    class: "box",
                    "Laddar.."
                }
            }
        }
    }
}
