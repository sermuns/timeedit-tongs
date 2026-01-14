use dioxus::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::sync::LazyLock;

#[cfg(debug_assertions)]
pub use web_sys::window;

use types::ObjectRecord;

pub type ObjectId = u32;

pub static OBJECT_RECORDS: LazyLock<Vec<ObjectRecord>> = LazyLock::new(|| {
    wincode::deserialize::<Vec<ObjectRecord>>(include_bytes!("../assets/records.bin")).unwrap()
});
pub const MATCH_LIMIT: usize = 20;
pub static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(|| {
    SkimMatcherV2::default()
        .element_limit(MATCH_LIMIT)
        .ignore_case()
});
pub const LOGO_ICO: Asset = asset!(
    "/assets/favicon.ico",
    AssetOptions::builder().with_hash_suffix(false)
);
pub const LOGO_SVG: Asset = asset!("/assets/logo.svg");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub const HOME_ROUTE_STR: &str = "Hem";
pub const ICS_ROUTE_STR: &str = "Smörgåsbord";
pub const UN_ROUTE_STR: &str = "Undervisningsnummer";
pub const OBOKAT_ROUTE_STR: &str = "Obokat";
