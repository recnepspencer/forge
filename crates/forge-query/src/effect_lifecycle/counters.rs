use crate::identity::hash_parts;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectLifecycleCounters {
    raw_intent_width: usize,
    normalized_effect_family_count: usize,
    workflow_declaration_family_check_count: usize,
    workflow_authority_target_check_count: usize,
    source_path_count: usize,
    support_lookup_count: usize,
    support_lookup_width: usize,
    basis_scope_check_count: usize,
    admitted_effect_count: usize,
    authority_scoped_plan_count: usize,
    lowered_effect_count: usize,
    batch_lowering_count: usize,
    lowering_denied_count: usize,
    effect_lowering_width: usize,
    effect_executor_rediscovery_count: usize,
    executed_effect_count: usize,
    execution_denied_count: usize,
    effect_execution_width: usize,
    advisory_count: usize,
    rebind_required_count: usize,
    deferred_count: usize,
    denied_count: usize,
    effect_support_row_count: usize,
}

impl EffectLifecycleCounters {
    pub(crate) fn normalized(
        source_path_count: usize,
        workflow_authority_target_check_count: usize,
        basis_scope_check_count: usize,
    ) -> Self {
        Self {
            raw_intent_width: 1,
            normalized_effect_family_count: 1,
            workflow_declaration_family_check_count: 1,
            workflow_authority_target_check_count,
            source_path_count,
            basis_scope_check_count,
            ..Self::default()
        }
    }

    pub(crate) fn support_lookup(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn admitted(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn authority_scoped_plan(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            authority_scoped_plan_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn rebind_required(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            rebind_required_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn lowered(
        effect_support_row_count: usize,
        effect_lowering_width: usize,
        effect_executor_rediscovery_count: usize,
    ) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            authority_scoped_plan_count: 1,
            lowered_effect_count: 1,
            batch_lowering_count: 0,
            effect_lowering_width,
            effect_executor_rediscovery_count,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn lowering_denied(
        effect_support_row_count: usize,
        effect_lowering_width: usize,
    ) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            authority_scoped_plan_count: 1,
            lowering_denied_count: 1,
            batch_lowering_count: 0,
            effect_lowering_width,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn deferred(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            deferred_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn executed(
        effect_support_row_count: usize,
        effect_lowering_width: usize,
        effect_executor_rediscovery_count: usize,
        effect_execution_width: usize,
    ) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            authority_scoped_plan_count: 1,
            lowered_effect_count: 1,
            batch_lowering_count: 0,
            effect_lowering_width,
            effect_executor_rediscovery_count,
            executed_effect_count: 1,
            effect_execution_width,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn execution_denied(
        effect_support_row_count: usize,
        effect_lowering_width: usize,
        effect_executor_rediscovery_count: usize,
    ) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            admitted_effect_count: 1,
            authority_scoped_plan_count: 1,
            lowered_effect_count: 1,
            batch_lowering_count: 0,
            effect_lowering_width,
            effect_executor_rediscovery_count,
            execution_denied_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn denied(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            denied_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn advisory(effect_support_row_count: usize) -> Self {
        Self {
            support_lookup_count: 1,
            support_lookup_width: effect_support_row_count,
            basis_scope_check_count: 1,
            advisory_count: 1,
            effect_support_row_count,
            ..Self::default()
        }
    }

    pub(crate) fn admitted_batch(component_count: usize) -> Self {
        Self {
            admitted_effect_count: component_count,
            effect_support_row_count: component_count,
            ..Self::default()
        }
    }

    pub(crate) fn batch_admission_denied(component_count: usize) -> Self {
        Self {
            denied_count: 1,
            raw_intent_width: component_count,
            effect_support_row_count: component_count,
            ..Self::default()
        }
    }

    pub(crate) fn lowered_batch(
        component_count: usize,
        effect_lowering_width: usize,
        effect_executor_rediscovery_count: usize,
    ) -> Self {
        Self {
            admitted_effect_count: component_count,
            lowered_effect_count: component_count,
            batch_lowering_count: 1,
            effect_lowering_width,
            effect_executor_rediscovery_count,
            effect_support_row_count: component_count,
            ..Self::default()
        }
    }

    pub(crate) fn executed_batch(
        component_count: usize,
        effect_lowering_width: usize,
        effect_executor_rediscovery_count: usize,
        effect_execution_width: usize,
    ) -> Self {
        Self {
            admitted_effect_count: component_count,
            lowered_effect_count: component_count,
            batch_lowering_count: 1,
            executed_effect_count: component_count,
            effect_lowering_width,
            effect_executor_rediscovery_count,
            effect_execution_width,
            effect_support_row_count: component_count,
            ..Self::default()
        }
    }

    pub(crate) fn intent_denial(
        source_path_count: usize,
        workflow_authority_target_check_count: usize,
        basis_scope_check_count: usize,
    ) -> Self {
        Self {
            raw_intent_width: 1,
            workflow_declaration_family_check_count: 1,
            workflow_authority_target_check_count,
            source_path_count,
            basis_scope_check_count,
            denied_count: 1,
            ..Self::default()
        }
    }

    pub fn raw_intent_width(&self) -> usize {
        self.raw_intent_width
    }

    pub fn normalized_effect_family_count(&self) -> usize {
        self.normalized_effect_family_count
    }

    pub fn workflow_declaration_family_check_count(&self) -> usize {
        self.workflow_declaration_family_check_count
    }

    pub fn workflow_authority_target_check_count(&self) -> usize {
        self.workflow_authority_target_check_count
    }

    pub fn source_path_count(&self) -> usize {
        self.source_path_count
    }

    pub fn support_lookup_count(&self) -> usize {
        self.support_lookup_count
    }

    pub fn support_lookup_width(&self) -> usize {
        self.support_lookup_width
    }

    pub fn basis_scope_check_count(&self) -> usize {
        self.basis_scope_check_count
    }

    pub fn admitted_effect_count(&self) -> usize {
        self.admitted_effect_count
    }

    pub fn authority_scoped_plan_count(&self) -> usize {
        self.authority_scoped_plan_count
    }

    pub fn lowered_effect_count(&self) -> usize {
        self.lowered_effect_count
    }

    pub fn lowering_denied_count(&self) -> usize {
        self.lowering_denied_count
    }

    pub fn batch_lowering_count(&self) -> usize {
        self.batch_lowering_count
    }

    pub fn effect_lowering_width(&self) -> usize {
        self.effect_lowering_width
    }

    pub fn effect_executor_rediscovery_count(&self) -> usize {
        self.effect_executor_rediscovery_count
    }

    pub fn executed_effect_count(&self) -> usize {
        self.executed_effect_count
    }

    pub fn execution_denied_count(&self) -> usize {
        self.execution_denied_count
    }

    pub fn effect_execution_width(&self) -> usize {
        self.effect_execution_width
    }

    pub fn advisory_count(&self) -> usize {
        self.advisory_count
    }

    pub fn rebind_required_count(&self) -> usize {
        self.rebind_required_count
    }

    pub fn deferred_count(&self) -> usize {
        self.deferred_count
    }

    pub fn denied_count(&self) -> usize {
        self.denied_count
    }

    pub fn effect_support_row_count(&self) -> usize {
        self.effect_support_row_count
    }

    pub fn digest(&self) -> String {
        hash_parts(&[
            format!("raw:{}", self.raw_intent_width),
            format!("normalized:{}", self.normalized_effect_family_count),
            format!(
                "workflow_family_checks:{}",
                self.workflow_declaration_family_check_count
            ),
            format!(
                "workflow_target_checks:{}",
                self.workflow_authority_target_check_count
            ),
            format!("source_paths:{}", self.source_path_count),
            format!("support_lookups:{}", self.support_lookup_count),
            format!("support_width:{}", self.support_lookup_width),
            format!("basis_scope_checks:{}", self.basis_scope_check_count),
            format!("admitted:{}", self.admitted_effect_count),
            format!(
                "authority_scoped_plans:{}",
                self.authority_scoped_plan_count
            ),
            format!("lowered_effects:{}", self.lowered_effect_count),
            format!("batch_lowering:{}", self.batch_lowering_count),
            format!("lowering_denied:{}", self.lowering_denied_count),
            format!("effect_lowering_width:{}", self.effect_lowering_width),
            format!(
                "effect_executor_rediscovery:{}",
                self.effect_executor_rediscovery_count
            ),
            format!("executed_effects:{}", self.executed_effect_count),
            format!("execution_denied:{}", self.execution_denied_count),
            format!("effect_execution_width:{}", self.effect_execution_width),
            format!("advisory:{}", self.advisory_count),
            format!("rebind_required:{}", self.rebind_required_count),
            format!("deferred:{}", self.deferred_count),
            format!("denied:{}", self.denied_count),
            format!("effect_support_rows:{}", self.effect_support_row_count),
        ])
    }
}
