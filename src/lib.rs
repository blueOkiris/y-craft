//! Entry point for Y-Craft engine library

use std::hash::Hash;

pub mod res;
pub mod obj;
pub mod room;
pub mod app;
pub mod util;

/// Y-Craft generically uses hypothetical enums to index various things, and there are restrictions
/// on these enums for the user
pub trait IndexRestriction: Clone + Copy + Hash + Eq + 'static {}
impl<T: Clone + Copy + Hash + Eq + 'static> IndexRestriction for T {}

