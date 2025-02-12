//! Import commonly use types and traits.
//!
//! Re-export types that you can use by just importing it.
//! # Example
//!
//! Import the prelude with:
//! ```
//! use magpie_engine::prelude::*;
//! ```

pub use crate::{
    ext::{aug::*, desc::*},
    query::{FilterFn, Filters, QueryBuilder, QueryOrder, ToFilter},
    *,
};

#[cfg(feature = "fetch")]
pub use crate::fetch::{
    fetch_aug_set, fetch_cti_set, fetch_desc_set, fetch_imf_set, AugBranch, SetError,
};
