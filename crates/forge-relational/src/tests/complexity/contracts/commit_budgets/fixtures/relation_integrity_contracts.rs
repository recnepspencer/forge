use super::super::*;

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_cardinality_runtime(
) -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_one".into(),
                source_max: Some(1),
                source_min: None,
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: None,
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_uniqueness_runtime(
) -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            vec![crate::schema::data::UniquenessContractDeclaration {
                contract_id: "uniq".into(),
                scope: crate::schema::data::UniquenessScope::NormalizedSymmetricEdge,
            }],
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_symmetry_runtime(
) -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![crate::schema::data::SymmetryContractDeclaration {
                contract_id: "paired_twin".into(),
                mode: crate::schema::data::SymmetryMode::PairedTwinRequired,
            }],
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_multi_contract_runtime(
) -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "kind".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: false,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_max_two".into(),
                source_max: Some(2),
                source_min: None,
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: None,
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            vec![crate::schema::data::UniquenessContractDeclaration {
                contract_id: "uniq".into(),
                scope: crate::schema::data::UniquenessScope::NormalizedSymmetricEdge,
            }],
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_endpoint_deletion_runtime(
) -> RelationalRuntime {
    endpoint_deletion_runtime(
        crate::schema::data::EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
        CascadeDeletePolicy::RetainDanglingForAudit,
    )
}

pub(in crate::tests::complexity::contracts::commit_budgets) fn relation_integrity_minimum_certification_runtime(
) -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "minimum".into(),
                source_max: None,
                source_min: Some(1),
                target_max: None,
                target_min: None,
                pair_max: None,
                pair_min: Some(2),
                pair_min_semantics:
                    crate::schema::data::PairMinimumSemantics::ObservedDirectedPairs,
                minimum_enforcement:
                    crate::schema::data::MinimumCardinalityEnforcement::CertificationBoundary,
            }],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ..RelationIntegritySchemaFixture::default()
    }
    .build_runtime()
}
