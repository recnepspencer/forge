//! Associated-projection authority producer specimens.

pub fn hostile_primitive_trait_associated_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionFactory { type Output; fn issue() -> Self::Output; }
impl AdmissionFactory for () {
    type Output = EntryAdmission;
    fn issue() -> Self::Output { EntryAdmission { _gate: () } }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_primitive_associated_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
impl AdmissionProjection for () { type Output = EntryAdmission; }
pub fn issue() -> <() as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_alias_associated_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
pub type Primitive = ();
impl AdmissionProjection for Primitive { type Output = EntryAdmission; }
pub fn issue() -> <() as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_alias_chain_associated_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
pub type First = ();
pub mod aliases { pub type Second = super::First; }
use aliases::Second as Primitive;
impl AdmissionProjection for () { type Output = EntryAdmission; }
pub fn issue() -> <Primitive as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_long_alias_chain_associated_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
pub type A00 = ();
pub type A01 = A00; pub type A02 = A01; pub type A03 = A02; pub type A04 = A03;
pub type A05 = A04; pub type A06 = A05; pub type A07 = A06; pub type A08 = A07;
pub type A09 = A08; pub type A10 = A09; pub type A11 = A10; pub type A12 = A11;
pub type A13 = A12; pub type A14 = A13; pub type A15 = A14; pub type A16 = A15;
pub type A17 = A16; pub type A18 = A17; pub type A19 = A18; pub type A20 = A19;
impl AdmissionProjection for () { type Output = EntryAdmission; }
pub fn issue() -> <A20 as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_reexported_trait_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub mod owner {
    pub trait Projection { type Output; }
    impl Projection for () { type Output = super::EntryAdmission; }
}
pub mod first { pub use super::owner::Projection as Renamed; }
pub mod second { pub use super::first::Renamed as PublicProjection; }
pub fn issue() -> <() as second::PublicProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_blanket_associated_projection_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
impl<T> AdmissionProjection for T { type Output = EntryAdmission; }
pub fn issue() -> <u8 as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_generic_output_substitution_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection<T> { type Output; }
impl<T> AdmissionProjection<T> for () { type Output = T; }
pub fn issue() -> <() as AdmissionProjection<EntryAdmission>>::Output {
    EntryAdmission { _gate: () }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_satisfied_local_constraint_projection() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait Eligible {}
impl Eligible for () {}
pub trait AdmissionProjection { type Output; }
impl<T: Eligible> AdmissionProjection for T { type Output = EntryAdmission; }
pub fn issue() -> <() as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_satisfied_where_constraint_projection() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait Eligible {}
impl Eligible for () {}
pub trait AdmissionProjection { type Output; }
impl<T> AdmissionProjection for T where T: Eligible { type Output = EntryAdmission; }
pub fn issue() -> <() as AdmissionProjection>::Output { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_primitive_associated_projection_borrow() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionProjection { type Output; }
impl AdmissionProjection for () { type Output = EntryAdmission; }
pub fn inspect(value: &<() as AdmissionProjection>::Output) -> &<() as AdmissionProjection>::Output {
    value
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_wrong_qualified_associated_projection() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub mod authority {
    pub trait Projection { type Output; }
    impl Projection for () { type Output = super::EntryAdmission; }
}
pub mod ordinary {
    pub trait Projection { type Output; }
    impl Projection for () { type Output = u8; }
}
pub fn ordinary_value() -> <() as ordinary::Projection>::Output { 7 }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_primitive_trait_borrowed_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionInspect { type Output; fn inspect(value: &Self::Output) -> &Self::Output; }
impl AdmissionInspect for () {
    type Output = EntryAdmission;
    fn inspect(value: &Self::Output) -> &Self::Output { value }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_trait_argument_mismatch_projection() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait Projection<T> { type Output; }
impl Projection<u8> for () { type Output = EntryAdmission; }
impl Projection<u16> for () { type Output = u16; }
pub fn ordinary() -> <() as Projection<u16>>::Output { 7 }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_unsatisfied_local_constraint_projection() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait Eligible {}
impl Eligible for u8 {}
pub trait AdmissionProjection { type Output; }
impl<T: Eligible> AdmissionProjection for T { type Output = EntryAdmission; }
pub trait OrdinaryProjection { type Output; }
impl OrdinaryProjection for () { type Output = u16; }
pub fn ordinary() -> <() as OrdinaryProjection>::Output { 7 }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}
