#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactParts<T, S, A> {
    payload: T,
    proofs: S,
    basis: A,
}

impl<T, S, A> ArtifactParts<T, S, A> {
    pub(crate) fn new(payload: T, proofs: S, basis: A) -> Self {
        Self {
            payload,
            proofs,
            basis,
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn proofs(&self) -> &S {
        &self.proofs
    }

    pub fn basis(&self) -> &A {
        &self.basis
    }

    pub fn into_parts(self) -> (T, S, A) {
        (self.payload, self.proofs, self.basis)
    }
}
