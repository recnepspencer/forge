use std::marker::PhantomData;

use super::carrier::Artifact;
use super::parts::ArtifactParts;

/// Read-only borrowed view over a proof-bearing artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactView<'a, P, T, S, A> {
    payload: &'a T,
    proofs: &'a S,
    basis: &'a A,
    phase: PhantomData<P>,
}

impl<'a, P, T, S, A> ArtifactView<'a, P, T, S, A> {
    pub fn payload(&self) -> &'a T {
        self.payload
    }

    pub fn proofs(&self) -> &'a S {
        self.proofs
    }

    pub fn basis(&self) -> &'a A {
        self.basis
    }
}

impl<P, T, S, A> Artifact<P, T, S, A> {
    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn proofs(&self) -> &S {
        &self.proofs
    }

    pub fn basis(&self) -> &A {
        &self.basis
    }

    pub fn view(&self) -> ArtifactView<'_, P, T, S, A> {
        ArtifactView {
            payload: &self.payload,
            proofs: &self.proofs,
            basis: &self.basis,
            phase: PhantomData,
        }
    }

    pub fn into_parts(self) -> ArtifactParts<T, S, A> {
        ArtifactParts::new(self.payload, self.proofs, self.basis)
    }
}
