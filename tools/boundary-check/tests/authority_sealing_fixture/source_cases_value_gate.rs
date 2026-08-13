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

/// Hostile: a derived Default is an equally public marker mint.
pub fn hostile_derived_default_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

#[derive(Default)]
pub struct EntryAdmission { _value_gate: () }
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

/// Hostile: a reachable free function can publicly mint the marker.
pub fn hostile_public_free_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _value_gate: () }
pub fn issue() -> EntryAdmission { EntryAdmission { _value_gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a public alias cannot hide a returned concrete marker.
pub fn hostile_aliased_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _value_gate: () }
pub type Mint = EntryAdmission;
pub fn issue() -> Mint { EntryAdmission { _value_gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: an owning wrapper alias cannot hide a returned marker.
pub fn hostile_wrapped_alias_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _value_gate: () }
pub type Mint = Option<EntryAdmission>;
pub fn issue() -> Mint { Some(EntryAdmission { _value_gate: () }) }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a function-pointer alias cannot hide a public marker value.
pub fn hostile_function_pointer_alias_const() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _value_gate: () }
fn private_issue() -> EntryAdmission { EntryAdmission { _value_gate: () } }
pub type Factory = fn() -> EntryAdmission;
pub const ISSUE: Factory = private_issue;
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: an opaque iterator return owns its associated marker.
pub fn hostile_impl_trait_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _value_gate: () }
pub fn issue() -> impl Iterator<Item = EntryAdmission> {
    core::iter::once(EntryAdmission { _value_gate: () })
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a qualified derive path still exposes Default.
pub fn hostile_qualified_derived_default_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
#[derive(core::default::Default)]
pub struct EntryAdmission { _value_gate: () }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a wrapped trait declaration grants owned marker values.
pub fn hostile_wrapped_trait_declaration_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _value_gate: () }
pub trait AdmissionFactory { fn issue() -> Option<EntryAdmission>; }
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

pub fn hostile_enum_empty_named_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub enum EntryAdmission { Granted {} }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

pub fn hostile_enum_empty_tuple_marker() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub enum EntryAdmission { Granted() }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: an owning wrapper around a public constant still mints the marker.
pub fn hostile_wrapped_public_marker_const() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub const ENTRY_ADMISSIONS: [EntryAdmission; 1] = [EntryAdmission { _gate: () }];
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: an associated output projection cannot hide a public marker mint.
pub fn hostile_associated_output_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub struct Gate;
pub trait Factory { type Output; fn issue() -> Self::Output; }
impl Factory for Gate {
    type Output = EntryAdmission;
    fn issue() -> Self::Output { EntryAdmission { _gate: () } }
}
pub fn projected_issue() -> <Gate as Factory>::Output { Gate::issue() }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a reachable unrelated inherent impl publicly produces the marker.
pub fn hostile_unrelated_impl_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub struct Gate;
impl Gate {
    pub fn issue() -> EntryAdmission { EntryAdmission { _gate: () } }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Hostile: a public trait implementation returns its marker through `Self`.
pub fn hostile_trait_self_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub trait AdmissionFactory { fn issue() -> Self; }
impl AdmissionFactory for EntryAdmission {
    fn issue() -> Self { Self { _gate: () } }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

/// Legal: `Self` returned by an unrelated type is not the admitted marker.
pub fn legal_unrelated_self_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;

pub struct EntryAdmission { _gate: () }
pub struct Gate { _gate: () }
impl Gate {
    pub fn issue() -> Self { Self { _gate: () } }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}
