use crate::runtime::CausalInspectionPlan;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryInspectionCost {
    anchor_derivation_count: usize,
    evidence_reference_resolution_count: usize,
    admission_count: usize,
    bridge_envelope_assembly_count: usize,
    evidence_reference_count: usize,
}

impl WorthQueryInspectionCost {
    pub fn anchor_derivation_count(&self) -> usize {
        self.anchor_derivation_count
    }

    pub fn evidence_reference_resolution_count(&self) -> usize {
        self.evidence_reference_resolution_count
    }

    pub fn admission_count(&self) -> usize {
        self.admission_count
    }

    pub fn bridge_envelope_assembly_count(&self) -> usize {
        self.bridge_envelope_assembly_count
    }

    pub fn evidence_reference_count(&self) -> usize {
        self.evidence_reference_count
    }

    pub(crate) fn from_plan(plan: &CausalInspectionPlan, materialize: bool) -> Self {
        let estimated = plan.estimated_cost();
        Self {
            anchor_derivation_count: estimated.anchor_derivation_count(),
            evidence_reference_resolution_count: estimated.evidence_reference_resolution_count(),
            admission_count: estimated.admission_count(),
            bridge_envelope_assembly_count: usize::from(materialize),
            evidence_reference_count: estimated.evidence_reference_count(),
        }
    }
}
