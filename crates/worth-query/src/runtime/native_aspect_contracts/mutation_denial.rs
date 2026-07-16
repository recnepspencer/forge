use crate::runtime::WorthQueryAspectTouch;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMutationContractDenialKind {
    MissingContract,
    ContractValidationDenied,
    FieldMutationRequiresScalar,
    ClearDuringCreation,
    NestedFieldMutationUnsupported,
    IncompatibleSymbolicReference,
    AuthoritativePatchDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMutationContractDenial {
    kind: WorthQueryMutationContractDenialKind,
    touch: WorthQueryAspectTouch,
    detail: String,
}

impl WorthQueryMutationContractDenial {
    pub(super) fn new(
        kind: WorthQueryMutationContractDenialKind,
        touch: WorthQueryAspectTouch,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            touch,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryMutationContractDenialKind {
        self.kind
    }

    pub fn touch(&self) -> &WorthQueryAspectTouch {
        &self.touch
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub(crate) fn portable_export_denied(
        denial: worth_foundational::facade::PortableAspectExportDenial,
    ) -> Self {
        let key = match &denial {
            worth_foundational::facade::PortableAspectExportDenial::MissingContract(key)
            | worth_foundational::facade::PortableAspectExportDenial::ContractIdentityDrift {
                key,
                ..
            }
            | worth_foundational::facade::PortableAspectExportDenial::ContractRevisionDrift {
                key,
                ..
            } => key.clone(),
        };
        Self::new(
            WorthQueryMutationContractDenialKind::AuthoritativePatchDenied,
            WorthQueryAspectTouch::whole_aspect(key),
            format!("authoritative patch could not cross the portable boundary: {denial:?}"),
        )
    }
}

impl std::fmt::Display for WorthQueryMutationContractDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "native mutation contract admission denied `{}`: {}",
            self.touch.admitted_touch_digest_part(),
            self.detail
        )
    }
}

impl std::error::Error for WorthQueryMutationContractDenial {}
