use dioxus::prelude::*;

use crate::{Route, constants::*};

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Hem | {PKG_NAME}" }
        p {
            "Und(vik)erlätta användning av TimeEdit som student på Linköpings universitet."
        }

        Link {
            to: Route::Ics { objects: String::new() },
            class: "block-link",
            h3 { "🗓️" {ICS_ROUTE_STR} }
            p { "Skapa kalenderprenumerationslänkar (.ics) genom att plocka ihop kurser och/eller studentgrupper." }
        }

        Link {
            to: Route::Un { object: None },
            class: "block-link",
            h3 { "🔢 " {UN_ROUTE_STR} }
            p { "Räkna antal föreläsningar, lektioner, etc. som har passerat i en kurs." }
        }

        h2 { "Om denna sida" }

        p {
            "Skapades i frustration av att behöva klicka i så många djupa menyer och interaktivt SKRÄP på TimeEdit för att välja min personliga kalender - speciellt nu med alla valbara kurser på masternivå. 😤"
        }

        p {
            "Tanken är att "
            i {"så mycket som möjligt"}
            " ska beräknas lokalt i webbläsaren."
        }

        p {
            "{PKG_NAME} är ett öppen-källkod projekt. Bidra gärna med funktionalitet eller ta kontakt vid buggar/fel! "
            br {}
            a { href: PKG_REPOSITORY, target: "_blank", {PKG_REPOSITORY} }
        }
    }
}
