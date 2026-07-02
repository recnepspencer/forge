use crate::declaration::declaration_handoff::derive_declaration_graph_handoff;
use crate::declaration::{
    UiAspectContract, UiAspectContractAdmission, UiAspectContractAdmissionDenial,
    UiAspectCoverageReport, UiDeclarationArtifactDigest, UiDeclarationDigestProjection,
    UiDeclarationFamily, UiDeclarationFamilyAdmission, UiDeclarationFamilyAdmissionDenial,
    UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial, UiDeclarationIdentity,
    UiDeclarationProvenance, UiDeclarationStructuralSemantics,
    UiDeclarationStructuralSemanticsAdmission, UiDeclarationStructuralSemanticsAdmissionDenial,
    UiDeclarationSupportSnapshot, UiDeclarationSupportSnapshotAdmission,
    UiDeclarationSupportSnapshotAdmissionDenial, UiDeclaredPostureAdmission,
    UiDeclaredPostureAdmissionDenial, UiDeclaredPostureContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationArtifact {
    identity: UiDeclarationIdentity,
    digests: UiDeclarationDigestProjection,
    aspect_contract_admission: UiAspectContractAdmission,
    declared_posture_admission: UiDeclaredPostureAdmission,
    declaration_support_snapshot_admission: UiDeclarationSupportSnapshotAdmission,
    structural_semantics_admission: UiDeclarationStructuralSemanticsAdmission,
    family_admission: UiDeclarationFamilyAdmission,
    provenance: UiDeclarationProvenance,
}

impl UiDeclarationArtifact {
    pub(crate) fn new(
        identity: UiDeclarationIdentity,
        digests: UiDeclarationDigestProjection,
        aspect_contract_admission: UiAspectContractAdmission,
        declared_posture_admission: UiDeclaredPostureAdmission,
        declaration_support_snapshot_admission: UiDeclarationSupportSnapshotAdmission,
        structural_semantics_admission: UiDeclarationStructuralSemanticsAdmission,
        family_admission: UiDeclarationFamilyAdmission,
        provenance: UiDeclarationProvenance,
    ) -> Self {
        Self {
            identity,
            digests,
            aspect_contract_admission,
            declared_posture_admission,
            declaration_support_snapshot_admission,
            structural_semantics_admission,
            family_admission,
            provenance,
        }
    }

    pub fn identity(&self) -> &UiDeclarationIdentity {
        &self.identity
    }

    pub fn artifact_digest(&self) -> UiDeclarationArtifactDigest {
        self.digests.artifact()
    }

    pub fn identity_digest(&self) -> crate::declaration::UiDeclarationIdentityDigest {
        self.digests.identity()
    }

    pub fn digest_projection(&self) -> &UiDeclarationDigestProjection {
        &self.digests
    }

    #[cfg(test)]
    pub(crate) fn aspect_contract_admission(&self) -> &UiAspectContractAdmission {
        &self.aspect_contract_admission
    }

    pub fn aspect_contract(&self) -> Result<&UiAspectContract, &UiAspectContractAdmissionDenial> {
        self.aspect_contract_admission.admitted_contract()
    }

    pub fn aspect_coverage_report(
        &self,
    ) -> Result<UiAspectCoverageReport, &UiAspectContractAdmissionDenial> {
        self.aspect_contract()
            .map(UiAspectContract::coverage_report)
    }

    #[cfg(test)]
    pub(crate) fn declared_posture_admission(&self) -> &UiDeclaredPostureAdmission {
        &self.declared_posture_admission
    }

    pub fn declared_posture(
        &self,
    ) -> Result<&UiDeclaredPostureContract, &UiDeclaredPostureAdmissionDenial> {
        self.declared_posture_admission.admitted_contract()
    }

    #[cfg(test)]
    pub(crate) fn support_snapshot_admission(&self) -> &UiDeclarationSupportSnapshotAdmission {
        &self.declaration_support_snapshot_admission
    }

    pub fn support_snapshot(
        &self,
    ) -> Result<&UiDeclarationSupportSnapshot, &UiDeclarationSupportSnapshotAdmissionDenial> {
        self.declaration_support_snapshot_admission
            .admitted_snapshot()
    }

    #[cfg(test)]
    pub(crate) fn structural_semantics_admission(
        &self,
    ) -> &UiDeclarationStructuralSemanticsAdmission {
        &self.structural_semantics_admission
    }

    pub fn structural_semantics(
        &self,
    ) -> Result<&UiDeclarationStructuralSemantics, &UiDeclarationStructuralSemanticsAdmissionDenial>
    {
        self.structural_semantics_admission
            .admitted_structural_semantics()
    }

    pub fn graph_handoff(
        &self,
    ) -> Result<UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial> {
        let aspect_contract = self.aspect_contract().map_err(|denial| {
            UiDeclarationGraphHandoffDenial::AspectContractNotAdmitted {
                denial: denial.clone(),
            }
        })?;
        let family =
            self.family().map_err(
                |denial| UiDeclarationGraphHandoffDenial::FamilyNotAdmitted {
                    denial: denial.clone(),
                },
            )?;
        let semantics = self.structural_semantics().map_err(|denial| {
            UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                denial: denial.clone(),
            }
        })?;
        let declared_posture = self.declared_posture().map_err(|denial| {
            UiDeclarationGraphHandoffDenial::DeclaredPostureNotAdmitted {
                denial: denial.clone(),
            }
        })?;

        Ok(derive_declaration_graph_handoff(
            &self.identity,
            &self.provenance,
            aspect_contract,
            family,
            semantics,
            declared_posture,
        ))
    }

    #[cfg(test)]
    pub(crate) fn structural_handoff(
        &self,
    ) -> Result<UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial> {
        self.graph_handoff()
    }

    #[cfg(test)]
    pub(crate) fn family_admission(&self) -> &UiDeclarationFamilyAdmission {
        &self.family_admission
    }

    pub fn family(&self) -> Result<&UiDeclarationFamily, &UiDeclarationFamilyAdmissionDenial> {
        self.family_admission.admitted_family()
    }

    pub fn provenance(&self) -> &UiDeclarationProvenance {
        &self.provenance
    }
}
