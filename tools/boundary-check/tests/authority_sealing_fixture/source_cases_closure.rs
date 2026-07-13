//! Closure-surface specimens: external module/type ownership, impl macros, attrs.

/// Entry: re-export an external public module that owns a generic ceremony.
pub fn entry_reexport_external_module() -> &'static str {
    r#"
pub use worth_schema_external::api;
"#
}

/// External: public module `api` with AuthorityMarker ceremony.
pub fn external_hostile_module_admit() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub mod api {
    use super::AuthorityMarker;

    pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
}
"#
}

/// Entry: re-export an external type whose inherent method is a generic ceremony.
pub fn entry_reexport_external_type() -> &'static str {
    r#"
pub use worth_schema_external::Ceremony;
"#
}

/// External: public type with inherent AuthorityMarker method.
pub fn external_hostile_type_method_admit() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Ceremony;

impl Ceremony {
    pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
}
"#
}

/// Hostile: macro invocation inside impl of a public type mints a ceremony.
pub fn hostile_impl_macro_member() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Ceremony;

macro_rules! open_method {
    () => {
        pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
    };
}

impl Ceremony {
    open_method!();
}
"#
}

/// Hostile: opaque attribute on a reachable impl block.
pub fn hostile_opaque_attr_on_impl() -> &'static str {
    r#"
pub struct Ceremony;

#[ceremony_factory]
impl Ceremony {}
"#
}

/// Hostile: opaque attribute on a public inherent method.
pub fn hostile_opaque_attr_on_impl_method() -> &'static str {
    r#"
pub struct Ceremony;

impl Ceremony {
    #[ceremony_factory]
    pub fn admit() {}
}
"#
}

/// Hostile: opaque attribute on a public trait member.
pub fn hostile_opaque_attr_on_trait_member() -> &'static str {
    r#"
pub trait Gate {
    #[ceremony_factory]
    fn admit();
}
"#
}

/// External: nested re-export chain ending in a generic ceremony.
pub fn external_hostile_nested_reexport_chain() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

mod outer {
    mod secret {
        use super::super::AuthorityMarker;

        pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
    }

    pub use secret::*;
}

pub use outer::*;
"#
}

/// Hostile: exported macro templates a trait-bound fragment into a public ceremony.
pub fn hostile_macro_export_trait_bound_template() -> &'static str {
    r#"
#[macro_export]
macro_rules! open_ceremony {
    ($bound:path) => {
        pub fn admit(_authority: impl $bound) {}
    };
}

pub fn seed() {}
"#
}

/// Legal: concrete external module re-export (no generic authority).
pub fn external_legal_module_admit() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}

pub mod api {
    use super::{AuthorityWitnessPlaceholder, EntryAdmission};

    pub fn admit(_authority: AuthorityWitnessPlaceholder<EntryAdmission>) {}
}
"#
}

/// Legal: concrete external type method re-export.
pub fn external_legal_type_method_admit() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}

pub struct Ceremony;

impl Ceremony {
    pub fn admit(_authority: AuthorityWitnessPlaceholder<EntryAdmission>) {}
}
"#
}

/// Hostile: public foreign fn carries open AuthorityMarker bound.
pub fn hostile_foreign_fn_authority_marker() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

extern "Rust" {
    pub fn admit<Auth: AuthorityMarker>(authority: Auth);
}
"#
}

/// Hostile: public foreign static carries open CapabilityMarker carrier type.
pub fn hostile_foreign_static_capability_marker() -> &'static str {
    r#"
pub trait CapabilityMarker: 'static {}

extern "Rust" {
    pub static GATE: fn(&dyn CapabilityMarker);
}
"#
}

/// Hostile: opaque attribute on a public foreign fn is an uninspectable ceremony site.
pub fn hostile_opaque_attr_on_foreign_fn() -> &'static str {
    r#"
extern "Rust" {
    #[ceremony_factory]
    pub fn admit();
}
"#
}

/// Legal: public foreign fn with concrete authority witness (no open marker bound).
pub fn legal_foreign_fn_concrete_authority() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}

extern "Rust" {
    pub fn admit(authority: AuthorityWitnessPlaceholder<EntryAdmission>);
}
"#
}
