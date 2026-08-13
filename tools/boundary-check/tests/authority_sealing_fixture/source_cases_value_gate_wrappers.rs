//! Owned-wrapper authority producer specimens.

pub fn hostile_result_wrapped_self_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
impl EntryAdmission { pub fn issue() -> Result<Self, ()> { Ok(Self { _gate: () }) } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_option_wrapped_trait_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait AdmissionFactory { fn issue() -> Option<Self> where Self: Sized; }
impl AdmissionFactory for EntryAdmission { fn issue() -> Option<Self> { Some(Self { _gate: () }) } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_box_wrapped_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
impl EntryAdmission { pub fn issue() -> Box<Self> { Box::new(Self { _gate: () }) } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_array_wrapped_self_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
impl EntryAdmission { pub fn issue() -> [Self; 1] { [Self { _gate: () }] } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_borrowed_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
impl EntryAdmission { pub fn inspect(value: &Self) -> &Self { value } }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_boxed_iterator_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub fn issue() -> Box<dyn Iterator<Item = EntryAdmission>> {
    Box::new(core::iter::once(EntryAdmission { _gate: () }))
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_boxed_function_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub fn issue() -> Box<dyn Fn() -> EntryAdmission> {
    Box::new(|| EntryAdmission { _gate: () })
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_borrowed_trait_object_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub trait Inspect { fn inspect(&self) -> &EntryAdmission; }
pub fn inspect(value: &dyn Inspect) -> &EntryAdmission { value.inspect() }
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn legal_phantom_marker_metadata_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub fn metadata() -> core::marker::PhantomData<EntryAdmission> {
    core::marker::PhantomData
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

pub fn hostile_same_named_phantom_wrapper_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub struct PhantomData<T>(pub T);
pub fn issue() -> PhantomData<EntryAdmission> {
    PhantomData(EntryAdmission { _gate: () })
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

/// Legal: a same-terminal-name qualified return remains a different type.
pub fn legal_wrong_qualified_same_named_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub struct EntryAdmission { _gate: () }
pub mod ordinary { pub struct EntryAdmission { pub token: u8 } }
impl EntryAdmission {
    pub fn metadata() -> ordinary::EntryAdmission { ordinary::EntryAdmission { token: 1 } }
}
pub fn admit(_authority: AuthorityWitness<EntryAdmission>) {}"#
}

/// Hostile: a qualified exact marker return is still a public mint.
pub fn hostile_qualified_exact_marker_factory() -> &'static str {
    r#"use worth_proof::AuthorityWitness;
pub mod authority { pub struct EntryAdmission { pub(crate) _gate: () } }
pub struct Gate;
impl Gate {
    pub fn issue() -> crate::authority::EntryAdmission {
        crate::authority::EntryAdmission { _gate: () }
    }
}
pub fn admit(_authority: AuthorityWitness<authority::EntryAdmission>) {}"#
}
