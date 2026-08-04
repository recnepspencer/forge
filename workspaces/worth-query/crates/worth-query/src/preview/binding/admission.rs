use crate::basis::{BasisAuthorityFamily, ExecutionPreflightBundle};
use crate::collection::{
    AggregateFunctionFamily, CollectionResultFamily, DerivedFieldComputationClass,
};
use crate::preview::binding::accounting::PreviewBindingCounters;
use crate::preview::binding::contract::{
    PreviewBindingReport, PreviewLifecycleMetadata, PreviewSessionBasis,
    PreviewSessionBindingTuple, PreviewSessionPlanBinding,
};
use crate::preview::binding::error::{PreviewBindingError, PreviewBindingFailureClass};
use crate::preview::evaluation::PreviewEvaluationClass;
use crate::preview::session_context::PreviewSessionQueryContext;
use crate::preview::workflow_context_identity;
use worth_runtime_bridge::facade::BridgePreviewLifecycleStateKind;

pub(crate) fn bind_preflight_to_preview_session(
    preflight: ExecutionPreflightBundle,
    query_context: PreviewSessionQueryContext,
) -> Result<PreviewSessionPlanBinding, PreviewBindingError> {
    reject_unsupported_preview_family(&preflight)?;

    if matches!(
        preflight.basis().identity().authority_family(),
        BasisAuthorityFamily::Store
    ) {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::StoreBackedRouteForbidden,
            "preview binding requires runtime basis authority",
            counters,
        ));
    }

    let source = query_context.source.snapshot();
    if source.lifecycle_state_kind != BridgePreviewLifecycleStateKind::Active {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_lifecycle_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle,
            "preview lifecycle must be active before binding",
            counters,
        ));
    }

    if source.execution_record_identity.is_none() || source.execution_record_digest.is_none() {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_lifecycle_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::MissingExecutionRecordIdentity,
            "active preview binding requires an explicit preview execution record",
            counters,
        ));
    }

    if let Some(execution_record_digest) = source.execution_record_digest.as_ref() {
        if execution_record_digest.is_empty() {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            counters.preview_broad_fallback_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record digest must not be empty",
                counters,
            ));
        }
    }

    if let Some(execution_record_session_identity) =
        source.execution_record_preview_session_identity.as_ref()
    {
        if execution_record_session_identity
            != workflow_context_identity::preview_session_identity_record_label(
                &source.preview_session_identity,
            )
        {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            counters.preview_broad_fallback_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record must belong to the requested preview session",
                counters,
            ));
        }
    }

    if let Some(execution_record_declaration_digest) =
        source.execution_record_declaration_digest.as_ref()
    {
        if execution_record_declaration_digest != &source.declaration_digest {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            counters.preview_broad_fallback_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record must match the requested preview declaration digest",
                counters,
            ));
        }
    }

    if let (Some(execution_record_identity), Some(session_execution_record_identity)) = (
        source.execution_record_identity.as_ref(),
        source.session_execution_record_identity.as_ref(),
    ) {
        if execution_record_identity != session_execution_record_identity {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            counters.preview_broad_fallback_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record identity must match the active preview session identity",
                counters,
            ));
        }
    }

    if matches!(
        query_context.evaluation_class(),
        PreviewEvaluationClass::ReadOnly(_)
    ) && query_context.promotion_record.is_some()
    {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        counters.preview_bridge_promotion_linkage_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::PromotionLinkageMismatch,
            "read-only preview evaluation cannot carry promotion linkage",
            counters,
        ));
    }

    if query_context.promotion_record.is_some() || query_context.replay_bundle.is_some() {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        counters.preview_bridge_promotion_linkage_count =
            usize::from(query_context.promotion_record.is_some());
        counters.preview_replay_bundle_lookup_count =
            usize::from(query_context.replay_bundle.is_some());
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::PromotionLinkageMismatch,
            "phase 1-2 preview binding does not admit replay or promotion linkage on active sessions",
            counters,
        ));
    }

    let counters = PreviewBindingCounters::for_admitted_path();
    let lifecycle_metadata = PreviewLifecycleMetadata {
        lifecycle_state_kind: source.lifecycle_state_kind,
        execution_record_identity: source.execution_record_identity.clone(),
        replay_bundle_digest: query_context
            .replay_bundle
            .as_ref()
            .map(|bundle| bundle.digest.clone()),
        promotion_record_identity: query_context
            .promotion_record
            .as_ref()
            .map(|record| record.record_identity.clone()),
        promotion_proof_digest: query_context
            .promotion_record
            .as_ref()
            .map(|record| record.proof_digest.clone()),
    };
    let binding_tuple = PreviewSessionBindingTuple::new(
        preflight.plan().query().canonical_query_digest().clone(),
        preflight
            .plan()
            .result_shape()
            .canonical_result_shape_digest()
            .clone(),
        preflight.plan().query().validated_query_digest().clone(),
        preflight
            .plan()
            .result_shape()
            .validated_result_shape_digest()
            .clone(),
        query_context.evaluation_class.clone(),
        source.preview_session_identity.clone(),
        source.declaration_identity.clone(),
        source.declaration_digest.clone(),
        source.lifecycle_state_kind,
        source.execution_record_identity.clone(),
        lifecycle_metadata.replay_bundle_digest().map(str::to_owned),
        lifecycle_metadata
            .promotion_record_identity()
            .map(str::to_owned),
        lifecycle_metadata
            .promotion_proof_digest()
            .map(str::to_owned),
    );
    let basis = PreviewSessionBasis::new(binding_tuple.clone());
    let report = PreviewBindingReport::new(
        binding_tuple.digest().to_string(),
        query_context.evaluation_class.clone(),
        counters,
    );

    Ok(PreviewSessionPlanBinding {
        preflight,
        query_context,
        basis,
        lifecycle_metadata,
        report,
    })
}

fn reject_unsupported_preview_family(
    preflight: &ExecutionPreflightBundle,
) -> Result<(), PreviewBindingError> {
    let Some(collection) = preflight.plan().collection() else {
        return Ok(());
    };

    let unsupported_family = matches!(
        collection.planning_context().result_family(),
        CollectionResultFamily::CdcCollection
    ) || !matches!(
        collection
            .post_read_shaping()
            .aggregate_shape()
            .function_family(),
        AggregateFunctionFamily::NoneAdmittedYet
    ) || !matches!(
        collection
            .post_read_shaping()
            .derived_field_plan()
            .computation_class(),
        DerivedFieldComputationClass::NoneAdmittedYet
    );

    if unsupported_family {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::UnsupportedPreviewQueryFamily,
            "preview binding only admits detail, ordinary collection, and bounded materialization families in phases 1-2",
            counters,
        ));
    }

    Ok(())
}
