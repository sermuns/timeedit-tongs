use dioxus::prelude::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::sync::LazyLock;

use types::ObjectRecord;

pub type ObjectId = u32;

pub static RECORDS: LazyLock<Vec<ObjectRecord>> = LazyLock::new(|| {
    wincode::deserialize::<Vec<ObjectRecord>>(include_bytes!("../assets/records.bin")).unwrap()
});
pub const MATCH_LIMIT: usize = 20;
pub static MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(|| {
    SkimMatcherV2::default()
        .element_limit(MATCH_LIMIT)
        .ignore_case()
});
pub const LOGO: Asset = asset!("/assets/logo.svg");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub const HOME_ROUTE_STR: &str = "Hem";
pub const ICS_ROUTE_STR: &str = "Smörgåsbord";
pub const UN_ROUTE_STR: &str = "Undervisningsnummer";
