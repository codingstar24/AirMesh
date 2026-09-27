//! Advanced search query parsing and result ranking.

use crate::index::types::{SearchQuery, SearchResult};

/// Search query parser and filter evaluator.
pub struct SearchEngine;

impl SearchEngine {
    /// Sanitizes and validates a raw search query string.
    pub fn sanitize_query(raw: &str) -> String {
        raw.chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '.' || *c == '_' || *c == '-')
            .collect::<String>()
            .trim()
            .to_string()
    }

    /// Filters and ranks results based on extension and query match.
    pub fn rank_results(mut hits: Vec<SearchResult>, query: &SearchQuery) -> Vec<SearchResult> {
        if let Some(mime) = &query.mime_filter {
            hits.retain(|h| h.mime_type.starts_with(mime));
        }

        if let Some(min_sz) = query.min_size {
            hits.retain(|h| h.size >= min_sz);
        }

        if let Some(max_sz) = query.max_size {
            hits.retain(|h| h.size <= max_sz);
        }

        hits.sort_by(|a, b| b.score.cmp(&a.score));
        if hits.len() > query.limit {
            hits.truncate(query.limit);
        }
        hits
    }
}
