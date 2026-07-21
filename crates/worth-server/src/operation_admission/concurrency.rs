use super::{WorthServerOperationAdmissionPosture, WorthServerOperationAuthorityKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerOperationConcurrencyClass {
    ConcurrentSharedRead,
    SerializeDeterministically,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerOperationConcurrencyDenialCode {
    ConflictingMutableAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationConcurrencyDenial {
    code: WorthServerOperationConcurrencyDenialCode,
    detail: String,
}

impl WorthServerOperationConcurrencyDenial {
    fn conflicting_mutable_authority(detail: impl Into<String>) -> Self {
        Self {
            code: WorthServerOperationConcurrencyDenialCode::ConflictingMutableAuthority,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerOperationConcurrencyDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Default)]
pub struct WorthServerOperationConcurrencyFacade;

impl WorthServerOperationConcurrencyFacade {
    pub fn classify_pair(
        &self,
        left: &WorthServerOperationAdmissionPosture,
        right: &WorthServerOperationAdmissionPosture,
    ) -> Result<WorthServerOperationConcurrencyClass, WorthServerOperationConcurrencyDenial> {
        if left.authority_footprint().authority_kind()
            == WorthServerOperationAuthorityKind::SharedReadOnly
            && right.authority_footprint().authority_kind()
                == WorthServerOperationAuthorityKind::SharedReadOnly
            && left.authority_footprint().canonical_digest()
                == right.authority_footprint().canonical_digest()
        {
            return Ok(WorthServerOperationConcurrencyClass::ConcurrentSharedRead);
        }
        if left.authority_footprint().authority_kind()
            == WorthServerOperationAuthorityKind::ProductDraftMutation
            && right.authority_footprint().authority_kind()
                == WorthServerOperationAuthorityKind::ProductDraftMutation
            && left.authority_footprint().scope().canonical_digest()
                == right.authority_footprint().scope().canonical_digest()
        {
            return Err(
                WorthServerOperationConcurrencyDenial::conflicting_mutable_authority(format!(
                    "product draft scope `{}` cannot admit concurrent mutable execution",
                    left.authority_footprint().scope().canonical_digest()
                )),
            );
        }
        if left.authority_footprint().authority_kind()
            == WorthServerOperationAuthorityKind::DurableProductMutation
            && right.authority_footprint().authority_kind()
                == WorthServerOperationAuthorityKind::DurableProductMutation
            && left.authority_footprint().scope().canonical_digest()
                == right.authority_footprint().scope().canonical_digest()
        {
            return Err(
                WorthServerOperationConcurrencyDenial::conflicting_mutable_authority(format!(
                    "durable product scope `{}` cannot admit concurrent mutable execution",
                    left.authority_footprint().scope().canonical_digest()
                )),
            );
        }
        Ok(WorthServerOperationConcurrencyClass::SerializeDeterministically)
    }
}
