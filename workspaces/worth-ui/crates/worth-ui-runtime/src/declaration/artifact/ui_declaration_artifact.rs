use crate::capability::MosaicSizingContractId;
use crate::declaration::declaration_handoff::derive_declaration_graph_handoff;
use crate::declaration::{
    UiAspectContract, UiAspectContractAdmission, UiAspectContractAdmissionDenial,
    UiAspectCoverageReport, UiDeclarationArtifactDigest, UiDeclarationContainmentIntent,
    UiDeclarationDigestProjection, UiDeclarationFamily, UiDeclarationFamilyAdmission,
    UiDeclarationFamilyAdmissionDenial, UiDeclarationGraphHandoff,
    UiDeclarationGraphHandoffDenial, UiDeclarationIdentity, UiDeclarationProvenance,
    UiDeclarationStructuralSemantics,
    UiDeclarationStructuralSemanticsAdmission, UiDeclarationStructuralSemanticsAdmissionDenial,
    UiDeclarationSupportSnapshot, UiDeclarationSupportSnapshotAdmission,
    UiDeclarationSupportSnapshotAdmissionDenial, UiDeclaredPostureAdmission,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementPolicyPosture,
    UiDeclaredPostureAdmissionDenial, UiDeclaredPostureContract, UiDeclaredPostureLane,
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
    source_backed_mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    source_backed_mosaic_membership_name: Option<Box<str>>,
    source_backed_measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
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
            source_backed_mosaic_sizing_contract_id: None,
            source_backed_mosaic_membership_name: None,
            source_backed_measurement_constraint_modifier: None,
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
        let admitted = self.admitted_graph_handoff_inputs()?;
        let semantics = self.effective_structural_semantics(admitted.semantics)?;
        let declared_posture = self.effective_declared_posture(admitted.declared_posture);

        Ok(derive_declaration_graph_handoff(
            &self.identity,
            &self.provenance,
            admitted.aspect_contract,
            admitted.family,
            self.digests.structural(),
            &semantics,
            &declared_posture,
        ))
    }

    pub(crate) fn admit_source_backed_mosaic_sizing_contract_id(
        &mut self,
        source_mosaic_sizing_contract_id: MosaicSizingContractId,
    ) -> Result<(), UiDeclarationGraphHandoffDenial> {
        if let Some(declared_mosaic_sizing_contract_id) = self
            .structural_semantics()
            .map_err(
                |denial| UiDeclarationGraphHandoffDenial::StructuralSemanticsNotAdmitted {
                    denial: denial.clone(),
                },
            )?
            .mosaic_sizing_contract_id()
        {
            if declared_mosaic_sizing_contract_id != &source_mosaic_sizing_contract_id {
                return Err(
                    UiDeclarationGraphHandoffDenial::SourceBackedMosaicSizingContractConflict {
                        declared: declared_mosaic_sizing_contract_id.clone(),
                        sourced: source_mosaic_sizing_contract_id,
                    },
                );
            }
        }
        self.source_backed_mosaic_sizing_contract_id = Some(source_mosaic_sizing_contract_id);
        Ok(())
    }

    pub(crate) fn admit_source_backed_mosaic_membership_name(
        &mut self,
        source_mosaic_membership_name: impl Into<Box<str>>,
    ) {
        self.source_backed_mosaic_membership_name = Some(source_mosaic_membership_name.into());
    }

    pub(crate) fn admit_source_backed_measurement_constraint_modifier(
        &mut self,
        modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    ) {
        self.source_backed_measurement_constraint_modifier = modifier;
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

struct AdmittedGraphHandoffInputs<'a> {
    aspect_contract: &'a UiAspectContract,
    family: &'a UiDeclarationFamily,
    semantics: &'a UiDeclarationStructuralSemantics,
    declared_posture: &'a UiDeclaredPostureContract,
}

impl UiDeclarationArtifact {
    fn effective_structural_semantics(
        &self,
        admitted: &UiDeclarationStructuralSemantics,
    ) -> Result<UiDeclarationStructuralSemantics, UiDeclarationGraphHandoffDenial> {
        let Some(source_backed_mosaic_sizing_contract_id) =
            self.source_backed_mosaic_sizing_contract_id.clone()
        else {
            return Ok(self.override_source_backed_membership(admitted, None));
        };

        Ok(self.override_source_backed_membership(
            admitted,
            Some(source_backed_mosaic_sizing_contract_id),
        ))
    }

    fn admitted_graph_handoff_inputs(
        &self,
    ) -> Result<AdmittedGraphHandoffInputs<'_>, UiDeclarationGraphHandoffDenial> {
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

        Ok(AdmittedGraphHandoffInputs {
            aspect_contract,
            family,
            semantics,
            declared_posture,
        })
    }

    fn override_source_backed_membership(
        &self,
        admitted: &UiDeclarationStructuralSemantics,
        mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    ) -> UiDeclarationStructuralSemantics {
        let containment_intent = self
            .source_backed_mosaic_membership_name
            .clone()
            .map(|mosaic_name| UiDeclarationContainmentIntent::DeclaredMosaicMembership {
                mosaic_name,
            })
            .unwrap_or_else(|| admitted.containment_intent().clone());

        UiDeclarationStructuralSemantics::new(
            admitted.family_kind(),
            admitted.role(),
            admitted.operator_kind(),
            mosaic_sizing_contract_id.or_else(|| admitted.mosaic_sizing_contract_id().cloned()),
            containment_intent,
            admitted.slot_participation_intent().clone(),
            admitted.ordering_guarantee(),
            admitted.repetition_posture(),
        )
    }

    fn effective_declared_posture(
        &self,
        admitted: &UiDeclaredPostureContract,
    ) -> UiDeclaredPostureContract {
        let Some(modifier) = self.source_backed_measurement_constraint_modifier else {
            return admitted.clone();
        };
        let measurement_lane = admitted.measurement_policy();
        let measurement_policy = measurement_lane.admitted().cloned().or_else(|| {
            UiDeclaredMeasurementPolicyPosture::new(
                None,
                Some(modifier),
                None,
                None,
                vec![],
            )
        });
        let measurement_policy = measurement_policy.map(|policy| {
            if policy.constraint_modifier().is_some() {
                policy
            } else {
                UiDeclaredMeasurementPolicyPosture::new(
                    policy.mode(),
                    Some(modifier),
                    policy.basis_source(),
                    policy.ownership_posture(),
                    policy.evidence_requirements().to_vec(),
                )
                .expect("source-backed measurement modifier should preserve admitted policy shape")
            }
        });

        UiDeclaredPostureContract::new(
            admitted.query_binding().clone(),
            admitted.service_usage().clone(),
            admitted.touch_meaning().clone(),
            UiDeclaredPostureLane::new(
                measurement_lane.applicability(),
                measurement_policy,
            ),
            admitted.host_capability().clone(),
        )
    }
}
