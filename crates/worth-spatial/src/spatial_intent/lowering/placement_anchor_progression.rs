use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker, ProofSetCons,
};

use crate::spatial_intent::lowering::placement_anchor_directions::{
    SpatialPlacementDirectionalAnchorError, SpatialPlacementReorientAnchorMode,
};
use crate::spatial_intent::lowering::placement_anchor_points::{
    SpatialPlacementPointAnchorError, SpatialPlacementPointAnchorKind,
};
use crate::spatial_intent::lowering::placement_anchor_resolution::{
    classify_feature_owned_failure, resolve_axis_world_direction,
    resolve_external_reference_world_point, resolve_feature_axis_world_direction,
    resolve_feature_owned_anchor_world_point, resolve_geometric_tag_anchor_world_point,
    resolve_shape_origin_world_point,
};
use crate::spatial_intent::lowering::SpatialPlacementSpec;
use crate::spatial_intent::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierPointRole, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::SpatialWitnessFailureClass;

type AnchorProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type ClassifiedAnchorProof = Proof<AnchorSemanticsSplitProven, SpatialAnchorProofAuthority>;
type LoweredAnchorProof<P> =
    ProofSetCons<ClassifiedAnchorProof, Proof<P, SpatialAnchorProofAuthority>>;

pub(crate) type LoweredPointAnchorArtifact = Artifact<
    LoweredPointAnchorPhase,
    LoweredPointAnchor,
    LoweredAnchorProof<PointAnchorLoweringProven>,
    AnchorProofBasis,
>;
pub(crate) type LoweredSubjectAnchorArtifact = Artifact<
    LoweredSubjectAnchorPhase,
    LoweredPointAnchor,
    LoweredAnchorProof<SubjectPointAnchorLoweringProven>,
    AnchorProofBasis,
>;
pub(crate) type LoweredTranslationAnchorArtifact = Artifact<
    LoweredTranslationAnchorPhase,
    LoweredPointAnchor,
    LoweredAnchorProof<TranslationAnchorLoweringProven>,
    AnchorProofBasis,
>;
pub(crate) type LoweredReorientAnchorArtifact = Artifact<
    LoweredReorientAnchorPhase,
    SpatialPlacementReorientAnchorMode,
    LoweredAnchorProof<ReorientAnchorLoweringProven>,
    AnchorProofBasis,
>;

pub(crate) struct ClassifiedAnchorPhase;
impl PhaseMarker for ClassifiedAnchorPhase {}

pub(crate) struct LoweredPointAnchorPhase;
impl PhaseMarker for LoweredPointAnchorPhase {}

pub(crate) struct LoweredSubjectAnchorPhase;
impl PhaseMarker for LoweredSubjectAnchorPhase {}

pub(crate) struct LoweredTranslationAnchorPhase;
impl PhaseMarker for LoweredTranslationAnchorPhase {}

pub(crate) struct LoweredReorientAnchorPhase;
impl PhaseMarker for LoweredReorientAnchorPhase {}

pub(crate) struct AnchorSemanticsSplitProven;
impl ProofMarker for AnchorSemanticsSplitProven {}

pub(crate) struct PointAnchorLoweringProven;
impl ProofMarker for PointAnchorLoweringProven {}

pub(crate) struct SubjectPointAnchorLoweringProven;
impl ProofMarker for SubjectPointAnchorLoweringProven {}

pub(crate) struct TranslationAnchorLoweringProven;
impl ProofMarker for TranslationAnchorLoweringProven {}

pub(crate) struct ReorientAnchorLoweringProven;
impl ProofMarker for ReorientAnchorLoweringProven {}

pub(crate) struct SpatialAnchorProofAuthority;
impl AuthorityMarker for SpatialAnchorProofAuthority {}
impl AuthorityProves<AnchorSemanticsSplitProven> for SpatialAnchorProofAuthority {}
impl AuthorityProves<PointAnchorLoweringProven> for SpatialAnchorProofAuthority {}
impl AuthorityProves<SubjectPointAnchorLoweringProven> for SpatialAnchorProofAuthority {}
impl AuthorityProves<TranslationAnchorLoweringProven> for SpatialAnchorProofAuthority {}
impl AuthorityProves<ReorientAnchorLoweringProven> for SpatialAnchorProofAuthority {}

#[derive(Clone, Debug, PartialEq)]
enum SpatialAnchorSemanticClass {
    SubjectOwnedPoint,
    ExternalReferencePoint,
    DirectionalAxis,
    FeatureOwnedReference,
    GeometricTagReference,
    CarrierLocalReference,
}

#[derive(Clone, Debug, PartialEq)]
struct ClassifiedSpatialAnchor {
    anchor: SpatialAnchorRef,
    semantic_class: SpatialAnchorSemanticClass,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LoweredPointAnchor {
    kind: SpatialPlacementPointAnchorKind,
    world_point: [f64; 3],
}

impl LoweredPointAnchor {
    #[allow(dead_code)]
    pub(crate) fn kind(&self) -> SpatialPlacementPointAnchorKind {
        self.kind
    }

    pub(crate) fn world_point(&self) -> [f64; 3] {
        self.world_point
    }
}

type ClassifiedAnchorArtifact = Artifact<
    ClassifiedAnchorPhase,
    ClassifiedSpatialAnchor,
    ClassifiedAnchorProof,
    AnchorProofBasis,
>;

fn anchor_authority() -> AuthorityWitness<SpatialAnchorProofAuthority> {
    AuthorityWitness::from_authority_marker(SpatialAnchorProofAuthority)
}

fn classify_anchor(anchor: &SpatialAnchorRef) -> ClassifiedAnchorArtifact {
    let semantic_class = match anchor {
        SpatialAnchorRef::ShapeOrigin => SpatialAnchorSemanticClass::SubjectOwnedPoint,
        SpatialAnchorRef::WorldOrigin | SpatialAnchorRef::FrameOrigin(_) => {
            SpatialAnchorSemanticClass::ExternalReferencePoint
        }
        SpatialAnchorRef::ShapeAxis(_) | SpatialAnchorRef::FrameAxis { .. } => {
            SpatialAnchorSemanticClass::DirectionalAxis
        }
        SpatialAnchorRef::FeatureOwned(_) => SpatialAnchorSemanticClass::FeatureOwnedReference,
        SpatialAnchorRef::GeometricTag(_) => SpatialAnchorSemanticClass::GeometricTagReference,
        SpatialAnchorRef::ParameterSpace { .. } => {
            SpatialAnchorSemanticClass::CarrierLocalReference
        }
    };
    let authority = anchor_authority();
    Artifact::with_proofs_and_current_basis(
        ClassifiedSpatialAnchor {
            anchor: anchor.clone(),
            semantic_class,
        },
        Proof::from_authority_witness(&authority),
        (),
        authority,
    )
}

pub(crate) fn lower_supported_point_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<LoweredPointAnchorArtifact, SpatialPlacementPointAnchorError> {
    lower_point_anchor_from_classified(placement, &classify_anchor(anchor))
}

pub(crate) fn lower_supported_point_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredPointAnchorArtifact, SpatialPlacementPointAnchorError> {
    lower_point_anchor_from_classified_with_catalog(placement, &classify_anchor(anchor), catalog)
}

pub(crate) fn lower_supported_subject_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredSubjectAnchorArtifact, SpatialPlacementPointAnchorError> {
    lower_subject_anchor_from_classified_with_catalog(placement, &classify_anchor(anchor), catalog)
}

pub(crate) fn lower_supported_translation_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<LoweredTranslationAnchorArtifact, SpatialPlacementPointAnchorError> {
    lower_translation_anchor_from_classified(placement, &classify_anchor(anchor))
}

pub(crate) fn lower_supported_translation_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredTranslationAnchorArtifact, SpatialPlacementPointAnchorError> {
    lower_translation_anchor_from_classified_with_catalog(
        placement,
        &classify_anchor(anchor),
        catalog,
    )
}

pub(crate) fn lower_supported_reorient_anchor(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
) -> Result<LoweredReorientAnchorArtifact, SpatialPlacementDirectionalAnchorError> {
    lower_reorient_anchor_from_classified(placement, &classify_anchor(anchor))
}

pub(crate) fn lower_supported_reorient_anchor_with_catalog(
    placement: &SpatialPlacementSpec,
    anchor: &SpatialAnchorRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchorArtifact, SpatialPlacementDirectionalAnchorError> {
    lower_reorient_anchor_from_classified_with_catalog(placement, &classify_anchor(anchor), catalog)
}

fn lower_point_anchor_from_classified(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
) -> Result<LoweredPointAnchorArtifact, SpatialPlacementPointAnchorError> {
    let anchor = classified.payload();
    match anchor.semantic_class {
        SpatialAnchorSemanticClass::SubjectOwnedPoint => lowered_point_anchor(
            SpatialPlacementPointAnchorKind::SubjectOwnedPoint,
            resolve_shape_origin_world_point(placement)?,
        ),
        SpatialAnchorSemanticClass::ExternalReferencePoint => lowered_point_anchor(
            SpatialPlacementPointAnchorKind::ExternalReferencePoint,
            resolve_external_reference_world_point(&anchor.anchor)?,
        ),
        SpatialAnchorSemanticClass::DirectionalAxis
        | SpatialAnchorSemanticClass::FeatureOwnedReference
        | SpatialAnchorSemanticClass::GeometricTagReference
        | SpatialAnchorSemanticClass::CarrierLocalReference => {
            Err(SpatialPlacementPointAnchorError::UnsupportedAnchor)
        }
    }
}

fn lower_point_anchor_from_classified_with_catalog(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredPointAnchorArtifact, SpatialPlacementPointAnchorError> {
    let anchor = classified.payload();
    match anchor.semantic_class {
        SpatialAnchorSemanticClass::FeatureOwnedReference => lowered_point_anchor(
            SpatialPlacementPointAnchorKind::FeatureOwnedPoint,
            resolve_feature_owned_anchor_world_point(&anchor.anchor, catalog)?,
        ),
        SpatialAnchorSemanticClass::GeometricTagReference => lowered_point_anchor(
            SpatialPlacementPointAnchorKind::GeometricTagPoint,
            resolve_geometric_tag_anchor_world_point(&anchor.anchor, catalog)?,
        ),
        _ => lower_point_anchor_from_classified(placement, classified),
    }
}

fn lower_subject_anchor_from_classified_with_catalog(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredSubjectAnchorArtifact, SpatialPlacementPointAnchorError> {
    let anchor = classified.payload();
    let lowered = match anchor.semantic_class {
        SpatialAnchorSemanticClass::SubjectOwnedPoint => LoweredPointAnchor {
            kind: SpatialPlacementPointAnchorKind::SubjectOwnedPoint,
            world_point: resolve_shape_origin_world_point(placement)?,
        },
        SpatialAnchorSemanticClass::FeatureOwnedReference => LoweredPointAnchor {
            kind: SpatialPlacementPointAnchorKind::FeatureOwnedPoint,
            world_point: resolve_feature_owned_anchor_world_point(&anchor.anchor, catalog)?,
        },
        SpatialAnchorSemanticClass::GeometricTagReference => LoweredPointAnchor {
            kind: SpatialPlacementPointAnchorKind::GeometricTagPoint,
            world_point: resolve_geometric_tag_anchor_world_point(&anchor.anchor, catalog)?,
        },
        SpatialAnchorSemanticClass::ExternalReferencePoint
        | SpatialAnchorSemanticClass::DirectionalAxis
        | SpatialAnchorSemanticClass::CarrierLocalReference => {
            return Err(SpatialPlacementPointAnchorError::UnsupportedAnchor);
        }
    };
    let authority = anchor_authority();
    Ok(Artifact::with_proofs_and_current_basis(
        lowered,
        ProofSetCons::new(
            Proof::from_authority_witness(&authority),
            Proof::from_authority_witness(&authority),
        ),
        (),
        authority,
    ))
}

fn lower_translation_anchor_from_classified(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
) -> Result<LoweredTranslationAnchorArtifact, SpatialPlacementPointAnchorError> {
    let lowered = lower_point_anchor_from_classified(placement, classified)?
        .payload()
        .to_owned();
    let authority = anchor_authority();
    Ok(Artifact::with_proofs_and_current_basis(
        lowered,
        ProofSetCons::new(
            Proof::from_authority_witness(&authority),
            Proof::from_authority_witness(&authority),
        ),
        (),
        authority,
    ))
}

fn lower_translation_anchor_from_classified_with_catalog(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredTranslationAnchorArtifact, SpatialPlacementPointAnchorError> {
    let lowered = lower_point_anchor_from_classified_with_catalog(placement, classified, catalog)?
        .payload()
        .to_owned();
    let authority = anchor_authority();
    Ok(Artifact::with_proofs_and_current_basis(
        lowered,
        ProofSetCons::new(
            Proof::from_authority_witness(&authority),
            Proof::from_authority_witness(&authority),
        ),
        (),
        authority,
    ))
}

fn lower_reorient_anchor_from_classified(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
) -> Result<LoweredReorientAnchorArtifact, SpatialPlacementDirectionalAnchorError> {
    let lowered = match classified.payload().semantic_class {
        SpatialAnchorSemanticClass::SubjectOwnedPoint => {
            SpatialPlacementReorientAnchorMode::PointLike
        }
        SpatialAnchorSemanticClass::ExternalReferencePoint => {
            SpatialPlacementReorientAnchorMode::PointLike
        }
        SpatialAnchorSemanticClass::DirectionalAxis => {
            SpatialPlacementReorientAnchorMode::Directional(resolve_axis_world_direction(
                placement,
                &classified.payload().anchor,
            )?)
        }
        SpatialAnchorSemanticClass::FeatureOwnedReference
        | SpatialAnchorSemanticClass::GeometricTagReference
        | SpatialAnchorSemanticClass::CarrierLocalReference => {
            return Err(SpatialPlacementDirectionalAnchorError::UnsupportedAnchor);
        }
    };
    lowered_reorient_anchor(lowered)
}

fn lower_reorient_anchor_from_classified_with_catalog(
    placement: &SpatialPlacementSpec,
    classified: &ClassifiedAnchorArtifact,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<LoweredReorientAnchorArtifact, SpatialPlacementDirectionalAnchorError> {
    let lowered = match classified.payload().semantic_class {
        SpatialAnchorSemanticClass::FeatureOwnedReference => {
            let feature = match &classified.payload().anchor {
                SpatialAnchorRef::FeatureOwned(feature) => feature,
                _ => unreachable!("feature-owned semantic class must carry feature anchor"),
            };
            let point_result =
                catalog.resolve_feature_owned_point(feature, SpatialCarrierPointRole::Anchor);
            let direction_result =
                catalog.resolve_feature_owned_direction(feature, SpatialCarrierDirectionRole::Axis);
            match (point_result, direction_result) {
                (Ok(_), Ok(_)) => {
                    return Err(SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning)
                }
                (Ok(_), Err(_)) => SpatialPlacementReorientAnchorMode::PointLike,
                (Err(SpatialWitnessFailureClass::Unsupported), Ok(_)) => {
                    SpatialPlacementReorientAnchorMode::Directional(
                        resolve_feature_axis_world_direction(feature.clone(), catalog)?,
                    )
                }
                (Err(SpatialWitnessFailureClass::Ambiguous), Ok(_)) => {
                    return Err(SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning)
                }
                (Err(point_error), Ok(_)) => {
                    return Err(
                        SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(point_error),
                    );
                }
                (Err(point_error), Err(direction_error)) => {
                    return Err(classify_feature_owned_failure(point_error, direction_error));
                }
            }
        }
        _ => return lower_reorient_anchor_from_classified(placement, classified),
    };
    lowered_reorient_anchor(lowered)
}

fn lowered_point_anchor(
    kind: SpatialPlacementPointAnchorKind,
    world_point: [f64; 3],
) -> Result<LoweredPointAnchorArtifact, SpatialPlacementPointAnchorError> {
    let authority = anchor_authority();
    Ok(Artifact::with_proofs_and_current_basis(
        LoweredPointAnchor { kind, world_point },
        ProofSetCons::new(
            Proof::from_authority_witness(&authority),
            Proof::from_authority_witness(&authority),
        ),
        (),
        authority,
    ))
}

fn lowered_reorient_anchor(
    mode: SpatialPlacementReorientAnchorMode,
) -> Result<LoweredReorientAnchorArtifact, SpatialPlacementDirectionalAnchorError> {
    let authority = anchor_authority();
    Ok(Artifact::with_proofs_and_current_basis(
        mode,
        ProofSetCons::new(
            Proof::from_authority_witness(&authority),
            Proof::from_authority_witness(&authority),
        ),
        (),
        authority,
    ))
}
