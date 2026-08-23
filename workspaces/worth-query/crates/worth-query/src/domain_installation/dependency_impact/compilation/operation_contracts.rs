use super::super::compiled::{
    WorthQueryCompiledSemanticAspectDependency, WorthQuerySemanticAspectDependencyLocus,
    WorthQuerySemanticAspectDependencySource, WorthQuerySemanticDependencyRole,
};
use super::operation_definition::SemanticAspectDependencyCompilation;

impl SemanticAspectDependencyCompilation {
    pub(super) fn push_collection_contract(
        &mut self,
        collection: &worth_query_installation::facade::WorthQueryOperationCollectionContract,
    ) {
        use worth_query_installation::facade::{
            WorthQueryOperationCollectionContract, WorthQueryOperationGroupingContract,
        };
        let WorthQueryOperationCollectionContract::Collection {
            row_identity_field,
            ordering_fields,
            grouping,
            window,
            ..
        } = collection
        else {
            return;
        };
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::CollectionRowIdentity,
                WorthQuerySemanticDependencyRole::SelectionOrMembership,
                WorthQuerySemanticAspectDependencySource::CollectionField(
                    row_identity_field.clone(),
                ),
            ));
        self.counters.collection_membership_edges += 1;
        for (field_ordinal, field) in ordering_fields.iter().enumerate() {
            self.dependencies
                .push(WorthQueryCompiledSemanticAspectDependency::new(
                    WorthQuerySemanticAspectDependencyLocus::CollectionOrdering { field_ordinal },
                    WorthQuerySemanticDependencyRole::Ordering,
                    WorthQuerySemanticAspectDependencySource::CollectionField(field.clone()),
                ));
            self.counters.collection_ordering_edges += 1;
        }
        if let WorthQueryOperationGroupingContract::Grouped { grouping_fields } = grouping {
            for (field_ordinal, field) in grouping_fields.iter().enumerate() {
                self.dependencies
                    .push(WorthQueryCompiledSemanticAspectDependency::new(
                        WorthQuerySemanticAspectDependencyLocus::CollectionGrouping {
                            field_ordinal,
                        },
                        WorthQuerySemanticDependencyRole::Grouping,
                        WorthQuerySemanticAspectDependencySource::CollectionField(field.clone()),
                    ));
                self.counters.collection_grouping_edges += 1;
            }
        }
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::CollectionWindow,
                WorthQuerySemanticDependencyRole::WindowBoundary,
                WorthQuerySemanticAspectDependencySource::CollectionWindowPolicy(*window),
            ));
        self.counters.collection_window_edges += 1;
    }

    pub(super) fn push_installed_semantic_contracts(
        &mut self,
        semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
    ) {
        self.push_result_shape_contract(semantics);
        self.push_touch_contract(&semantics.touches);
        self.push_effect_contract(&semantics.effects);
        self.push_invariant_contract(&semantics.invariants);
        self.push_execution_posture_contracts(semantics);
    }

    fn push_result_shape_contract(
        &mut self,
        semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
    ) {
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::ResultShape,
                WorthQuerySemanticDependencyRole::ProjectedValue,
                WorthQuerySemanticAspectDependencySource::ResultShape(
                    semantics.canonical_query.clone(),
                ),
            ));
        self.counters.result_shape_edges += 1;
    }

    fn push_touch_contract(
        &mut self,
        touches: &worth_query_installation::facade::WorthQueryOperationTouchContract,
    ) {
        use worth_query_installation::facade::WorthQueryOperationTouchContract;
        let WorthQueryOperationTouchContract::Declared {
            graph_roles,
            scopes,
        } = touches
        else {
            return;
        };
        for (role_ordinal, role) in graph_roles.iter().enumerate() {
            self.dependencies
                .push(WorthQueryCompiledSemanticAspectDependency::new(
                    WorthQuerySemanticAspectDependencyLocus::TouchGraphRole { role_ordinal },
                    WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                    WorthQuerySemanticAspectDependencySource::TouchGraphRole(role.clone()),
                ));
            self.counters.touch_edges += 1;
        }
        for (scope_ordinal, scope) in scopes.iter().enumerate() {
            let worth_query_installation::facade::WorthQueryOperationTouchScope::DeclaredDomain(
                identity,
            ) = scope
            else {
                continue;
            };
            self.dependencies
                .push(WorthQueryCompiledSemanticAspectDependency::new(
                    WorthQuerySemanticAspectDependencyLocus::TouchScope { scope_ordinal },
                    WorthQuerySemanticDependencyRole::SelectionOrMembership,
                    WorthQuerySemanticAspectDependencySource::TouchScope(
                        identity.as_str().to_owned(),
                    ),
                ));
            self.counters.touch_edges += 1;
        }
    }

    fn push_effect_contract(
        &mut self,
        effects: &worth_query_installation::facade::WorthQueryOperationEffectContract,
    ) {
        use worth_query_installation::facade::WorthQueryOperationEffectContract;
        let WorthQueryOperationEffectContract::Declared { effect_families } = effects else {
            return;
        };
        for (effect_ordinal, family) in effect_families.iter().enumerate() {
            self.dependencies
                .push(WorthQueryCompiledSemanticAspectDependency::new(
                    WorthQuerySemanticAspectDependencyLocus::EffectFamily { effect_ordinal },
                    WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                    WorthQuerySemanticAspectDependencySource::EffectFamily(*family),
                ));
            self.counters.effect_contract_edges += 1;
        }
    }

    fn push_invariant_contract(
        &mut self,
        invariants: &worth_query_installation::facade::WorthQueryOperationInvariantContract,
    ) {
        use worth_query_installation::facade::WorthQueryOperationInvariantContract;
        let WorthQueryOperationInvariantContract::Declared { invariant_slots } = invariants else {
            return;
        };
        for (invariant_ordinal, invariant) in invariant_slots.iter().enumerate() {
            self.dependencies
                .push(WorthQueryCompiledSemanticAspectDependency::new(
                    WorthQuerySemanticAspectDependencyLocus::InstalledInvariant {
                        invariant_ordinal,
                    },
                    WorthQuerySemanticDependencyRole::InstalledDomainInvariant,
                    WorthQuerySemanticAspectDependencySource::InstalledInvariant(invariant.clone()),
                ));
            self.counters.invariant_contract_edges += 1;
        }
    }

    fn push_execution_posture_contracts(
        &mut self,
        semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
    ) {
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::ReplayContract,
                WorthQuerySemanticDependencyRole::AdvisoryOnlyContext,
                WorthQuerySemanticAspectDependencySource::ReplayContract(semantics.replay),
            ));
        self.counters.replay_contract_edges += 1;
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::LineageContract,
                WorthQuerySemanticDependencyRole::OperationalIdentity,
                WorthQuerySemanticAspectDependencySource::LineageContract(semantics.lineage),
            ));
        self.counters.lineage_contract_edges += 1;
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::SupportContract,
                WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                WorthQuerySemanticAspectDependencySource::SupportContract(semantics.support),
            ));
        self.counters.support_contract_edges += 1;
    }
}
