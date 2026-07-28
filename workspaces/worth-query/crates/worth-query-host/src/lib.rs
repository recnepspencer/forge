//! Query host audience facade.
//!
//! Admission, lowering, execution, and publication consumers depend on this
//! crate instead of importing Query's internal authority packages directly.
//!
//! ```
//! use worth_query_host::facade::{admission, domain, primary_graph, runtime};
//! # fn _host_surface(
//! #     installer: runtime::WorthQueryExecutionRuntimeInstaller,
//! #     package: domain::WorthQueryPortableDomainPackage,
//! # ) {
//! #     let _ = (
//! #         installer,
//! #         package,
//! #         std::any::TypeId::of::<primary_graph::WorthQueryPrincipalResolutionMode>(),
//! #         std::any::TypeId::of::<admission::resource_admission::WorthQueryExecutionResourceAdmissionDenial>(),
//! #     );
//! # }
//! ```
//!
//! Raw primary-graph integration is not an audience capability:
//!
//! ```compile_fail
//! use worth_query_host::facade::runtime::WorthQueryExecutionRuntime;
//!
//! fn cannot_extract_relational_graph(runtime: &WorthQueryExecutionRuntime) {
//!     let _ = runtime.retain_primary_graph_integration_handle();
//! }
//! ```
//!
//! ```compile_fail
//! use worth_query_host::facade::domain::ApplicationSchema;
//! use worth_query_host::facade::primary_graph::WorthQueryPrimaryGraphBootstrap;
//! use worth_query_host::facade::runtime::{
//!     WorthQueryExecutionInstallationAuthority, WorthQueryExecutionRuntime,
//! };
//!
//! fn cannot_publish_broad_runtime<Schema: ApplicationSchema>(
//!     graph: WorthQueryPrimaryGraphBootstrap<Schema>,
//!     runtime: &mut WorthQueryExecutionRuntime,
//!     authority: &WorthQueryExecutionInstallationAuthority,
//! ) {
//!     graph.publish(runtime, authority).unwrap();
//! }
//! ```

pub mod facade;
