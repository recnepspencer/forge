#![allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingContextWitness {
    source_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingAuthorityWitness {
    handle_identity_digest: String,
    operating_context_identity_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingBasisWitness {
    basis_label: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingTargetWitnessSet {
    binding_digests: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBindingFamilyWitness {
    family_key: &'static str,
}

impl WorthQueryBindingContextWitness {
    pub fn source_count(&self) -> usize {
        self.source_count
    }

    pub(crate) fn new(source_count: usize) -> Self {
        Self { source_count }
    }
}

impl WorthQueryBindingAuthorityWitness {
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

impl WorthQueryBindingBasisWitness {
    pub fn basis_label(&self) -> &'static str {
        self.basis_label
    }

    pub(crate) fn new(basis_label: &'static str) -> Self {
        Self { basis_label }
    }
}

impl WorthQueryBindingTargetWitnessSet {
    pub fn binding_digests(&self) -> &[String] {
        &self.binding_digests
    }

    pub(crate) fn new(binding_digests: Vec<String>) -> Self {
        Self { binding_digests }
    }
}

impl WorthQueryBindingFamilyWitness {
    pub fn family_key(&self) -> &'static str {
        self.family_key
    }

    pub(crate) fn new(family_key: &'static str) -> Self {
        Self { family_key }
    }
}
