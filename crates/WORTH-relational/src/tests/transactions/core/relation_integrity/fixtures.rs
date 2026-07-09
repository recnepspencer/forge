use crate::tests::support::*;

pub(super) fn source_max_one_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
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

pub(super) fn publication_source_min_one_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint_domain".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "source_min_one".into(),
                source_max: None,
                source_min: Some(1),
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

pub(super) fn publication_pair_min_two_runtime() -> RelationalRuntime {
    RelationIntegritySchemaFixture {
        relation_integrity: crate::schema::data::RelationIntegrityDeclarations::new(
            vec![crate::schema::data::EndpointKindContractDeclaration {
                contract_id: "endpoint_domain".into(),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(1)],
                self_edges_allowed: true,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
            }],
            vec![crate::schema::data::CardinalityContractDeclaration {
                contract_id: "pair_min_two".into(),
                source_max: None,
                source_min: None,
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

pub(super) fn certification_authority_source_min_one_runtime() -> RelationalRuntime {
    publication_source_min_one_runtime()
}
