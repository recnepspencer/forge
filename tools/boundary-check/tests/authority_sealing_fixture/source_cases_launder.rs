//! Specimens for semantic authority-bound laundering (supertrait + blanket impl).

/// Hostile: private subtrait of AuthorityMarker on a public ceremony.
pub fn hostile_private_subtrait_bound() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait LocalGate: AuthorityMarker {}

pub fn admit<Auth: LocalGate>(_authority: Auth) {}
"#
}

/// Hostile: public subtrait of AuthorityMarker on a public ceremony.
pub fn hostile_public_subtrait_bound() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub trait LocalGate: AuthorityMarker {}

pub fn admit<Auth: LocalGate>(_authority: Auth) {}
"#
}

/// Hostile: multi-hop supertrait chain into AuthorityMarker.
pub fn hostile_multihop_supertrait_bound() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait Mid: AuthorityMarker {}
trait Outer: Mid {}

pub fn admit<Auth: Outer>(_authority: Auth) {}
"#
}

/// Hostile: blanket impl launders AuthorityMarker through a private gate trait.
pub fn hostile_blanket_impl_private_gate() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for T {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: blanket impl launders through a public gate trait.
pub fn hostile_blanket_impl_public_gate() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for T {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: where-clause blanket laundering form.
pub fn hostile_blanket_impl_where_clause() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T> GovernedGate for T where T: AuthorityMarker {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: renamed marker import used in blanket laundering.
pub fn hostile_blanket_impl_renamed_marker() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

use AuthorityMarker as AuthMark;

trait GovernedGate {}

impl<T: AuthMark> GovernedGate for T {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: multi-hop blanket laundering A → B → ceremony.
pub fn hostile_multihop_blanket_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait A {}
impl<T: AuthorityMarker> A for T {}

trait B {}
impl<T: A> B for T {}

pub fn governed_ceremony<T: B>(_authority: T) {}
"#
}

/// Hostile: where-clause form on the public ceremony itself.
pub fn hostile_where_clause_laundered_gate() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}
impl<T: AuthorityMarker> GovernedGate for T {}

pub fn governed_ceremony<T>(_authority: T)
where
    T: GovernedGate,
{
}
"#
}

/// Legal: ordinary non-authority trait bound stays ungoverned by sealing.
pub fn legal_non_authority_trait_bound() -> &'static str {
    r#"
pub fn describe<T: core::fmt::Debug>(_value: T) {}
"#
}

/// Legal: concrete ceremony is not a generic authority laundering surface.
pub fn legal_concrete_not_laundered() -> &'static str {
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

/// Hostile: tuple carrier blanket laundering.
pub fn hostile_tuple_carrier_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for (T,) {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: array carrier blanket laundering.
pub fn hostile_array_carrier_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for [T; 1] {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: reference carrier blanket laundering.
pub fn hostile_ref_carrier_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for &T {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: wrapper carrier blanket laundering.
pub fn hostile_wrapper_carrier_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Wrapper<T>(pub T);

trait GovernedGate {}

impl<T: AuthorityMarker> GovernedGate for Wrapper<T> {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: multi-parameter wrapper still carries forbidden param.
pub fn hostile_multiparam_wrapper_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Pair<A, B>(pub A, pub B);

trait GovernedGate {}

impl<T: AuthorityMarker, U> GovernedGate for Pair<T, U> {}

pub fn governed_ceremony<T: GovernedGate>(_authority: T) {}
"#
}

/// Hostile: path-qualified alias from nested module.
pub fn hostile_qualified_alias_launder() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

mod aliases {
    pub use super::AuthorityMarker as Gate;
}

pub fn governed_ceremony<T: aliases::Gate>(_authority: T) {}
"#
}
