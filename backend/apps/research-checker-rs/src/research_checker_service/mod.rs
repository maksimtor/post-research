pub mod proto;
mod service;

use proto::RetractedArticle;
pub use service::ResearchCheckerService;

use crate::db::Retraction;

impl From<Retraction> for RetractedArticle {
    fn from(value: Retraction) -> Self {
        Self {
            name: value.title.unwrap_or_default(),
            doi: value.doi.unwrap_or_default(),
            pubmed_id: value.pubmed_id,
        }
    }
}
