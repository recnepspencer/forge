//! Move-only authorization for one serialized application commit transition.

use std::marker::PhantomData;

use super::WorthQueryOperationAdmissionIdentity;
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitSerialization;

pub(in crate::domain_computation) struct WorthQueryApplicationCommitAuthorization<'serialization> {
    admission_identity: WorthQueryOperationAdmissionIdentity,
    _serialization: PhantomData<&'serialization ()>,
}

impl<'serialization> WorthQueryApplicationCommitAuthorization<'serialization> {
    pub(super) fn mint(
        _serialization: &'serialization WorthQueryApplicationCommitSerialization<'_>,
        admission_identity: WorthQueryOperationAdmissionIdentity,
    ) -> Self {
        Self {
            admission_identity,
            _serialization: PhantomData,
        }
    }

    pub(in crate::domain_computation) fn govern<Outcome>(
        self,
        admission_identity: WorthQueryOperationAdmissionIdentity,
        transition: impl FnOnce() -> Outcome,
    ) -> Result<Outcome, ()> {
        if self.admission_identity != admission_identity {
            return Err(());
        }
        Ok(transition())
    }
}
