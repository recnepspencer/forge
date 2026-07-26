use crate::domain_computation::{
    WorthQueryPortableArtifactContract, WorthQueryTransformationEvidenceContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let valid = match &contract.transformation {
        WorthQueryTransformationEvidenceContract::NotTransformation => true,
        WorthQueryTransformationEvidenceContract::Declared {
            source_occurrence,
            transformation,
            ..
        } => {
            portable_text(source_occurrence.identity_family())
                && portable_text(transformation.family())
                && transformation.version() > 0
        }
    };
    valid.then_some(()).ok_or_else(|| {
        WorthQueryArtifactContractValidationDenial::new(
            Kind::InvalidTransformationContract,
            contract.family.as_str(),
        )
    })
}
