use crate::constants::*;
use dioxus::prelude::*;
use fuzzy_matcher::FuzzyMatcher;

/// returns idx into `OBJECT_RECORDS`
pub fn fuzzy_search_object_records(query: &str) -> Result<Vec<usize>> {
    #[cfg(debug_assertions)]
    let perf = window().unwrap().performance().unwrap();
    #[cfg(debug_assertions)]
    let start = perf.now();

    let mut indices: Vec<_> = OBJECT_RECORDS
        .iter()
        .enumerate()
        .filter_map(|(idx, r)| {
            MATCHER
                .fuzzy_match(&r.values, query)
                .map(|score| (score, idx))
        })
        .collect();

    indices.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // sort by score
    indices.truncate(MATCH_LIMIT);

    #[cfg(debug_assertions)]
    info!("took {:.0} ms", perf.now() - start);

    dioxus::Ok(
        indices
            .into_iter()
            .map(|(_, idx)| idx)
            .collect::<Vec<usize>>(),
    )
}
