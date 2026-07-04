use super::admitted_input::SpatialCompiledProductFamilyAdmittedInput;
use super::declaration::SpatialCompiledProductFamilyDeclaration;
use super::error::{SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind};
use super::family_identity::SpatialCompiledProductFamilyIdentity;
use super::posture::SpatialLocalityFootprintBasisPosture;
use schema::facade::platform::authority::compiled_product_semantic_graph::{
    admit_compiled_product_authority_truth_identity,
    admit_compiled_product_authority_truth_identity_with_coordinates,
    admit_compiled_product_equivalence_policy_identity, admit_compiled_product_identity,
    admit_compiled_product_prior_proof_identity, admit_compiled_product_rebuild_denial_identity,
    admit_compiled_product_stage_identity, CompiledProductAuthorityInstanceCoordinate,
    CompiledProductAuthorityTruthIdentity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity, CompiledProductLocalityFootprintIdentity,
    CompiledProductRebuildDenialIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialCompiledProductLoweredIdentity {
    family_identity: SpatialCompiledProductFamilyIdentity,
    family_digest: String,
    authority_truth_identity: CompiledProductAuthorityTruthIdentity,
    compiled_product_identity: CompiledProductIdentity,
    prior_proof_identity: Option<schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductPriorProofIdentity>,
    equivalence_policy_identity: CompiledProductEquivalencePolicyIdentity,
}

impl SpatialCompiledProductLoweredIdentity {
    pub fn family_identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.family_identity
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn authority_truth_identity(&self) -> &CompiledProductAuthorityTruthIdentity {
        &self.authority_truth_identity
    }

    pub fn compiled_product_identity(&self) -> &CompiledProductIdentity {
        &self.compiled_product_identity
    }

    pub fn prior_proof_identity(
        &self,
    ) -> Option<
        &schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductPriorProofIdentity,
    >{
        self.prior_proof_identity.as_ref()
    }

    pub fn equivalence_policy_identity(&self) -> &CompiledProductEquivalencePolicyIdentity {
        &self.equivalence_policy_identity
    }

    pub fn rebuild_required_identity(
        &self,
        denial_reason: &str,
    ) -> Result<CompiledProductRebuildDenialIdentity, SpatialCompiledProductFamilyError> {
        admit_compiled_product_rebuild_denial_identity(
            &self.compiled_product_identity,
            denial_reason,
        )
        .map_err(|error| {
            SpatialCompiledProductFamilyError::new(
                SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                format!("compiled-product rebuild denial admission failed: {error:?}"),
            )
        })
    }
}

pub fn lower_spatial_compiled_product_identity(
    declaration: &SpatialCompiledProductFamilyDeclaration,
    admitted_input: &SpatialCompiledProductFamilyAdmittedInput,
) -> Result<SpatialCompiledProductLoweredIdentity, SpatialCompiledProductFamilyError> {
    let authority_truth_identity = authority_truth_identity(declaration, admitted_input)?;
    let locality_identity = locality_identity(declaration, admitted_input)?;
    let prior_proof_identity = admitted_input
        .prior_proof_digest()
        .map(|digest| {
            admit_compiled_product_prior_proof_identity(
                digest.to_string(),
                declaration
                    .compiled_product_prior_proof_role()
                    .expect("prior-proof digest only admitted for declarations that require it"),
            )
            .map_err(|error| {
                SpatialCompiledProductFamilyError::new(
                    SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                    format!("compiled-product prior-proof admission failed: {error:?}"),
                )
            })
        })
        .transpose()?;
    let stage_identity = admitted_input
        .stage_receipt_digest()
        .map(|digest| {
            admit_compiled_product_stage_identity(digest.to_string()).map_err(|error| {
                SpatialCompiledProductFamilyError::new(
                    SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                    format!("compiled-product stage identity admission failed: {error:?}"),
                )
            })
        })
        .transpose()?;
    let compiled_product_identity = admit_compiled_product_identity(
        authority_truth_identity.clone(),
        locality_identity,
        prior_proof_identity.clone(),
        stage_identity,
    );
    let equivalence_policy_identity = admit_compiled_product_equivalence_policy_identity(
        declaration.equivalence_policy_name(),
        declaration.equivalence_dimensions().iter().copied(),
    )
    .map_err(|error| {
        SpatialCompiledProductFamilyError::new(
            SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product equivalence policy admission failed: {error:?}"),
        )
    })?;
    Ok(SpatialCompiledProductLoweredIdentity {
        family_identity: declaration.identity(),
        family_digest: declaration.family_digest().to_string(),
        authority_truth_identity,
        compiled_product_identity,
        prior_proof_identity,
        equivalence_policy_identity,
    })
}

fn authority_truth_identity(
    declaration: &SpatialCompiledProductFamilyDeclaration,
    admitted_input: &SpatialCompiledProductFamilyAdmittedInput,
) -> Result<CompiledProductAuthorityTruthIdentity, SpatialCompiledProductFamilyError> {
    match declaration.source_authority_digest_basis() {
        super::posture::SpatialSourceAuthorityDigestBasisPosture::EvidenceLookupLedgerBasisWithStageReceiptCoordinate => admit_compiled_product_authority_truth_identity_with_coordinates(
            "worth-spatial",
            admitted_input.source_authority_digest(),
            "evidence-lookup-ledger-basis",
            [
                CompiledProductAuthorityInstanceCoordinate::stage_receipt_identity(
                    admitted_input
                        .stage_receipt_digest()
                        .ok_or_else(|| SpatialCompiledProductFamilyError::new(
                            SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
                            "evidence lookup lowering requires a stage receipt digest",
                        ))?
                        .to_string(),
                )
                .expect("evidence lookup stage receipt coordinate"),
            ],
        ),
        super::posture::SpatialSourceAuthorityDigestBasisPosture::RetainedCancellationChainAuthorityDigest => admit_compiled_product_authority_truth_identity(
            "worth-spatial",
            admitted_input.source_authority_digest(),
            "retained-cancellation-chain-authority",
        ),
        super::posture::SpatialSourceAuthorityDigestBasisPosture::RetainedPlanarHistoricalInspectionDigest => admit_compiled_product_authority_truth_identity(
            "worth-spatial",
            admitted_input.source_authority_digest(),
            "retained-planar-historical-inspection",
        ),
    }
    .map_err(|error| {
        SpatialCompiledProductFamilyError::new(
            SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product authority truth admission failed: {error:?}"),
        )
    })
}

fn locality_identity(
    declaration: &SpatialCompiledProductFamilyDeclaration,
    admitted_input: &SpatialCompiledProductFamilyAdmittedInput,
) -> Result<CompiledProductLocalityFootprintIdentity, SpatialCompiledProductFamilyError> {
    let locality = match declaration.locality_footprint_basis() {
        SpatialLocalityFootprintBasisPosture::GroupedBatchFootprintDigest => {
            CompiledProductLocalityFootprintIdentity::grouped_batch_footprint(
                admitted_input.locality_footprint_digest().to_string(),
            )
        }
        SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest => {
            CompiledProductLocalityFootprintIdentity::materialization_target_footprint(
                admitted_input.locality_footprint_digest().to_string(),
            )
        }
        SpatialLocalityFootprintBasisPosture::SpatialTouchDigest => {
            CompiledProductLocalityFootprintIdentity::evidence_neighborhood(
                admitted_input.locality_footprint_digest().to_string(),
            )
        }
    };
    locality.map_err(|error| {
        SpatialCompiledProductFamilyError::new(
            SpatialCompiledProductFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("compiled-product locality admission failed: {error:?}"),
        )
    })
}
