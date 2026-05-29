#![allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingContextWitness {
    source_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingAuthorityWitness {
    handle_identity_digest: String,
    operating_context_identity_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingBasisWitness {
    basis_label: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingTargetWitnessSet {
    binding_digests: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBindingFamilyWitness {
    family_key: &'static str,
}

impl ForgeQueryBindingContextWitness {
    pub fn source_count(&self) -> usize {
        self.source_count
    }

    pub(crate) fn new(source_count: usize) -> Self {
        Self { source_count }
    }
}

impl ForgeQueryBindingAuthorityWitness {
    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub(crate) fn new(
        handle_identity_digest: impl Into<String>,
        operating_context_identity_digest: impl Into<String>,
    ) -> Self {
        Self {
            handle_identity_digest: handle_identity_digest.into(),
            operating_context_identity_digest: operating_context_identity_digest.into(),
        }
    }
}

impl ForgeQueryBindingBasisWitness {
    pub fn basis_label(&self) -> &'static str {
        self.basis_label
    }

    pub(crate) fn new(basis_label: &'static str) -> Self {
        Self { basis_label }
    }
}

impl ForgeQueryBindingTargetWitnessSet {
    pub fn binding_digests(&self) -> &[String] {
        &self.binding_digests
    }

    pub(crate) fn new(binding_digests: Vec<String>) -> Self {
        Self { binding_digests }
    }
}

impl ForgeQueryBindingFamilyWitness {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub(crate) fn new(family_key: &'static str) -> Self {
        Self { family_key }
    }
}
