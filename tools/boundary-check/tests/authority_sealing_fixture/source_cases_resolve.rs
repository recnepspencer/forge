//! Specimens for definition-resolved authority (path-dep renames + non-bare where).

/// Path dependency: renames AuthorityMarker to Gate as a public export.
pub fn dep_renames_authority_marker_to_gate() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub use AuthorityMarker as Gate;
"#
}

/// Entry: private use of dependency-renamed Gate on a public ceremony.
pub fn entry_private_use_dep_gate() -> &'static str {
    r#"
use worth_schema_authgate::Gate;

pub fn governed_ceremony<T: Gate>(_authority: T) {}
"#
}

/// Entry: qualified path to dependency-renamed Gate (no local use).
pub fn entry_qualified_dep_gate() -> &'static str {
    r#"
pub fn governed_ceremony<T: worth_schema_authgate::Gate>(_authority: T) {}
"#
}

/// Hostile: non-bare where predicate launders via Wrapper<T>: AuthorityMarker.
pub fn hostile_nonbare_where_wrapper_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Wrapper<T>(pub T);

impl<T: AuthorityMarker> AuthorityMarker for Wrapper<T> {}

trait GovernedGate {}

impl<T> GovernedGate for Wrapper<T> where Wrapper<T>: AuthorityMarker {}

pub fn governed_ceremony<G: GovernedGate>(_authority: G) {}
"#
}

/// Hostile: non-bare where on bare Self param (`where Wrapper<T>: Auth`).
pub fn hostile_nonbare_where_on_param_self() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Wrapper<T>(pub T);

impl<T: AuthorityMarker> AuthorityMarker for Wrapper<T> {}

trait GovernedGate {}

impl<T> GovernedGate for T where Wrapper<T>: AuthorityMarker {}

pub fn governed_ceremony<G: GovernedGate>(_authority: G) {}
"#
}

/// Hostile: associated-type projection predicate carries forbidden dependence.
pub fn hostile_assoc_projection_where_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub trait HasAuthority {
    type Authority;
}

impl<T: AuthorityMarker> HasAuthority for T {
    type Authority = T;
}

trait GovernedGate {}

impl<T> GovernedGate for T where <T as HasAuthority>::Authority: AuthorityMarker {}

pub fn governed_ceremony<G: GovernedGate>(_authority: G) {}
"#
}

/// Hostile: higher-ranked / reference non-bare predicate form.
pub fn hostile_href_where_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T> GovernedGate for T where for<'a> &'a T: AuthorityMarker {}

pub fn governed_ceremony<G: GovernedGate>(_authority: G) {}
"#
}

/// Legal: path dep exports a non-authority trait only.
pub fn dep_non_authority_export() -> &'static str {
    r#"
pub trait Describe {}
pub fn seed() {}
"#
}

/// Legal entry using non-authority dep trait.
pub fn entry_legal_non_authority_dep() -> &'static str {
    r#"
use worth_schema_authgate::Describe;

pub fn describe<T: Describe>(_value: T) {}
"#
}

/// Legal concrete ceremony control.
pub fn legal_concrete_resolve_control() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}

pub fn admit(_authority: AuthorityWitnessPlaceholder<EntryAdmission>) {}
"#
}
