use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReadCompositionExtensionHookFamily {
    DomainReadFamilyLowering,
    DomainInvariantPack,
    DomainDecoder,
    DomainResultCertification,
}

impl WorthQueryReadCompositionExtensionHookFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DomainReadFamilyLowering => "domain_read_family_lowering",
            Self::DomainInvariantPack => "domain_invariant_pack",
            Self::DomainDecoder => "domain_decoder",
            Self::DomainResultCertification => "domain_result_certification",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReadCompositionExtensionHookBoundary {
    Lowering,
    InvariantPack,
    Decoder,
    Certification,
}

impl WorthQueryReadCompositionExtensionHookBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lowering => "lowering",
            Self::InvariantPack => "invariant-pack",
            Self::Decoder => "decoder",
            Self::Certification => "certification",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadCompositionExtensionHookSupportRow {
    family: WorthQueryReadCompositionExtensionHookFamily,
    boundary: WorthQueryReadCompositionExtensionHookBoundary,
    semantic_bypass_allowed: bool,
    row_digest: String,
}

impl WorthQueryReadCompositionExtensionHookSupportRow {
    pub(crate) fn new(
        family: WorthQueryReadCompositionExtensionHookFamily,
        boundary: WorthQueryReadCompositionExtensionHookBoundary,
        semantic_bypass_allowed: bool,
    ) -> Self {
        let row_digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("boundary:{}", boundary.as_str()),
            format!("semantic-bypass:{semantic_bypass_allowed}"),
        ]);
        Self {
            family,
            boundary,
            semantic_bypass_allowed,
            row_digest,
        }
    }

    pub fn family(&self) -> WorthQueryReadCompositionExtensionHookFamily {
        self.family
    }

    pub fn hook_family(&self) -> &str {
        self.family.as_str()
    }

    pub fn boundary(&self) -> WorthQueryReadCompositionExtensionHookBoundary {
        self.boundary
    }

    pub fn semantic_bypass_allowed(&self) -> bool {
        self.semantic_bypass_allowed
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
