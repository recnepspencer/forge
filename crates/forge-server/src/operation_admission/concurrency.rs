use super::{ForgeServerOperationAdmissionPosture, ForgeServerOperationAuthorityKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationConcurrencyClass {
    ConcurrentSharedRead,
    SerializeDeterministically,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationConcurrencyDenialCode {
    ConflictingMutableAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationConcurrencyDenial {
    code: ForgeServerOperationConcurrencyDenialCode,
    detail: String,
}

impl ForgeServerOperationConcurrencyDenial {
    fn conflicting_mutable_authority(detail: impl Into<String>) -> Self {
        Self {
            code: ForgeServerOperationConcurrencyDenialCode::ConflictingMutableAuthority,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerOperationConcurrencyDenialCode {
        self.code.clone()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Default)]
pub struct ForgeServerOperationConcurrencyFacade;

impl ForgeServerOperationConcurrencyFacade {
    pub fn classify_pair(
        &self,
        left: &ForgeServerOperationAdmissionPosture,
        right: &ForgeServerOperationAdmissionPosture,
    ) -> Result<ForgeServerOperationConcurrencyClass, ForgeServerOperationConcurrencyDenial> {
        if left.authority_footprint().authority_kind()
            == ForgeServerOperationAuthorityKind::SharedReadOnly
            && right.authority_footprint().authority_kind()
                == ForgeServerOperationAuthorityKind::SharedReadOnly
            && left.authority_footprint().canonical_digest()
                == right.authority_footprint().canonical_digest()
        {
            return Ok(ForgeServerOperationConcurrencyClass::ConcurrentSharedRead);
        }
        if left.authority_footprint().authority_kind()
            == ForgeServerOperationAuthorityKind::ProductDraftMutation
            && right.authority_footprint().authority_kind()
                == ForgeServerOperationAuthorityKind::ProductDraftMutation
            && left.authority_footprint().scope().canonical_digest()
                == right.authority_footprint().scope().canonical_digest()
        {
            return Err(
                ForgeServerOperationConcurrencyDenial::conflicting_mutable_authority(format!(
                    "product draft scope `{}` cannot admit concurrent mutable execution",
                    left.authority_footprint().scope().canonical_digest()
                )),
            );
        }
        Ok(ForgeServerOperationConcurrencyClass::SerializeDeterministically)
    }
}
