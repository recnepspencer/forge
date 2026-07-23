//! Query host audience facade.
//!
//! Admission, lowering, and execution consumers depend on this crate instead
//! of importing Query internals. The host surface preserves Query's runtime and
//! installed-domain namespaces without adding another executable authority.
//!
//! ```
//! use worth_query_host::facade::{domain, installed, runtime};
//! # fn _host_surface(
//! #     builder: runtime::WorthQueryRuntimeBuilder,
//! #     package: domain::WorthQueryDomainPackage<impl domain::WorthQueryDomainEntryMarker>,
//! # ) {
//! #     let _ = (
//! #         builder,
//! #         package,
//! #         std::any::TypeId::of::<installed::operation::WorthQueryOperationResultState>(),
//! #     );
//! # }
//! ```

pub mod facade;
