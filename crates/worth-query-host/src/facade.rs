//! Host-audience surface: exact re-exports from the Query engine.

/// Application-facing Query host contract for admission, lowering, and execution.
///
/// ```
/// use worth_query_host::facade::WorthQueryApplicationFacade;
/// # fn _enter(host: &WorthQueryApplicationFacade) {
/// #     let _ = host;
/// # }
/// ```
pub use worth_query::facade::WorthQueryApplicationFacade;
