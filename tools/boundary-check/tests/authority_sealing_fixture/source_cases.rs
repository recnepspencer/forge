//! Focused legal and hostile Rust source specimens for authority sealing.

/// Legal concrete ceremony signature (direct spelling).
pub fn legal_concrete_direct() -> &'static str {
    r#"
pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AdmissionEligible;

pub fn admit(
    _authority: AuthorityWitnessPlaceholder<EntryAdmission>,
) {
}

// Stand-in so the specimen parses without worth-proof; sealing inspects bounds only.
pub struct AuthorityWitnessPlaceholder<A> {
    _marker: core::marker::PhantomData<A>,
}
"#
}

/// Legal concrete signature re-exported under a same-crate alias.
pub fn legal_concrete_reexport() -> &'static str {
    r#"
mod inner {
    pub struct EntryAdmission {
        _value_gate: (),
    }

    pub struct AuthorityWitnessPlaceholder<A> {
        _marker: core::marker::PhantomData<A>,
    }

    pub fn admit(
        _authority: AuthorityWitnessPlaceholder<EntryAdmission>,
    ) {
    }
}

pub use inner::admit as public_admit;
pub use inner::EntryAdmission;
"#
}

/// Hostile: public fn generic over AuthorityMarker.
pub fn hostile_authority_marker_bound() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
"#
}

/// Hostile: where-clause CapabilityMarker bound.
pub fn hostile_capability_where_clause() -> &'static str {
    r#"
pub trait CapabilityMarker: 'static {}

pub fn execute<Cap>(_cap: Cap)
where
    Cap: CapabilityMarker,
{
}
"#
}

/// Hostile: AuthorityProves bound.
pub fn hostile_authority_proves_bound() -> &'static str {
    r#"
pub trait AuthorityProves<P>: 'static {}

pub fn mint<Auth, Fact>(_auth: Auth)
where
    Auth: AuthorityProves<Fact>,
{
}
"#
}

/// Hostile: ProofSetAuthorizedBy bound.
pub fn hostile_proof_set_authorized_by() -> &'static str {
    r#"
pub trait ProofSetAuthorizedBy<Auth>: 'static {}

pub fn join<Proofs, Auth>(_proofs: Proofs)
where
    Proofs: ProofSetAuthorizedBy<Auth>,
{
}
"#
}

/// Hostile: renamed import of AuthorityMarker used as bound.
pub fn hostile_renamed_import() -> &'static str {
    r#"
mod markers {
    pub trait AuthorityMarker: 'static {}
}

use markers::AuthorityMarker as AuthMark;

pub fn admit<Auth: AuthMark>(_authority: Auth) {}
"#
}

/// Hostile: public re-export of a private generic authority-bound fn.
pub fn hostile_reexport_promotion() -> &'static str {
    r#"
mod private_lane {
    pub trait AuthorityMarker: 'static {}

    pub fn admit<Auth: AuthorityMarker>(_authority: Auth) {}
}

pub use private_lane::admit;
"#
}

/// Private generic bound must not fire (not externally reachable).
pub fn private_generic_bound_is_legal() -> &'static str {
    r#"
trait AuthorityMarker: 'static {}

fn internal_admit<Auth: AuthorityMarker>(_authority: Auth) {}

pub fn seed() {}
"#
}

/// Hostile: impl Trait return with AuthorityMarker.
pub fn hostile_impl_trait_return() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Local;
impl AuthorityMarker for Local {}

pub fn issue() -> impl AuthorityMarker {
    Local
}
"#
}

/// Hostile: public inherent method with `impl AuthorityMarker` parameter.
pub fn hostile_public_method_impl_trait_param() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Ceremony;

impl Ceremony {
    pub fn admit(_authority: impl AuthorityMarker) {}
}
"#
}

/// Hostile: public trait associated type bound on AuthorityMarker.
pub fn hostile_trait_associated_type_bound() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub trait Ceremony {
    type Authority: AuthorityMarker;
}
"#
}

/// Hostile: public struct field carrier of dyn AuthorityMarker.
pub fn hostile_public_field_carrier() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub struct Carrier {
    pub authority: &'static dyn AuthorityMarker,
}
"#
}

/// Hostile: public type alias RHS with AuthorityMarker object.
pub fn hostile_type_alias_rhs() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub type AuthObject = dyn AuthorityMarker;
"#
}

/// Hostile: public const type carrying AuthorityMarker.
pub fn hostile_const_type_carrier() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

pub const SLOT: Option<fn(&dyn AuthorityMarker)> = None;
"#
}

/// Hostile: item-position macro that expands to a generic public ceremony.
pub fn hostile_macro_expanded_public_fn() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

macro_rules! mint_admit {
    () => {
        pub fn admit<A: AuthorityMarker>(_authority: A) {}
    };
}

mint_admit!();
"#
}

/// Legal: pub method on a private type must not be governed.
pub fn private_type_public_method_is_legal() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

struct PrivateCeremony;

impl PrivateCeremony {
    pub fn admit(_authority: impl AuthorityMarker) {}
}

pub fn seed() {}
"#
}

/// Hostile: nested module + re-export of inherent method ceremony.
pub fn hostile_nested_reexport_method() -> &'static str {
    r#"
pub trait AuthorityMarker: 'static {}

mod inner {
    use super::AuthorityMarker;

    pub struct Ceremony;

    impl Ceremony {
        pub fn admit(_authority: impl AuthorityMarker) {}
    }
}

pub use inner::Ceremony;
"#
}
