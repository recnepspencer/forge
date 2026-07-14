//! Value-gate specimens: concrete ceremony markers must not be caller-mintable.

/// Legal: real AuthorityWitness ceremony with private-field platform marker.
pub fn legal_value_gated_authority_witness() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission {
    _value_gate: (),
}

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Legal: CapabilityWitness + Proof with value-gated markers.
pub fn legal_value_gated_capability_and_proof() -> &'static str {
    r#"
use worth_proof::{CapabilityWitness, Proof};

pub struct EntryExecution {
    _value_gate: (),
}

pub struct EntryAdmission {
    _value_gate: (),
}

pub struct AdmissionEligible;

pub fn admit_cap(_capability: CapabilityWitness<EntryExecution>) {}

pub fn admit_proof(_proof: Proof<AdmissionEligible, EntryAdmission>) {}
"#
}

/// Hostile: unit struct platform marker admitted by AuthorityWitness ceremony.
pub fn hostile_unit_marker_authority_witness() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission;

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: public fields make the marker caller-constructible.
pub fn hostile_public_fields_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission {
    pub token: u8,
}

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: private field but public Default still mints the marker.
pub fn hostile_default_impl_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission {
    _value_gate: (),
}

impl Default for EntryAdmission {
    fn default() -> Self {
        Self { _value_gate: () }
    }
}

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: private field but public constructor returns Self.
pub fn hostile_public_constructor_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission {
    _value_gate: (),
}

impl EntryAdmission {
    pub fn new() -> Self {
        Self { _value_gate: () }
    }
}

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: unit enum variant is caller-constructible.
pub fn hostile_enum_unit_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub enum EntryAdmission {
    Granted,
}

pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: public method ceremony admits a unit marker.
pub fn hostile_method_unit_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission;

pub struct Gate;

impl Gate {
    pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
}
"#
}

/// Hostile: an alias of the Cargo-identified carrier remains the same carrier.
pub fn hostile_aliased_carrier() -> &'static str {
    r#"
use worth_proof::AuthorityWitness as AdmissionWitness;

pub struct EntryAdmission;
pub fn admit(_authority: AdmissionWitness<EntryAdmission>) {}
"#
}

/// Hostile: a generic type alias cannot erase the platform carrier identity.
pub fn hostile_type_aliased_carrier() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
type AdmissionWitness<A> = AuthorityWitness<A>;

pub struct EntryAdmission;
pub fn admit(_authority: AdmissionWitness<EntryAdmission>) {}
"#
}

/// Legal: a same-named local carrier is not `worth-proof` authority machinery.
pub fn legal_same_named_foreign_carrier() -> &'static str {
    r#"
mod ordinary {
    pub struct AuthorityWitness<A>(pub core::marker::PhantomData<A>);
}
pub struct EntryAdmission;
pub fn observe(_value: ordinary::AuthorityWitness<EntryAdmission>) {}
"#
}

/// Legal: an in-crate re-export preserves the marker's definition identity.
pub fn legal_reexported_value_gated_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

mod authority { pub struct EntryAdmission { _gate: () } }
pub mod facade { pub use crate::authority::EntryAdmission; }
pub fn admit(_authority: AuthorityWitness<facade::EntryAdmission>) {}
"#
}

/// Hostile: qualified marker identity must select the mintable definition.
pub fn hostile_qualified_same_named_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub mod sealed { pub struct EntryAdmission { _gate: () } }
pub mod exposed { pub struct EntryAdmission; }
pub fn admit(_authority: AuthorityWitness<exposed::EntryAdmission>) {}
"#
}

/// Hostile: a public constant is a caller-visible marker mint.
pub fn hostile_public_marker_const() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub const ENTRY_ADMISSION: EntryAdmission = EntryAdmission { _gate: () };
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a public trait factory is not an owning ceremony function.
pub fn hostile_trait_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub trait AdmissionFactory { fn issue() -> EntryAdmission; }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}
