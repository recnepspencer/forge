use super::architecture_claim::{TouchedGraphParityArchitectureClaim, TouchedGraphParityClaimKind};
use super::error::{TouchedGraphParityReadinessError, TouchedGraphParityReadinessErrorKind};
use super::family_kind::TouchedGraphParityFamilyKind;
use super::residue_classification::TouchedGraphParityResidueClassification;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchedGraphParityReadinessInput {
    claim: TouchedGraphParityArchitectureClaim,
    residue_classification: TouchedGraphParityResidueClassification,
    touched_closure_digest: String,
    overlap_identity_digests: Vec<String>,
    representative_family_coverage: Vec<TouchedGraphParityFamilyKind>,
    topology_query_posture_digest: String,
    spatial_query_posture_digest: String,
    residue_digest: String,
    source_firewall_digest: String,
    architecture_claim_digest: String,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn admit_touched_graph_parity_readiness_input(
    claim: TouchedGraphParityArchitectureClaim,
    residue_classification: TouchedGraphParityResidueClassification,
    touched_closure_digest: impl Into<String>,
    overlap_identity_digests: Vec<String>,
    representative_family_coverage: Vec<TouchedGraphParityFamilyKind>,
    topology_query_posture_digest: impl Into<String>,
    spatial_query_posture_digest: impl Into<String>,
    residue_digest: impl Into<String>,
    source_firewall_digest: impl Into<String>,
    architecture_claim_digest: impl Into<String>,
) -> Result<TouchedGraphParityReadinessInput, TouchedGraphParityReadinessError> {
    let touched_closure_digest = touched_closure_digest.into();
    let topology_query_posture_digest = topology_query_posture_digest.into();
    let spatial_query_posture_digest = spatial_query_posture_digest.into();
    let residue_digest = residue_digest.into();
    let source_firewall_digest = source_firewall_digest.into();
    let architecture_claim_digest = architecture_claim_digest.into();

    if claim.kind() != TouchedGraphParityClaimKind::ReadinessParity {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::ClaimKindMustBeReadinessParity,
            "readiness input requires a readiness-parity claim rather than a lower parity kind",
        ));
    }
    if claim
        .selected_route_identity()
        .identity_digest()
        .trim()
        .is_empty()
    {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingSelectedRouteIdentity,
            "readiness input requires the carried selected-route identity digest",
        ));
    }
    if claim
        .selected_family_identity()
        .selected_family_name()
        .trim()
        .is_empty()
    {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingSelectedFamilyIdentity,
            "readiness input requires the carried selected-family identity",
        ));
    }
    let selected_product_identity = claim.selected_product_identity().ok_or_else(|| {
        TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingSelectedProductIdentity,
            "readiness input requires the carried selected-product identity digest",
        )
    })?;
    if selected_product_identity
        .identity_digest()
        .trim()
        .is_empty()
    {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingSelectedProductIdentity,
            "readiness input requires the carried selected-product identity digest",
        ));
    }
    if touched_closure_digest.trim().is_empty() && overlap_identity_digests.is_empty() {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingTouchedOrOverlapIdentity,
            "readiness input requires touched or overlap identity carried from selected-route authority",
        ));
    }
    if representative_family_coverage.is_empty() {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingRepresentativeFamilyCoverage,
            "readiness input requires explicit representative family coverage",
        ));
    }
    if topology_query_posture_digest.trim().is_empty()
        || spatial_query_posture_digest.trim().is_empty()
    {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingQueryPostureEvidence,
            "readiness input requires topology and spatial Query posture evidence digests",
        ));
    }
    if residue_digest.trim().is_empty() || source_firewall_digest.trim().is_empty() {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingResidueOrFirewallDigest,
            "readiness input requires carried residue and source-firewall digests",
        ));
    }
    if architecture_claim_digest.trim().is_empty() {
        return Err(TouchedGraphParityReadinessError::new(
            TouchedGraphParityReadinessErrorKind::MissingArchitectureClaimDigest,
            "readiness input requires the closed architecture-claim digest",
        ));
    }

    Ok(TouchedGraphParityReadinessInput {
        claim,
        residue_classification,
        touched_closure_digest,
        overlap_identity_digests,
        representative_family_coverage,
        topology_query_posture_digest,
        spatial_query_posture_digest,
        residue_digest,
        source_firewall_digest,
        architecture_claim_digest,
    })
}

impl TouchedGraphParityReadinessInput {
    /// ```compile_fail
    /// use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
    ///
    /// let _constructor = TouchedGraphParityReadinessInput {
    ///     claim: unsafe { std::mem::zeroed() },
    ///     residue_classification: unsafe { std::mem::zeroed() },
    ///     touched_closure_digest: String::new(),
    ///     overlap_identity_digests: Vec::new(),
    ///     representative_family_coverage: Vec::new(),
    ///     topology_query_posture_digest: String::new(),
    ///     spatial_query_posture_digest: String::new(),
    ///     residue_digest: String::new(),
    ///     source_firewall_digest: String::new(),
    ///     architecture_claim_digest: String::new(),
    /// };
    /// ```
    pub fn claim(&self) -> &TouchedGraphParityArchitectureClaim {
        &self.claim
    }

    pub const fn residue_classification(&self) -> TouchedGraphParityResidueClassification {
        self.residue_classification
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        self.claim.selected_route_identity().identity_digest()
    }

    pub fn selected_family_identity(&self) -> &str {
        self.claim.selected_family_identity().selected_family_name()
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        self.claim
            .selected_product_identity()
            .expect("readiness input requires selected-product identity")
            .identity_digest()
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.claim
            .witness_identity()
            .map(|identity| identity.identity_digest())
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn representative_family_coverage(&self) -> &[TouchedGraphParityFamilyKind] {
        &self.representative_family_coverage
    }

    pub fn topology_query_posture_digest(&self) -> &str {
        &self.topology_query_posture_digest
    }

    pub fn spatial_query_posture_digest(&self) -> &str {
        &self.spatial_query_posture_digest
    }

    pub fn residue_digest(&self) -> &str {
        &self.residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }
}
