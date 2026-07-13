//! Public consumption flows through [`facade`] only.
//!
//! ```rust
//! use worth_pack_registry::facade::{
//!     ContributionDescriptor,
//!     ContributionKind,
//!     PackName,
//!     PackRegistration,
//! };
//!
//! let descriptor = ContributionDescriptor::new(
//!     ContributionKind::Component,
//!     PackName::new("worth-pack-wall-basic").unwrap(),
//! );
//! let registration = PackRegistration::new(descriptor);
//!
//! assert_eq!(registration.contribution_kind(), ContributionKind::Component);
//! assert_eq!(registration.pack_name().as_str(), "worth-pack-wall-basic");
//! ```
//!
//! ```compile_fail
//! use worth_pack_registry::registration::PackRegistration;
//! ```

pub mod facade;

mod contribution_descriptor;
mod contribution_kind;
mod pack_name;
mod registration;
