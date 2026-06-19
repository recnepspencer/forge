use super::error::TopologyMutationApplicationError;

impl std::fmt::Display for TopologyMutationApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedFamilies(families) => write!(
                f,
                "topology query mutation application does not admit families `{families:?}` yet"
            ),
            Self::DeclarationEntry {
                family,
                stop_class,
                stop_stage,
                refusal_class,
                recovery,
                graph_obligation_envelope_digest,
                reason,
            } => {
                write!(
                    f,
                    "topology query declaration entry orchestration for family `{family:?}` stopped as `{}`",
                    stop_class.as_str(),
                )?;
                if let Some(stop_stage) = stop_stage {
                    write!(f, " at stage `{stop_stage:?}`")?;
                }
                if let Some(refusal_class) = refusal_class {
                    write!(f, " with refusal class `{}`", refusal_class.as_str())?;
                }
                if let Some(recovery) = recovery {
                    write!(
                        f,
                        " owned by `{:?}` recommending `{:?}`",
                        recovery.authority_surface(),
                        recovery.recommended_action()
                    )?;
                }
                if let Some(digest) = graph_obligation_envelope_digest {
                    write!(f, " with graph obligation envelope `{digest}`")?;
                }
                write!(f, ": {reason}")
            }
            Self::MissingCreatedEntityReference(create_key) => write!(
                f,
                "topology query mutation application is missing same-mutation-set created entity `{create_key}`"
            ),
            Self::MissingExistingEntityBinding(entity_id) => write!(
                f,
                "topology query mutation application is missing live query binding for authoritative entity `{entity_id:?}`"
            ),
            Self::MissingExistingRelationBinding(relation_id) => write!(
                f,
                "topology query mutation application is missing live query binding for authoritative relation `{relation_id:?}`"
            ),
            Self::CreatedEntityKindMismatch {
                create_key,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected created entity `{create_key}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingEntityKindMismatch {
                entity_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationKindMismatch {
                relation_id,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative relation `{relation_id:?}` to be `{}`, found `{}`",
                expected.kind_name(),
                actual.kind_name()
            ),
            Self::ExistingRelationSourceMismatch {
                relation_id,
                expected_source_entity_id,
                actual_source_identity,
            } => write!(
                f,
                "topology query mutation application expected authoritative relation `{relation_id:?}` to originate from halfedge `{expected_source_entity_id:?}`, found query source identity `{actual_source_identity}`"
            ),
            Self::ExistingEntityOutgoingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to have exactly {expected} outgoing `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingEntityIncomingRelationCountMismatch {
                entity_id,
                relation_kind,
                expected,
                actual,
            } => write!(
                f,
                "topology query mutation application expected authoritative entity `{entity_id:?}` to have exactly {expected} incoming `{}` relation(s), found {actual}",
                relation_kind.kind_name()
            ),
            Self::ExistingHalfEdgesNotOnSameEdge {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_edge_identity,
                target_edge_identity,
            } => write!(
                f,
                "topology query mutation application expected radial splice relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same edge, found source edge `{source_edge_identity}` and target edge `{target_edge_identity}`"
            ),
            Self::ExistingHalfEdgesNotOnSameLoop {
                relation_id,
                source_half_edge_id,
                target_half_edge_id,
                source_loop_identity,
                target_loop_identity,
            } => write!(
                f,
                "topology query mutation application expected loop-successor relation `{relation_id:?}` to keep halfedges `{source_half_edge_id:?}` and `{target_half_edge_id:?}` on the same loop, found source loop `{source_loop_identity}` and target loop `{target_loop_identity}`"
            ),
            Self::Query(error) => write!(f, "{error}"),
            Self::MaterializedDecode(message) => write!(f, "{message}"),
            Self::QueryAnchorFamilyMismatch {
                semantic_family_key,
                query_declaration_family_key,
            } => write!(
                f,
                "topology query mutation application refused to project local aftermath for semantic family `{semantic_family_key}` from Query declaration family `{query_declaration_family_key}`"
            ),
            Self::RetainedSemanticAftermathMismatch {
                semantic_family_key,
                reason,
            } => write!(
                f,
                "topology query mutation application retained Query semantic aftermath that did not match the declared topology mutation sequence for `{semantic_family_key}`: {reason}"
            ),
        }
    }
}
