//! Query host audience facade.
//!
//! Admission, lowering, execution, and publication consumers depend on this
//! crate instead of importing Query's internal authority packages directly.
//! The read-only [`facade::domain`] surface exposes an installed schema's
//! `native_contracts()`, an installed operation's typed `graph_reads()` and
//! `touches()`, and complete aftermath inspection through `authority()`,
//! `recovery()`, `reconciliation()`, and
//! `external_effect().correlation_family()`. These accessors inspect retained
//! meaning; they do not grant installation, execution, correction, recovery,
//! or external-effect authority.
//!
//! ```
//! use worth_query_host::facade::domain::{
//!     WorthQueryInstalledApplicationOperation, WorthQueryOperationGraphReadScope,
//! };
//!
//! fn inspect<Schema, Operation, Input>(
//!     operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
//! ) {
//!     for role in operation.contracts().graph_reads().roles() {
//!         for scope in role.read_scopes() {
//!             if let WorthQueryOperationGraphReadScope::NativeProjection(scope) = scope {
//!                 let _ = (
//!                     scope.entity().semantic_key(),
//!                     scope.aspect().as_str(),
//!                     scope.projection().mask(),
//!                 );
//!             }
//!         }
//!     }
//!
//!     if let Some(aftermath) = operation.contracts().aftermath() {
//!         let _ = (
//!             aftermath.authority(),
//!             aftermath.recovery(),
//!             aftermath.reconciliation(),
//!             aftermath.external_effect().correlation_family(),
//!         );
//!     }
//! }
//! ```
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
