use crate::execution::ExecutionCounters;
use crate::preview::binding::PreviewBindingCounters;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreviewExecutionCounters {
    pub(in crate::preview) binding_counters: PreviewBindingCounters,
    pub(in crate::preview) execution_counters: ExecutionCounters,
    pub(in crate::preview) preview_execution_envelope_count: usize,
    pub(in crate::preview) preview_execution_count: usize,
    pub(in crate::preview) preview_promotable_execution_count: usize,
    pub(in crate::preview) preview_read_only_execution_count: usize,
    pub(in crate::preview) preview_comparison_eligibility_proof_count: usize,
    pub(in crate::preview) preview_comparison_shape_check_width: usize,
    pub(in crate::preview) preview_workflow_foundation_admission_count: usize,
    pub(in crate::preview) preview_workflow_foundation_denial_count: usize,
    pub(in crate::preview) preview_workflow_foundation_artifact_lookup_count: usize,
    pub(in crate::preview) preview_work_avoided_by_explicit_basis_count: usize,
}

impl PreviewExecutionCounters {
    pub fn binding_counters(&self) -> &PreviewBindingCounters {
        &self.binding_counters
    }

    pub fn execution_counters(&self) -> &ExecutionCounters {
        &self.execution_counters
    }

    pub fn preview_execution_envelope_count(&self) -> usize {
        self.preview_execution_envelope_count
    }

    pub fn preview_execution_count(&self) -> usize {
        self.preview_execution_count
    }

    pub fn preview_promotable_execution_count(&self) -> usize {
        self.preview_promotable_execution_count
    }

    pub fn preview_read_only_execution_count(&self) -> usize {
        self.preview_read_only_execution_count
    }

    pub fn preview_comparison_eligibility_proof_count(&self) -> usize {
        self.preview_comparison_eligibility_proof_count
    }

    pub fn preview_comparison_shape_check_width(&self) -> usize {
        self.preview_comparison_shape_check_width
    }

    pub fn preview_workflow_foundation_admission_count(&self) -> usize {
        self.preview_workflow_foundation_admission_count
    }

    pub fn preview_workflow_foundation_denial_count(&self) -> usize {
        self.preview_workflow_foundation_denial_count
    }

    pub fn preview_workflow_foundation_artifact_lookup_count(&self) -> usize {
        self.preview_workflow_foundation_artifact_lookup_count
    }

    pub fn preview_work_avoided_by_explicit_basis_count(&self) -> usize {
        self.preview_work_avoided_by_explicit_basis_count
    }

    #[cfg(test)]
    pub(crate) fn absorb(&mut self, other: &Self) {
        self.binding_counters.absorb(other.binding_counters());
        self.execution_counters.absorb(other.execution_counters());
        self.preview_execution_envelope_count += other.preview_execution_envelope_count;
        self.preview_execution_count += other.preview_execution_count;
        self.preview_promotable_execution_count += other.preview_promotable_execution_count;
        self.preview_read_only_execution_count += other.preview_read_only_execution_count;
        self.preview_comparison_eligibility_proof_count +=
            other.preview_comparison_eligibility_proof_count;
        self.preview_comparison_shape_check_width += other.preview_comparison_shape_check_width;
        self.preview_workflow_foundation_admission_count +=
            other.preview_workflow_foundation_admission_count;
        self.preview_workflow_foundation_denial_count +=
            other.preview_workflow_foundation_denial_count;
        self.preview_workflow_foundation_artifact_lookup_count +=
            other.preview_workflow_foundation_artifact_lookup_count;
        self.preview_work_avoided_by_explicit_basis_count +=
            other.preview_work_avoided_by_explicit_basis_count;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreviewComparisonCounters {
    pub(in crate::preview) preview_promotion_comparison_count: usize,
    pub(in crate::preview) preview_promotion_comparison_denial_count: usize,
    pub(in crate::preview) preview_comparison_eligibility_proof_count: usize,
    pub(in crate::preview) preview_comparison_shape_check_width: usize,
    pub(in crate::preview) preview_basis_pair_width: usize,
}

impl PreviewComparisonCounters {
    pub fn preview_promotion_comparison_count(&self) -> usize {
        self.preview_promotion_comparison_count
    }

    pub fn preview_promotion_comparison_denial_count(&self) -> usize {
        self.preview_promotion_comparison_denial_count
    }

    pub fn preview_comparison_eligibility_proof_count(&self) -> usize {
        self.preview_comparison_eligibility_proof_count
    }

    pub fn preview_comparison_shape_check_width(&self) -> usize {
        self.preview_comparison_shape_check_width
    }

    pub fn preview_basis_pair_width(&self) -> usize {
        self.preview_basis_pair_width
    }

    #[cfg(test)]
    pub(crate) fn absorb(&mut self, other: &Self) {
        self.preview_promotion_comparison_count += other.preview_promotion_comparison_count;
        self.preview_promotion_comparison_denial_count +=
            other.preview_promotion_comparison_denial_count;
        self.preview_comparison_eligibility_proof_count +=
            other.preview_comparison_eligibility_proof_count;
        self.preview_comparison_shape_check_width += other.preview_comparison_shape_check_width;
        self.preview_basis_pair_width += other.preview_basis_pair_width;
    }
}
