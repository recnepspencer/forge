use forge_relational::facade::diagnostics::DiagnosticCode;
use forge_relational::facade::errors::ErrorContext;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{RecordRef, TransactionCommitError};
use schema::facade::topology_authoring::{
    build_milestone_one_primitive_intent, verify_topology_intent,
    MilestoneOnePrimitiveAuthoringError, MilestoneOnePrimitiveCase,
};
use schema::facade::{
    CreateKey, EntityKind, EntityReference, MutationOrigin, RawTopologyIntent, RelationKind,
    TopologyAuthorityError, TopologyMutation, TopologyRelationKind,
};

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::support::reporting::{
    IllegalTopologyRejectionCaseReport, IllegalTopologyRejectionReport, PrimitiveRejectionReport,
};

pub(crate) fn summarize_primitive_rejection(
    error: &MilestoneOnePrimitiveAuthoringError,
) -> PrimitiveRejectionReport {
    match error {
        MilestoneOnePrimitiveAuthoringError::InvalidParameter {
            family,
            parameter,
            requirement,
        } => PrimitiveRejectionReport {
            rejection_class: "OutOfClass".to_string(),
            validator_family: None,
            diagnostic_code: None,
            detail: format!("invalid `{family}` parameter `{parameter}`; expected {requirement}"),
            fields_json: Some(format!(
                "{{\"family\":\"{family}\",\"parameter\":{parameter},\"requirement\":\"{requirement}\"}}"
            )),
            context: None,
            localized_entity_count: 0,
            localized_relation_count: 0,
        },
        MilestoneOnePrimitiveAuthoringError::Authority(authority) => {
            summarize_authority_rejection(authority, None, None)
        }
    }
}

pub(crate) fn summarize_authority_rejection(
    error: &TopologyAuthorityError,
    rejection_class_override: Option<&str>,
    validator_family_override: Option<&str>,
) -> PrimitiveRejectionReport {
    match error {
        TopologyAuthorityError::Commit(TransactionCommitError::Conflict { error, .. }) => {
            let (localized_entity_count, localized_relation_count) =
                summarize_localized_record_counts(&error.context);
            PrimitiveRejectionReport {
                rejection_class: rejection_class_override
                    .unwrap_or("InvariantFailure")
                    .to_string(),
                validator_family: validator_family_override
                    .map(ToString::to_string)
                    .or_else(|| infer_validator_family(Some(error.code()), Some(&error.context))),
                diagnostic_code: Some(error.code()),
                detail: error.detail(),
                fields_json: error.fields().map(ToString::to_string),
                context: Some(error.context.clone()),
                localized_entity_count,
                localized_relation_count,
            }
        }
        other => PrimitiveRejectionReport {
            rejection_class: rejection_class_override
                .unwrap_or("AuthorityBlocked")
                .to_string(),
            validator_family: validator_family_override.map(ToString::to_string),
            diagnostic_code: None,
            detail: format!("{other:?}"),
            fields_json: None,
            context: None,
            localized_entity_count: 0,
            localized_relation_count: 0,
        },
    }
}

fn summarize_localized_record_counts(context: &ErrorContext) -> (usize, usize) {
    let mut entity_count = 0usize;
    let mut relation_count = 0usize;
    for record in &context.affected_records {
        match record {
            RecordRef::Entity(_) => entity_count += 1,
            RecordRef::Relation(_) => relation_count += 1,
        }
    }
    (entity_count, relation_count)
}

fn infer_validator_family(
    diagnostic_code: Option<DiagnosticCode>,
    _context: Option<&ErrorContext>,
) -> Option<String> {
    match diagnostic_code {
        Some(DiagnosticCode::RelationCardinalityViolation)
        | Some(DiagnosticCode::RelationEndpointDeletionIntegrityViolation)
        | Some(DiagnosticCode::RelationUniquenessViolation)
        | Some(DiagnosticCode::RelationSymmetryViolation) => Some("shell_closure".to_string()),
        Some(DiagnosticCode::InvalidRelationEndpoint)
        | Some(DiagnosticCode::RelationEndpointKindViolation) => Some("ownership".to_string()),
        Some(DiagnosticCode::InvariantViolation) => Some("authority_boundary".to_string()),
        _ => None,
    }
}

pub(crate) fn certify_milestone_one_illegal_topology_rejections<F>(
    runtime_factory: &mut F,
    stem: &str,
) -> Result<IllegalTopologyRejectionReport, MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::new();

    run_illegal_case(
        runtime_factory,
        "missing_persistent_names",
        "SheetDisk(n)",
        "InvariantFailure",
        Some("naming"),
        missing_persistent_names_intent(&format!("{stem}.missing_names")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "disconnected_wire",
        "WireOpen(n)",
        "IllegalAdmittedTopology",
        Some("vertex_disks"),
        disconnected_wire_intent(&format!("{stem}.disconnected_wire")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "illegal_wire_branch",
        "WireBranch(k)",
        "IllegalAdmittedTopology",
        Some("vertex_disks"),
        illegal_wire_branch_intent(&format!("{stem}.illegal_wire_branch")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "non_manifold_closed_shell",
        "SolidShell(f)",
        "IllegalAdmittedTopology",
        Some("shell_closure"),
        non_manifold_closed_shell_intent(&format!("{stem}.non_manifold_closed_shell")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "broken_loop_wiring",
        "WireClosed(n)",
        "IllegalAdmittedTopology",
        Some("loop_wiring"),
        broken_loop_wiring_intent(&format!("{stem}.broken_loop_wiring")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "broken_radial_ring",
        "NmtEdgeFan(k)",
        "IllegalAdmittedTopology",
        Some("radial_rings"),
        broken_radial_ring_intent(&format!("{stem}.broken_radial_ring")),
        &mut cases,
    )?;
    run_illegal_case(
        runtime_factory,
        "open_boundary_solid_shell",
        "SolidShell(f)",
        "IllegalAdmittedTopology",
        Some("shell_closure"),
        open_boundary_solid_shell_intent(&format!("{stem}.open_boundary_solid_shell")),
        &mut cases,
    )?;

    let rejection_digest = digest_rows(cases.iter().map(|case| {
        format!(
            "rejection:{}:{}:{}:{}",
            case.name,
            case.rejection.rejection_class,
            case.rejection
                .diagnostic_code
                .map(|code| format!("{code:?}"))
                .unwrap_or_else(|| "-".to_string()),
            case.rejection.detail
        )
    }));

    Ok(IllegalTopologyRejectionReport {
        case_count: cases.len(),
        cases,
        rejection_digest,
    })
}

fn run_illegal_case<F>(
    runtime_factory: &mut F,
    name: &str,
    family: &str,
    role: &str,
    validator_family: Option<&str>,
    intent: RawTopologyIntent,
    cases: &mut Vec<IllegalTopologyRejectionCaseReport>,
) -> Result<(), MilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut runtime = runtime_factory();
    let rejection = match verify_topology_intent(&mut runtime, intent) {
        Ok(_) => {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "illegal topology case `{name}` unexpectedly admitted"
            )))
        }
        Err(error) => {
            summarize_authority_rejection(&error.into_error(), Some(role), validator_family)
        }
    };
    cases.push(IllegalTopologyRejectionCaseReport {
        name: name.to_string(),
        family: family.to_string(),
        role: role.to_string(),
        rejection,
    });
    Ok(())
}

fn missing_persistent_names_intent(stem: &str) -> RawTopologyIntent {
    RawTopologyIntent::new(
        vec![
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new(format!("{stem}.model")),
                kind: EntityKind::Topology(schema::facade::TopologyEntityKind::Model),
            },
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new(format!("{stem}.body")),
                kind: EntityKind::Topology(schema::facade::TopologyEntityKind::Body),
            },
            TopologyMutation::CreateRelation {
                create_key: CreateKey::new(format!("{stem}.owns_body")),
                kind: RelationKind::Topology(TopologyRelationKind::ModelOwnsBody),
                source: EntityReference::Created(CreateKey::new(format!("{stem}.model"))),
                target: EntityReference::Created(CreateKey::new(format!("{stem}.body"))),
            },
        ],
        MutationOrigin::LocalEdit,
    )
}

fn disconnected_wire_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 2 },
    )
    .expect("build disconnected wire intent");
    intent.mutations.retain(|mutation| {
        !matches!(
            mutation,
            TopologyMutation::CreateRelation { create_key, .. }
                if create_key.as_str().ends_with("wire_open.half_edge.1.start")
        )
    });
    intent
}

fn illegal_wire_branch_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::WireBranch { branch_count: 3 },
    )
    .expect("build illegal wire branch intent");
    for suffix in [
        "wire_branch.branch_half_edge.1.edge",
        "wire_branch.branch_half_edge.1.start",
        "wire_branch.branch_half_edge.1.end",
    ] {
        intent.mutations.retain(|mutation| {
            !matches!(
                mutation,
                TopologyMutation::CreateRelation { create_key, .. }
                    if create_key.as_str().ends_with(suffix)
            )
        });
    }
    intent.mutations.push(topology_relation(
        stem,
        "wire_branch.branch_half_edge.1.edge.illegal_reuse",
        TopologyRelationKind::HalfEdgeUsesEdge,
        "wire_branch.branch_half_edge.1",
        "wire_branch.branch_edge.0",
    ));
    intent.mutations.push(topology_relation(
        stem,
        "wire_branch.branch_half_edge.1.start.illegal_reuse",
        TopologyRelationKind::HalfEdgeStartsAtVertex,
        "wire_branch.branch_half_edge.1",
        "wire_branch.center_vertex",
    ));
    intent.mutations.push(topology_relation(
        stem,
        "wire_branch.branch_half_edge.1.end.illegal_reuse",
        TopologyRelationKind::HalfEdgeEndsAtVertex,
        "wire_branch.branch_half_edge.1",
        "wire_branch.branch_vertex.0",
    ));
    intent
}

fn non_manifold_closed_shell_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
    )
    .expect("build non-manifold closed shell intent");
    intent.mutations.retain(|mutation| {
        !matches!(
            mutation,
            TopologyMutation::CreateRelation { create_key, .. }
                if create_key.as_str().ends_with("solid_shell.base_half_edge.1.radial")
        )
    });
    intent
}

fn broken_loop_wiring_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::WireClosed { half_edge_count: 4 },
    )
    .expect("build broken loop wiring intent");
    intent.mutations.retain(|mutation| {
        !matches!(
            mutation,
            TopologyMutation::CreateRelation { create_key, .. }
                if create_key.as_str().ends_with("wire_closed.half_edge.0.prev")
        )
    });
    intent
}

fn broken_radial_ring_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("build broken radial ring intent");
    intent.mutations.retain(|mutation| {
        !matches!(
            mutation,
            TopologyMutation::CreateRelation { create_key, .. }
                if create_key.as_str().ends_with("nmt_edge_fan.shared_half_edge.0.radial")
        )
    });
    intent
}

fn open_boundary_solid_shell_intent(stem: &str) -> RawTopologyIntent {
    let mut intent = build_milestone_one_primitive_intent(
        stem,
        &MilestoneOnePrimitiveCase::SolidShell { face_count: 4 },
    )
    .expect("build open boundary solid shell intent");
    intent.mutations.retain(|mutation| {
        !matches!(
            mutation,
            TopologyMutation::CreateRelation { create_key, .. }
                if create_key.as_str().ends_with("solid_shell.base_half_edge.0.radial")
        )
    });
    intent
}

fn topology_relation(
    stem: &str,
    key: &str,
    kind: TopologyRelationKind,
    source: &str,
    target: &str,
) -> TopologyMutation {
    TopologyMutation::CreateRelation {
        create_key: CreateKey::new(format!("{stem}.{key}")),
        kind: RelationKind::Topology(kind),
        source: EntityReference::Created(CreateKey::new(format!("{stem}.{source}"))),
        target: EntityReference::Created(CreateKey::new(format!("{stem}.{target}"))),
    }
}
