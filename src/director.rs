
//! predicate controllor

use crate::{api::ImageCache, transact::TransAct};

/// operation director using predicates
#[derive(Debug)]
pub struct PredicateDirector {
    cache: ImageCache
}

impl PredicateDirector {
    /// construct a new PredicateDirector and load its cache
    pub fn new() -> Self {
        Self { cache: ImageCache::load() }
    }

    /// check if a given predicate passes
    pub fn test(&self, _tcode: TransAct, _name: String) -> bool {


        false
    }
}
