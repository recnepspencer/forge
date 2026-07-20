use std::sync::Arc;

use crate::facade::RuntimeBridge;
use crate::mapping::{BridgeAuthoritativeSourcePrecisionPolicy, SliceWideningPolicy};

use super::admission::CorrespondenceAdmissionOutcome;
use super::mapping_admission::{admit_mapping, mapping_for};
use super::{
    BridgeCorrespondenceDenialKind, BridgeCorrespondencePrecision,
    BridgeSignalAspectTargetDeclaration,
};

pub(super) struct MappedTarget {
    pub(super) declaration: BridgeSignalAspectTargetDeclaration,
    pub(super) mapping_identity: Arc<str>,
    pub(super) precision: BridgeCorrespondencePrecision,
    pub(super) widening_policy: SliceWideningPolicy,
    pub(super) admitted_source_widening:
        Option<crate::input::envelope::BridgeAspectChangeWideningCause>,
}

pub(super) struct MappedCorrespondence {
    pub(super) resolved: super::resolution::ResolvedCorrespondence,
    pub(super) targets: Vec<MappedTarget>,
}

pub(super) fn map_targets(
    runtime: &RuntimeBridge,
    mut resolved: super::resolution::ResolvedCorrespondence,
) -> Result<MappedCorrespondence, CorrespondenceAdmissionOutcome> {
    let mut targets = Vec::with_capacity(resolved.declarations.len());
    for declaration in std::mem::take(&mut resolved.declarations) {
        resolved.counters.mapping_lookups += 1;
        let Some(mapping) = mapping_for(runtime, &declaration) else {
            return Err(super::admission::denied(
                BridgeCorrespondenceDenialKind::MissingMapping,
                resolved.counters,
            ));
        };
        let mapping_precision = admit_mapping(mapping, resolved.recipe.payload())
            .map_err(|kind| super::admission::denied(kind, resolved.counters))?;
        let admitted_source_widening = match mapping.source_precision_policy() {
            BridgeAuthoritativeSourcePrecisionPolicy::ExactOnly => None,
            BridgeAuthoritativeSourcePrecisionPolicy::AdmitDeclared(cause) => Some(cause),
        };
        let precision = if mapping_precision == BridgeCorrespondencePrecision::DeclaredWidening
            || admitted_source_widening.is_some()
        {
            BridgeCorrespondencePrecision::DeclaredWidening
        } else {
            BridgeCorrespondencePrecision::Exact
        };
        count_precision(&mut resolved.counters, precision, mapping.widening_policy());
        if let Some(cause) = admitted_source_widening {
            count_source_widening(&mut resolved.counters, cause);
        }
        targets.push(MappedTarget {
            declaration,
            mapping_identity: mapping.identity_basis().clone(),
            precision,
            widening_policy: mapping.widening_policy(),
            admitted_source_widening,
        });
    }
    Ok(MappedCorrespondence { resolved, targets })
}

fn count_precision(
    counters: &mut super::CorrespondenceAdmissionCounters,
    precision: BridgeCorrespondencePrecision,
    policy: SliceWideningPolicy,
) {
    match precision {
        BridgeCorrespondencePrecision::Exact => counters.exact_matches += 1,
        BridgeCorrespondencePrecision::DeclaredWidening => counters.widened_matches += 1,
    }
    match policy {
        SliceWideningPolicy::Disallow => {}
        SliceWideningPolicy::RegisteredEntityCoarseWidening => counters.entity_widened_matches += 1,
        SliceWideningPolicy::RegisteredAspectCoarseWidening => counters.aspect_widened_matches += 1,
        SliceWideningPolicy::RegisteredSurfaceCoarseWidening => {
            counters.surface_widened_matches += 1
        }
        SliceWideningPolicy::RegisteredPartitionWidening => counters.partition_widened_matches += 1,
    }
}

fn count_source_widening(
    counters: &mut super::CorrespondenceAdmissionCounters,
    cause: crate::input::envelope::BridgeAspectChangeWideningCause,
) {
    use crate::input::envelope::BridgeAspectChangeWideningCause as Cause;
    match cause {
        Cause::FieldToWholeAspect => counters.field_to_whole_source_admissions += 1,
        Cause::AspectToEntity => counters.aspect_to_entity_source_admissions += 1,
        Cause::SurfaceBroadening => counters.surface_broadening_source_admissions += 1,
        Cause::OpaquePayloadToWholeAspect => counters.opaque_source_admissions += 1,
    }
}
