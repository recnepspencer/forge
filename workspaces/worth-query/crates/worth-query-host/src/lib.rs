//! Query host audience facade.
//!
//! Admission, lowering, execution, and publication consumers depend on this
//! crate instead of importing Query's internal authority packages directly.
//!
//! ```
//! use worth_query_host::facade::{admission, domain, runtime};
//! # fn _host_surface(
//! #     installer: runtime::WorthQueryExecutionRuntimeInstaller,
//! #     package: domain::WorthQueryPortableDomainPackage,
//! # ) {
//! #     let _ = (
//! #         installer,
//! #         package,
//! #         std::any::TypeId::of::<admission::resource_admission::WorthQueryExecutionResourceAdmissionDenial>(),
//! #     );
//! # }
//! ```

pub mod facade;
