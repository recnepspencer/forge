use crate::domain_artifacts::core_artifact::{impl_hadwiger_artifact, HadwigerArtifactCore};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactKind, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference, HadwigerCanonicalArtifact, HadwigerCheckerCausalEvidence,
    HadwigerQueryDeclarationReference,
};
use crate::domain_declarations::WholePlaneColoringConstructionDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::{admitted_declaration_reference, checker_evidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerPlaneColoringError {
    ZeroDenominator,
    InvalidSideLength,
    InvalidColorRule,
    QueryDeclarationNotAdmitted,
    Artifact(HadwigerArtifactShapeError),
}

impl From<HadwigerArtifactShapeError> for HadwigerPlaneColoringError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexagonalSevenColoringConstruction {
    side_numerator: i128,
    side_denominator: i128,
    color_a: i128,
    color_b: i128,
}

impl HexagonalSevenColoringConstruction {
    pub fn with_side_length_fraction(
        side_numerator: i128,
        side_denominator: i128,
    ) -> Result<Self, HadwigerPlaneColoringError> {
        if side_denominator == 0 {
            return Err(HadwigerPlaneColoringError::ZeroDenominator);
        }
        Ok(Self {
            side_numerator,
            side_denominator,
            color_a: 3,
            color_b: 1,
        })
    }

    pub fn with_color_rule(mut self, color_a: i128, color_b: i128) -> Self {
        self.color_a = color_a;
        self.color_b = color_b;
        self
    }

    fn stable_identity(&self) -> String {
        format!(
            "hex7:{}:{}:{}:{}",
            self.side_numerator, self.side_denominator, self.color_a, self.color_b
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WholePlaneColoringConstruction {
    core: HadwigerArtifactCore,
    color_count: u32,
}

impl WholePlaneColoringConstruction {
    fn new(
        construction: &HexagonalSevenColoringConstruction,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::WholePlaneColoringConstruction,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "hexagonal_seven_coloring_construction".to_string(),
            },
            Vec::new(),
            vec![
                HadwigerArtifactPayloadEntry::text("construction", construction.stable_identity()),
                HadwigerArtifactPayloadEntry::unsigned("color_count", 7),
            ],
        )?;
        Ok(Self {
            core,
            color_count: 7,
        })
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }
}

impl_hadwiger_artifact!(WholePlaneColoringConstruction, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WholePlaneColoringVerification {
    core: HadwigerArtifactCore,
    verified_color_count: u32,
    causal_evidence: HadwigerCheckerCausalEvidence,
    query_declaration_reference: HadwigerQueryDeclarationReference,
}

impl WholePlaneColoringVerification {
    fn admitted(
        construction: &WholePlaneColoringConstruction,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        causal_evidence: HadwigerCheckerCausalEvidence,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let mut payload_entries = vec![
            HadwigerArtifactPayloadEntry::text("checker_identity", "hadwiger.hexagonal_7_checker"),
            HadwigerArtifactPayloadEntry::text("checker_version", "0.1.0"),
            HadwigerArtifactPayloadEntry::text("result_posture", "admitted"),
            HadwigerArtifactPayloadEntry::unsigned("verified_color_count", 7),
            HadwigerArtifactPayloadEntry::text(
                "query_declaration_reference",
                query_declaration_reference.stable_token(),
            ),
        ];
        payload_entries.extend(causal_evidence.payload_entries());
        let core = artifact_core(
            HadwigerArtifactKind::WholePlaneColoringVerification,
            HadwigerArtifactAuthorityOwner::Checker,
            HadwigerArtifactSourceReference::CheckerBoundary {
                checker_identity: "hadwiger.hexagonal_7_checker".to_string(),
                checker_version: "0.1.0".to_string(),
            },
            vec![construction.reference()],
            payload_entries,
        )?;
        Ok(Self {
            core,
            verified_color_count: 7,
            causal_evidence,
            query_declaration_reference,
        })
    }

    pub fn admits_upper_bound_evidence(&self) -> bool {
        self.verified_color_count == 7
    }

    pub fn verified_color_count(&self) -> u32 {
        self.verified_color_count
    }

    pub fn causal_evidence(&self) -> &HadwigerCheckerCausalEvidence {
        &self.causal_evidence
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }
}

impl_hadwiger_artifact!(WholePlaneColoringVerification, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexagonalSevenColoringVerificationChecked {
    construction: WholePlaneColoringConstruction,
    verification: WholePlaneColoringVerification,
}

impl HexagonalSevenColoringVerificationChecked {
    pub fn construction(&self) -> &WholePlaneColoringConstruction {
        &self.construction
    }

    pub fn verification(&self) -> &WholePlaneColoringVerification {
        &self.verification
    }

    pub fn verified_color_count(&self) -> u32 {
        self.verification.verified_color_count()
    }
}

pub fn verify_hexagonal_seven_coloring_checked(
    handle: &HadwigerResearchHandle,
    construction: HexagonalSevenColoringConstruction,
) -> Result<HexagonalSevenColoringVerificationChecked, HadwigerPlaneColoringError> {
    let declared = handle.declare_checked(WholePlaneColoringConstructionDeclaration::new(
        construction.stable_identity(),
        7,
    ));
    let query_declaration_reference = admitted_declaration_reference(declared)
        .ok_or(HadwigerPlaneColoringError::QueryDeclarationNotAdmitted)?;
    let query_identity = format!(
        "{}:{}",
        handle.handle_identity_digest(),
        query_declaration_reference.declaration_digest()
    );
    verify_side_length(&construction)?;
    verify_color_rule(&construction)?;
    let construction_artifact = WholePlaneColoringConstruction::new(&construction)?;
    let verification = WholePlaneColoringVerification::admitted(
        &construction_artifact,
        query_declaration_reference,
        checker_evidence("hexagonal-seven-coloring", &query_identity)?,
    )?;
    Ok(HexagonalSevenColoringVerificationChecked {
        construction: construction_artifact,
        verification,
    })
}

fn verify_side_length(
    construction: &HexagonalSevenColoringConstruction,
) -> Result<(), HadwigerPlaneColoringError> {
    let n = construction.side_numerator;
    let d = construction.side_denominator;
    if n <= 0 || d <= 0 || 2 * n >= d {
        return Err(HadwigerPlaneColoringError::InvalidSideLength);
    }
    if 21 * n * n <= (d + 2 * n) * (d + 2 * n) {
        return Err(HadwigerPlaneColoringError::InvalidSideLength);
    }
    Ok(())
}

fn verify_color_rule(
    construction: &HexagonalSevenColoringConstruction,
) -> Result<(), HadwigerPlaneColoringError> {
    if construction.color_a.rem_euclid(7) != 3 || construction.color_b.rem_euclid(7) != 1 {
        return Err(HadwigerPlaneColoringError::InvalidColorRule);
    }
    for q in -7_i128..=7_i128 {
        for r in -7_i128..=7_i128 {
            if q == 0 && r == 0 {
                continue;
            }
            if (3 * q + r).rem_euclid(7) == 0 {
                let norm = q * q + q * r + r * r;
                if norm < 7 {
                    return Err(HadwigerPlaneColoringError::InvalidColorRule);
                }
            }
        }
    }
    Ok(())
}
