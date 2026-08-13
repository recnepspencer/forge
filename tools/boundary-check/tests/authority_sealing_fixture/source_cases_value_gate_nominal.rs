//! Value-gate specimens for public aliases and nominal owning wrappers.

pub fn hostile_public_alias_to_private_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
struct EntryAdmission { _gate: () }
pub type Mint = EntryAdmission;
pub fn issue() -> Mint { EntryAdmission { _gate: () } }
pub fn admit(_authority: AuthorityWitness<Mint>) {}
"#
}

pub fn hostile_public_tuple_wrapper_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub struct Packet(pub EntryAdmission);
pub fn issue() -> Packet { Packet(EntryAdmission { _gate: () }) }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

pub fn hostile_public_enum_wrapper_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub enum Packet { Granted(EntryAdmission) }
pub fn issue() -> Packet { Packet::Granted(EntryAdmission { _gate: () }) }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}

pub fn legal_private_wrapper_marker_factory() -> &'static str {
    r#"
use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub struct Packet(EntryAdmission);
pub fn issue() -> Packet { Packet(EntryAdmission { _gate: () }) }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}
"#
}
