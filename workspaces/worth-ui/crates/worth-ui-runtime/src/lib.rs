mod admission;
mod capability;
mod declaration;
mod evidence;
pub mod facade;
mod graph;
pub(crate) mod host;
mod lifecycle;
mod obligations;
mod runtime;
mod source;

// Support fixtures: crate-private by default. Public only under feature so product crates cannot
// bind `worth_ui_runtime::certification_support` without opting into support authority.
#[cfg(feature = "certification-support")]
#[doc(hidden)]
pub mod certification_support;

#[cfg(not(feature = "certification-support"))]
pub(crate) mod certification_support;

