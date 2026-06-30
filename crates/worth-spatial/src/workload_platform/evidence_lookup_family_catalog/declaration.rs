use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictOverlapIdentityInput,
    ConflictPriorProofInput, ConflictRoutingContract, ConflictRoutingPosture,
    ConflictRoutingVocabularyError,
};
use schema::facade::platform::authority::touched_graph_conflict_internal::admit_conflict_evidence_participant_identity_from_digest;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};
use super::family_identity::EvidenceLookupFamilyIdentity;
use super::posture::{
    EvidenceLookupDiagnosticWitnessShape, EvidenceLookupEvidenceClassSet,
    EvidenceLookupFamilyIndexPosture, EvidenceLookupFamilyQueryPosture,
    EvidenceLookupTopologyInputPosture,
};
use super::source_pressure::EvidenceLookupFamilySourceInventoryPressure;
use super::stage_applicability::EvidenceLookupStageApplicability;
use crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority;
use crate::touched_graph_conflict::{
    current_spatial_conflict_family_catalog_closeout, SpatialConflictFamilyApplicability,
    SpatialConflictFamilyIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSpatialTouchAuthorityRequirement {
    SealedSpatialTouchAuthorityRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupProductPosture {
    DeclarationOnlySelectionRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyDeclarationDigest(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilyDeclaration {
    identity: EvidenceLookupFamilyIdentity,
    spatial_touch_authority: EvidenceLookupSpatialTouchAuthorityRequirement,
    topology_input_posture: EvidenceLookupTopologyInputPosture,
    stage_applicability: EvidenceLookupStageApplicability,
    evidence_classes: EvidenceLookupEvidenceClassSet,
    lookup_product_posture: EvidenceLookupProductPosture,
    index_posture: EvidenceLookupFamilyIndexPosture,
    query_posture: EvidenceLookupFamilyQueryPosture,
    diagnostic_witness: EvidenceLookupDiagnosticWitnessShape,
    source_inventory_pressure: EvidenceLookupFamilySourceInventoryPressure,
    declaration_digest: EvidenceLookupFamilyDeclarationDigest,
}

impl EvidenceLookupFamilyDeclaration {
    pub(crate) fn builder() -> EvidenceLookupFamilyDeclarationBuilder {
        EvidenceLookupFamilyDeclarationBuilder::default()
    }

    pub fn identity(&self) -> &EvidenceLookupFamilyIdentity {
        &self.identity
    }

    pub const fn spatial_touch_authority(&self) -> EvidenceLookupSpatialTouchAuthorityRequirement {
        self.spatial_touch_authority
    }

    pub const fn lookup_product_posture(&self) -> EvidenceLookupProductPosture {
        self.lookup_product_posture
    }

    pub const fn topology_input_posture(&self) -> &EvidenceLookupTopologyInputPosture {
        &self.topology_input_posture
    }

    pub const fn stage_applicability(&self) -> &EvidenceLookupStageApplicability {
        &self.stage_applicability
    }

    pub const fn evidence_classes(&self) -> &EvidenceLookupEvidenceClassSet {
        &self.evidence_classes
    }

    pub const fn index_posture(&self) -> &EvidenceLookupFamilyIndexPosture {
        &self.index_posture
    }

    pub const fn query_posture(&self) -> &EvidenceLookupFamilyQueryPosture {
        &self.query_posture
    }

    pub const fn diagnostic_witness(&self) -> &EvidenceLookupDiagnosticWitnessShape {
        &self.diagnostic_witness
    }

    pub const fn source_inventory_pressure(&self) -> &EvidenceLookupFamilySourceInventoryPressure {
        &self.source_inventory_pressure
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_str()
    }

    pub fn authority_digest(&self) -> &EvidenceLookupFamilyDeclarationDigest {
        &self.declaration_digest
    }

    pub fn conflict_participant_identity(
        &self,
    ) -> Result<
        schema::facade::platform::authority::touched_graph_conflict::ConflictParticipantIdentity,
        ConflictRoutingVocabularyError,
    > {
        admit_conflict_evidence_participant_identity_from_digest(self.authority_digest())
    }

    pub(crate) fn conflict_routing_contract(
        &self,
        authority: &SpatialGeometryEvidenceTouchAuthority,
    ) -> Result<ConflictRoutingContract, ConflictRoutingVocabularyError> {
        let locality = authority.conflict_locality_identity()?;
        let participants = vec![
            authority.conflict_participant_identity()?,
            self.conflict_participant_identity()?,
        ];
        let overlap = admit_conflict_overlap_identity(ConflictOverlapIdentityInput::evidence(
            locality,
            participants,
        ))?;
        Ok(admit_conflict_routing_contract(
            overlap,
            ConflictPriorProofInput::none(),
            ConflictRoutingPosture::RequiresFamilySelection,
        ))
    }

    pub(crate) fn matching_conflict_family_identities(
        &self,
        authority: &SpatialGeometryEvidenceTouchAuthority,
    ) -> Result<Vec<SpatialConflictFamilyIdentity>, ConflictRoutingVocabularyError> {
        let contract = self.conflict_routing_contract(authority)?;
        Ok(self.matching_conflict_family_identities_for_contract(authority, &contract))
    }

    pub(crate) fn matching_conflict_family_identities_for_contract(
        &self,
        authority: &SpatialGeometryEvidenceTouchAuthority,
        contract: &ConflictRoutingContract,
    ) -> Vec<SpatialConflictFamilyIdentity> {
        let closeout = current_spatial_conflict_family_catalog_closeout()
            .expect("spatial conflict family catalog closes");
        closeout
            .catalog()
            .matching_families(
                contract,
                SpatialConflictFamilyApplicability::EvidenceLookup { authority },
            )
            .into_iter()
            .map(|declaration| declaration.identity())
            .collect()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EvidenceLookupFamilyDeclarationBuilder {
    identity: Option<EvidenceLookupFamilyIdentity>,
    spatial_touch_authority: Option<EvidenceLookupSpatialTouchAuthorityRequirement>,
    topology_input_posture: Option<EvidenceLookupTopologyInputPosture>,
    stage_applicability: Option<EvidenceLookupStageApplicability>,
    evidence_classes: Option<EvidenceLookupEvidenceClassSet>,
    lookup_product_posture: Option<EvidenceLookupProductPosture>,
    index_posture: Option<EvidenceLookupFamilyIndexPosture>,
    query_posture: Option<EvidenceLookupFamilyQueryPosture>,
    diagnostic_witness: Option<EvidenceLookupDiagnosticWitnessShape>,
    source_inventory_pressure: Option<EvidenceLookupFamilySourceInventoryPressure>,
}

impl EvidenceLookupFamilyDeclarationBuilder {
    pub(crate) fn identity(mut self, value: EvidenceLookupFamilyIdentity) -> Self {
        self.identity = Some(value);
        self
    }

    pub(crate) const fn spatial_touch_authority(
        mut self,
        value: EvidenceLookupSpatialTouchAuthorityRequirement,
    ) -> Self {
        self.spatial_touch_authority = Some(value);
        self
    }

    pub(crate) fn topology_input_posture(
        mut self,
        value: EvidenceLookupTopologyInputPosture,
    ) -> Self {
        self.topology_input_posture = Some(value);
        self
    }

    pub(crate) fn stage_applicability(mut self, value: EvidenceLookupStageApplicability) -> Self {
        self.stage_applicability = Some(value);
        self
    }

    pub(crate) fn evidence_classes(mut self, value: EvidenceLookupEvidenceClassSet) -> Self {
        self.evidence_classes = Some(value);
        self
    }

    pub(crate) const fn lookup_product_posture(
        mut self,
        value: EvidenceLookupProductPosture,
    ) -> Self {
        self.lookup_product_posture = Some(value);
        self
    }

    pub(crate) fn index_posture(mut self, value: EvidenceLookupFamilyIndexPosture) -> Self {
        self.index_posture = Some(value);
        self
    }

    pub(crate) fn query_posture(mut self, value: EvidenceLookupFamilyQueryPosture) -> Self {
        self.query_posture = Some(value);
        self
    }

    pub(crate) fn diagnostic_witness(
        mut self,
        value: EvidenceLookupDiagnosticWitnessShape,
    ) -> Self {
        self.diagnostic_witness = Some(value);
        self
    }

    pub(crate) fn source_inventory_pressure(
        mut self,
        value: EvidenceLookupFamilySourceInventoryPressure,
    ) -> Self {
        self.source_inventory_pressure = Some(value);
        self
    }

    pub(crate) fn build(
        self,
    ) -> Result<EvidenceLookupFamilyDeclaration, EvidenceLookupFamilyCatalogError> {
        let identity = self
            .identity
            .ok_or_else(|| missing(EvidenceLookupFamilyCatalogErrorKind::MissingFamilyIdentity))?;
        let spatial_touch_authority = self.spatial_touch_authority.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingSpatialTouchAuthority)
        })?;
        let topology_input_posture = self.topology_input_posture.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingTopologyInputPosture)
        })?;
        let stage_applicability = self.stage_applicability.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingStageApplicability)
        })?;
        let evidence_classes = self
            .evidence_classes
            .ok_or_else(|| missing(EvidenceLookupFamilyCatalogErrorKind::MissingEvidenceClass))?;
        let lookup_product_posture = self.lookup_product_posture.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingLookupProductPosture)
        })?;
        let index_posture = self
            .index_posture
            .ok_or_else(|| missing(EvidenceLookupFamilyCatalogErrorKind::MissingIndexPosture))?;
        let query_posture = self
            .query_posture
            .ok_or_else(|| missing(EvidenceLookupFamilyCatalogErrorKind::MissingQueryPosture))?;
        let diagnostic_witness = self.diagnostic_witness.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingDiagnosticWitness)
        })?;
        let source_inventory_pressure = self.source_inventory_pressure.ok_or_else(|| {
            missing(EvidenceLookupFamilyCatalogErrorKind::MissingSourceInventoryPressure)
        })?;
        let declaration_digest = EvidenceLookupFamilyDeclarationDigest::new(declaration_digest(
            &identity,
            &topology_input_posture,
            &stage_applicability,
            &evidence_classes,
            &index_posture,
            &query_posture,
            &diagnostic_witness,
            &source_inventory_pressure,
        ));
        Ok(EvidenceLookupFamilyDeclaration {
            identity,
            spatial_touch_authority,
            topology_input_posture,
            stage_applicability,
            evidence_classes,
            lookup_product_posture,
            index_posture,
            query_posture,
            diagnostic_witness,
            source_inventory_pressure,
            declaration_digest,
        })
    }
}

impl EvidenceLookupFamilyDeclarationDigest {
    fn new(digest: String) -> Self {
        Self(digest)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for EvidenceLookupFamilyDeclarationDigest {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

fn declaration_digest(
    identity: &EvidenceLookupFamilyIdentity,
    topology: &EvidenceLookupTopologyInputPosture,
    stages: &EvidenceLookupStageApplicability,
    classes: &EvidenceLookupEvidenceClassSet,
    index: &EvidenceLookupFamilyIndexPosture,
    query: &EvidenceLookupFamilyQueryPosture,
    diagnostic: &EvidenceLookupDiagnosticWitnessShape,
    source_inventory_pressure: &EvidenceLookupFamilySourceInventoryPressure,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-family-declaration:v1".to_string(),
            identity.digest().to_string(),
            format!("{:?}:{:?}", topology.state(), topology.required_family()),
            format!("stages:{:?}", stages.stages()),
            format!("classes:{:?}", classes.classes()),
            format!("index:{:?}", index.kind()),
            format!(
                "query:{:?}:{:?}",
                query.kind(),
                query.imported_evidence_digest()
            ),
            format!("diagnostic:{:?}", diagnostic.kind()),
            format!(
                "inventory-pressure:{}",
                source_inventory_pressure.pressure_digest_basis()
            ),
        ],
    )
}

const fn missing(kind: EvidenceLookupFamilyCatalogErrorKind) -> EvidenceLookupFamilyCatalogError {
    EvidenceLookupFamilyCatalogError::new(kind)
}
