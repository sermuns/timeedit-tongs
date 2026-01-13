use std::mem::discriminant;

use dioxus::prelude::*;

mod constants;
mod pages;

use crate::constants::*;
use crate::pages::Home;
use crate::pages::Ics;
use crate::pages::Un;

fn main() {
    dioxus::launch(|| {
        rsx! { Router::<Route> {} }
    });
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(BaseLayout)]

        #[route("/")]
        Home,

        #[route("/ics?:objects")]
        Ics { objects: String },

        #[route("/un?:object")]
        Un { object: Option<ObjectId> },

        #[route("/:..route")]
        NotFound {
            route: Vec<String>,
        },
}

#[component]
fn BaseLayout() -> Element {
    rsx! {
        document::Link { rel: "icon", href: LOGO }
        Stylesheet { href: asset!("/assets/style.css") }

        header {
            h1 {
                img { src: LOGO, height: 50 }
                { PKG_NAME }
            }
            nav {
                NavBarLink { to: Route::Home }
                NavBarLink { to: Route::Ics { objects: String::new() } }
                NavBarLink { to: Route::Un { object: None } }
            }
        }

        main { Outlet::<Route> {} }

        footer {
            span {
                "Skapad av "
                a {
                    href: "https://github.com/sermuns/timeedit-tongs",
                    target: "_blank",
                    "Samuel \"sermuns\" Åkesson"
                }
            }
            span {
                id: "info",
                "v"
                { env!("CARGO_PKG_VERSION") }
                " | "
                { env!("VERGEN_BUILD_TIMESTAMP") }
            }
        }
    }
}

#[component]
fn NavBarLink(to: Route) -> Element {
    let current_route = use_route::<Route>();

    // WARNING:  FUCKING HACKKK!! this might break shit with nested routes?
    let class = if discriminant(&current_route) == discriminant(&to) {
        "active"
    } else {
        ""
    };

    let text = match to {
        Route::Home => HOME_ROUTE_STR,
        Route::Ics { .. } => ICS_ROUTE_STR,
        Route::Un { .. } => UN_ROUTE_STR,
        _ => "",
    };

    rsx! {
        Link { to, class, {text} }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Title { "Sidan hittades inte | {PKG_NAME}" }

        h1 { "404 - Sidan hittades inte" }
        p { "'/{route:?}' kunde inte hittas." }
        Link {
            to: Route::Home,
            "Tillbaka till startsidan"
        }
    }
}
