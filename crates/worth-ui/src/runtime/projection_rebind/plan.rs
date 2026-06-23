use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiAdmittedRuntimeChangeEvidence,
    WorthUiComponentCompatibility, WorthUiProjectionPlanContract,
    WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeChangeFamilyStatus,
};

use super::{
    WorthUiProjectionRebindBatchReceipt, WorthUiProjectionRebindCounters,
    WorthUiProjectionRebindPlanDenial, WorthUiProjectionRebindRowReceipt,
    WorthUiProjectionRebindStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionRebindPlan<P: WorthUiProjectionPlanContract> {
    Preserve(WorthUiPreservedProjectionRebindPlan<P>),
    Rebuild(WorthUiActivatedProjectionRebindPlan<P>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPreservedProjectionRebindPlan<P: WorthUiProjectionPlanContract> {
    evidence: WorthUiAdmittedRuntimeChangeEvidence,
    admitted_projection: WorthUiAdmittedProjectionPlan<P>,
    status: WorthUiProjectionRebindStatus,
    component_compatibility: Option<WorthUiComponentCompatibility>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActivatedProjectionRebindPlan<P: WorthUiProjectionPlanContract> {
    evidence: WorthUiAdmittedRuntimeChangeEvidence,
    admitted_projection: WorthUiAdmittedProjectionPlan<P>,
    component_compatibility: Option<WorthUiComponentCompatibility>,
}

impl<P: WorthUiProjectionPlanContract> WorthUiProjectionRebindPlan<P> {
    pub(crate) fn prepare(
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        admitted_projection: WorthUiAdmittedProjectionPlan<P>,
    ) -> Result<Self, WorthUiProjectionRebindPlanDenial> {
        if admitted_projection.runtime_instance() != evidence.runtime_instance() {
            return Err(WorthUiProjectionRebindPlanDenial::RuntimeEvidenceMismatch);
        }
        let projection_rebind_classification =
            classify_projection_rebind(evidence, &admitted_projection)?;
        Ok(match projection_rebind_classification {
            WorthUiProjectionRebindClassification::Preserve {
                status,
                component_compatibility,
            } => Self::Preserve(WorthUiPreservedProjectionRebindPlan {
                evidence: evidence.clone(),
                admitted_projection,
                status,
                component_compatibility,
            }),
            WorthUiProjectionRebindClassification::Rebuild {
                component_compatibility,
            } => Self::Rebuild(WorthUiActivatedProjectionRebindPlan {
                evidence: evidence.clone(),
                admitted_projection,
                component_compatibility,
            }),
        })
    }
}

impl<P: WorthUiProjectionPlanContract> WorthUiPreservedProjectionRebindPlan<P> {
    pub(crate) fn evidence(&self) -> &WorthUiAdmittedRuntimeChangeEvidence {
        &self.evidence
    }

    pub(crate) fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }

    pub(crate) fn complete_preserved(
        self,
    ) -> (
        WorthUiAdmittedProjectionPlan<P>,
        WorthUiProjectionRebindBatchReceipt,
    ) {
        let previous_frame_digest = self
            .admitted_projection
            .plan()
            .projection_equivalence_digest();
        let row = projection_rebind_row_receipt(
            &self.admitted_projection,
            self.status,
            false,
            previous_frame_digest,
            previous_frame_digest,
            self.component_compatibility.clone(),
        );
        let receipt = WorthUiProjectionRebindBatchReceipt::single_row(
            self.evidence.runtime_instance(),
            self.evidence.digest(),
            WorthUiProjectionRebindCounters::inspected_without_intersection(self.status),
            row,
        );
        (self.admitted_projection, receipt)
    }
}

impl<P: WorthUiProjectionPlanContract> WorthUiActivatedProjectionRebindPlan<P> {
    pub(crate) fn evidence(&self) -> &WorthUiAdmittedRuntimeChangeEvidence {
        &self.evidence
    }

    pub(crate) fn complete_rebuild(
        self,
        rebound_projection: WorthUiAdmittedProjectionPlan<P>,
    ) -> (
        WorthUiAdmittedProjectionPlan<P>,
        WorthUiProjectionRebindBatchReceipt,
    ) {
        let previous_frame_digest = self.previous_frame_digest();
        let rebound_frame_digest = rebound_projection.plan().projection_equivalence_digest();
        let status = if previous_frame_digest == rebound_frame_digest {
            WorthUiProjectionRebindStatus::EquivalentAfterActivation
        } else {
            WorthUiProjectionRebindStatus::ReboundAfterActivation
        };
        let row = projection_rebind_row_receipt(
            &self.admitted_projection,
            status,
            true,
            previous_frame_digest,
            rebound_frame_digest,
            self.component_compatibility.clone(),
        );
        let receipt = WorthUiProjectionRebindBatchReceipt::single_row(
            self.evidence.runtime_instance(),
            self.evidence.digest(),
            WorthUiProjectionRebindCounters::after_rebuild(status),
            row,
        );
        (rebound_projection, receipt)
    }

    fn previous_frame_digest(&self) -> u64 {
        self.admitted_projection
            .plan()
            .projection_equivalence_digest()
    }
}

fn projection_rebind_row_receipt<P: WorthUiProjectionPlanContract>(
    admitted_projection: &WorthUiAdmittedProjectionPlan<P>,
    status: WorthUiProjectionRebindStatus,
    rebuild_attempted: bool,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
    component_compatibility: Option<WorthUiComponentCompatibility>,
) -> WorthUiProjectionRebindRowReceipt {
    WorthUiProjectionRebindRowReceipt::new_with_component_compatibility(
        admitted_projection.dependencies().identity().clone(),
        admitted_projection.dependencies().family(),
        status,
        rebuild_attempted,
        previous_frame_digest,
        rebound_frame_digest,
        component_compatibility,
    )
}

enum WorthUiProjectionRebindClassification {
    Preserve {
        status: WorthUiProjectionRebindStatus,
        component_compatibility: Option<WorthUiComponentCompatibility>,
    },
    Rebuild {
        component_compatibility: Option<WorthUiComponentCompatibility>,
    },
}

fn classify_projection_rebind<P: WorthUiProjectionPlanContract>(
    evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    admitted_projection: &WorthUiAdmittedProjectionPlan<P>,
) -> Result<WorthUiProjectionRebindClassification, WorthUiProjectionRebindPlanDenial> {
    if evidence
        .family_rows()
        .iter()
        .any(|row| row.status() == WorthUiRuntimeChangeFamilyStatus::ReadyForFrameBoundary)
    {
        return Err(WorthUiProjectionRebindPlanDenial::ReloadNotActivated);
    }
    match evidence.posture() {
        WorthUiRuntimeChangeActivationPosture::EquivalentNoOp => {
            Ok(WorthUiProjectionRebindClassification::Preserve {
                status: WorthUiProjectionRebindStatus::PreservedEquivalentReload,
                component_compatibility: None,
            })
        }
        WorthUiRuntimeChangeActivationPosture::Denied => {
            Ok(WorthUiProjectionRebindClassification::Preserve {
                status: WorthUiProjectionRebindStatus::PreservedDeniedReload,
                component_compatibility: None,
            })
        }
        WorthUiRuntimeChangeActivationPosture::ReadyForFrameBoundary => {
            Err(WorthUiProjectionRebindPlanDenial::ReloadNotActivated)
        }
        WorthUiRuntimeChangeActivationPosture::Activated
        | WorthUiRuntimeChangeActivationPosture::Mixed(_) => {
            let intersecting_row = evidence.family_rows().iter().find(|row| {
                row.status() == WorthUiRuntimeChangeFamilyStatus::Activated
                    && admitted_projection
                        .dependencies()
                        .intersects_changed_facts(row.changed_facts())
            });
            if let Some(row) = intersecting_row {
                Ok(WorthUiProjectionRebindClassification::Rebuild {
                    component_compatibility: row.component_compatibility().cloned(),
                })
            } else {
                Ok(WorthUiProjectionRebindClassification::Preserve {
                    status: WorthUiProjectionRebindStatus::EquivalentAfterActivation,
                    component_compatibility: None,
                })
            }
        }
    }
}
