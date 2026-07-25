//! Host-audience surface: exact Query runtime, installation, and ordinary
//! installed-operation namespaces.

/// Runtime assembly, workspace, and execution contracts.
pub use worth_query::facade::runtime;

/// Runtime-installed domain declaration and handle contracts.
pub use worth_query::facade::domain;

/// Ordinary installed-operation progression after runtime construction.
pub use worth_query::facade::installed;
