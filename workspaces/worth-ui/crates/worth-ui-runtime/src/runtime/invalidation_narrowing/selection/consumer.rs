use crate::runtime::{
    UiAllocationFrameSourceFact, UiAllocationInvalidationFamily, UiResolvedAllocationFramePlan,
};

use super::consumer_support::{enforce_drag_resize_target_budget, map_lookup_denial, target_count};
use super::{
    UiAllocationInvalidationNarrowingCounters, UiAllocationInvalidationNarrowingDenial,
    UiAllocationInvalidationNarrowingRejection, UiAllocationInvalidationTarget,
    UiNarrowedAllocationFramePlan, UiNarrowedAllocationInvalidation,
};

pub(crate) enum UiAllocationInvalidationNarrowingDisposition {
    Accepted(UiNarrowedAllocationFramePlan),
    Rejected(UiAllocationInvalidationNarrowingRejection),
}

pub(crate) fn narrow_resolved_frame(
    plan: UiResolvedAllocationFramePlan,
    authority: &std::cell::RefCell<super::UiAllocationInvalidationAuthority>,
) -> UiAllocationInvalidationNarrowingDisposition {
    let epoch = plan.frame_epoch();
    let target_ceiling = plan.policy().budget().max_invalidation_targets();
    let invalidation_families = plan
        .invalidations()
        .iter()
        .map(|intent| intent.family())
        .collect::<Vec<_>>();
    let (identity, sources) = plan.into_narrowing_parts();
    let mut counters = UiAllocationInvalidationNarrowingCounters::default();
    if invalidation_families.len() != sources.len() {
        let Ok(invalidations) = u16::try_from(invalidation_families.len()) else {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::CardinalityExhausted,
                counters,
            );
        };
        let Ok(source_count) = u16::try_from(sources.len()) else {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::CardinalityExhausted,
                counters,
            );
        };
        return rejected(
            epoch,
            UiAllocationInvalidationNarrowingDenial::SourceCardinalityMismatch {
                invalidations,
                sources: source_count,
            },
            counters,
        );
    }

    let mut narrowed = Vec::with_capacity(sources.len());
    let last_preview = invalidation_families
        .iter()
        .rposition(|family| *family == UiAllocationInvalidationFamily::ResizePreviewDelta);
    let last_durable = invalidation_families
        .iter()
        .rposition(|family| *family == UiAllocationInvalidationFamily::DurableLocalResizeChange);
    for (ordinal_index, (family, source)) in invalidation_families
        .into_iter()
        .zip(sources.into_vec())
        .enumerate()
    {
        let Ok(ordinal) = u16::try_from(ordinal_index) else {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::OrdinalExhausted,
                counters,
            );
        };
        if counters.visit_invalidation().is_err() {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
                counters,
            );
        }
        if (family == UiAllocationInvalidationFamily::ResizePreviewDelta
            && last_preview != Some(ordinal_index))
            || (family == UiAllocationInvalidationFamily::DurableLocalResizeChange
                && last_durable != Some(ordinal_index))
        {
            continue;
        }
        let ingress_key = &identity.ingress_keys()[ordinal_index];
        let target = match narrow_source(
            family,
            source,
            ingress_key,
            ordinal,
            authority,
            target_ceiling,
            &mut counters,
        ) {
            Ok(target) => target,
            Err(denial) => return rejected(epoch, denial, counters),
        };
        let Some(target_count) = target_count(&target) else {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
                counters,
            );
        };
        if counters.emit_targets(target_count).is_err() {
            return rejected(
                epoch,
                UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
                counters,
            );
        }
        let admitted_family = if matches!(
            target,
            UiAllocationInvalidationTarget::ScrollOwnedExtent { .. }
        ) {
            UiAllocationInvalidationFamily::ScrollOwnedExtentChange
        } else {
            family
        };
        narrowed.push(UiNarrowedAllocationInvalidation::new(
            admitted_family,
            target,
        ));
    }

    UiAllocationInvalidationNarrowingDisposition::Accepted(UiNarrowedAllocationFramePlan::new(
        identity,
        narrowed.into_boxed_slice(),
        counters,
    ))
}

fn narrow_source(
    family: UiAllocationInvalidationFamily,
    source: UiAllocationFrameSourceFact,
    ingress_key: &crate::runtime::UiAllocationFrameIngressKey,
    ordinal: u16,
    authority: &std::cell::RefCell<super::UiAllocationInvalidationAuthority>,
    target_ceiling: u16,
    counters: &mut UiAllocationInvalidationNarrowingCounters,
) -> Result<UiAllocationInvalidationTarget, UiAllocationInvalidationNarrowingDenial> {
    match source {
        UiAllocationFrameSourceFact::Interaction(interaction)
            if matches!(
                family,
                UiAllocationInvalidationFamily::TextContentChange
                    | UiAllocationInvalidationFamily::ResizePreviewDelta
            ) =>
        {
            counted(counters.lookup_graph_target(), ordinal)?;
            let lookup = authority
                .borrow()
                .graph_target(interaction.target())
                .map_err(|denial| map_lookup_denial(denial, ordinal))?;
            counted(counters.record_authority_probes(lookup.probes), ordinal)?;
            let target = lookup.target.ok_or(
                UiAllocationInvalidationNarrowingDenial::GraphTargetNotAdmitted { ordinal },
            )?;
            if family == UiAllocationInvalidationFamily::ResizePreviewDelta {
                enforce_drag_resize_target_budget(
                    ordinal,
                    target.neighborhood_count(),
                    target_ceiling,
                    counters,
                )?;
                let sample = interaction.resize_preview().ok_or(
                    UiAllocationInvalidationNarrowingDenial::SourceFamilyMismatch { ordinal },
                )?;
                Ok(UiAllocationInvalidationTarget::ResizePreview { sample, target })
            } else {
                Ok(UiAllocationInvalidationTarget::Graph(target))
            }
        }
        UiAllocationFrameSourceFact::QueryProjection { source, .. }
            if matches!(
                family,
                UiAllocationInvalidationFamily::QueryMeasurementFactChange
                    | UiAllocationInvalidationFamily::ContentExtentChange
            ) =>
        {
            counted(counters.visit_query_settlement(), ordinal)?;
            let basis = source.allocation_invalidation_basis();
            if ingress_key.source_generation().as_u64() != basis.source_generation().as_u64() {
                return Err(
                    UiAllocationInvalidationNarrowingDenial::QuerySourceGenerationMismatch {
                        ordinal,
                    },
                );
            }
            if ingress_key.source_order().as_u64() != basis.source_order().as_u64() {
                return Err(
                    UiAllocationInvalidationNarrowingDenial::QuerySourceOrderMismatch { ordinal },
                );
            }
            if ingress_key.ingress_identity().as_u64() != source.allocation_ingress_identity() {
                return Err(
                    UiAllocationInvalidationNarrowingDenial::QueryConsumptionReceiptMismatch {
                        ordinal,
                    },
                );
            }
            if basis.consumed_families().is_empty() {
                return Err(
                    UiAllocationInvalidationNarrowingDenial::QuerySettlementFamilyMissing {
                        ordinal,
                    },
                );
            }
            counted(
                counters.visit_query_observations(basis.observations().len()),
                ordinal,
            )?;
            counted(counters.lookup_graph_target(), ordinal)?;
            if basis
                .observations()
                .iter()
                .any(|value| value.extent().as_f32().is_nan())
            {
                return Err(
                    UiAllocationInvalidationNarrowingDenial::QueryExtentUnordered { ordinal },
                );
            }
            if family == UiAllocationInvalidationFamily::ContentExtentChange {
                let authority_ref = authority.borrow();
                let scroll_lookup = authority_ref.scroll_query_target(
                    basis.query_authority(),
                ).map_err(|denial| {
                    if counters.record_authority_probes(denial.probes).is_err() {
                        return UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal };
                    }
                    match denial.reason {
                    super::authority::UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted =>
                        UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
                    super::authority::UiInvalidationAuthorityLookupDenial::QueryAuthorityNotIndexable =>
                        UiAllocationInvalidationNarrowingDenial::QueryAuthorityNotIndexable { ordinal },
                    _ => unreachable!("Query lookup cannot return host denial"),
                }})?;
                counted(
                    counters.record_authority_probes(scroll_lookup.probes()),
                    ordinal,
                )?;
                if scroll_lookup.is_empty() {
                    return Err(
                        UiAllocationInvalidationNarrowingDenial::ScrollOwnershipNotAdmitted {
                            ordinal,
                        },
                    );
                }
                return Ok(UiAllocationInvalidationTarget::ScrollOwnedContentExtent {
                    basis,
                    bindings: scroll_lookup.materialize_bindings(),
                });
            }
            let lookup = authority
                .borrow()
                .query_target(basis.query_authority())
                .map_err(|denial| match denial {
                    super::authority::UiInvalidationAuthorityLookupDenial::AuthorityCounterExhausted =>
                        UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
                    super::authority::UiInvalidationAuthorityLookupDenial::QueryAuthorityNotIndexable =>
                        UiAllocationInvalidationNarrowingDenial::QueryAuthorityNotIndexable { ordinal },
                    super::authority::UiInvalidationAuthorityLookupDenial::HostEvidenceGenerationMismatch
                    | super::authority::UiInvalidationAuthorityLookupDenial::HostNormalizationAuthorityMismatch
                    => unreachable!("Query lookup cannot return host denial"),
                })?;
            counted(counters.record_authority_probes(lookup.probes), ordinal)?;
            let target = lookup.target.ok_or(
                UiAllocationInvalidationNarrowingDenial::QueryTargetNotAdmitted { ordinal },
            )?;
            Ok(UiAllocationInvalidationTarget::QueryProjection { basis, target })
        }
        UiAllocationFrameSourceFact::HostMeasurement(measurement)
            if matches!(
                family,
                UiAllocationInvalidationFamily::ViewportExtentChange
                    | UiAllocationInvalidationFamily::ScrollExtentObservation
                    | UiAllocationInvalidationFamily::ScrollOwnedExtentChange
                    | UiAllocationInvalidationFamily::PortalAnchorMovement
                    | UiAllocationInvalidationFamily::HostMeasurementResultReplacement
            ) =>
        {
            let authority = authority.borrow();
            super::host_measurement_narrowing::narrow_host_measurement(
                family,
                &measurement,
                ordinal,
                &authority,
                target_ceiling,
                counters,
            )
        }
        UiAllocationFrameSourceFact::DurableResize(source)
            if family == UiAllocationInvalidationFamily::DurableLocalResizeChange =>
        {
            counted(counters.lookup_graph_target(), ordinal)?;
            let lookup = authority
                .borrow()
                .durable_target(source.input().identity_digest())
                .map_err(|denial| map_lookup_denial(denial, ordinal))?;
            counted(counters.record_authority_probes(lookup.probes), ordinal)?;
            lookup
                .target
                .map(|target| {
                    enforce_drag_resize_target_budget(
                        ordinal,
                        target.neighborhood_count(),
                        target_ceiling,
                        counters,
                    )?;
                    Ok(UiAllocationInvalidationTarget::DurableResize {
                        identity_digest: source.input().identity_digest(),
                        extent: source.extent(),
                        target,
                    })
                })
                .transpose()?
                .ok_or(
                    UiAllocationInvalidationNarrowingDenial::DurableResizeTargetNotAdmitted {
                        ordinal,
                    },
                )
        }
        _ => Err(UiAllocationInvalidationNarrowingDenial::SourceFamilyMismatch { ordinal }),
    }
}

fn counted(
    result: Result<(), ()>,
    ordinal: u16,
) -> Result<(), UiAllocationInvalidationNarrowingDenial> {
    result.map_err(
        |()| UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
    )
}

fn rejected(
    epoch: crate::runtime::UiAllocationFrameEpoch,
    denial: UiAllocationInvalidationNarrowingDenial,
    counters: UiAllocationInvalidationNarrowingCounters,
) -> UiAllocationInvalidationNarrowingDisposition {
    UiAllocationInvalidationNarrowingDisposition::Rejected(
        UiAllocationInvalidationNarrowingRejection::new(epoch, denial, counters),
    )
}
