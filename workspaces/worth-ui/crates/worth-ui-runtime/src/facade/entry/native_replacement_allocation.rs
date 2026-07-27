use std::collections::BTreeSet;

use worth_ui_host_contract::{UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::{
    UiMountedAllocationMeasurementRequest, WorthUiActiveApplicationSession,
    WorthUiMountedAllocationEstablishmentDenial, WorthUiPreparedApplicationReplacement,
};

struct NativeReplacementAllocationCandidate {
    declaration: crate::declaration::UiDeclarationIdentity,
    node: crate::graph::UiGraphNodeIdentity,
    selected: crate::obligations::selection::UiSelectedObligationSet,
    transition: crate::graph::UiGraphMountEligibilityTransition,
    policy: crate::declaration::UiDeclaredMeasurementPolicyPosture,
}

impl WorthUiActiveApplicationSession {
    pub(super) fn admit_native_replacement_allocation_catalog(
        &self,
        prepared: &mut WorthUiPreparedApplicationReplacement,
    ) -> Result<
        crate::graph::UiAdmittedAllocationCatalogDelta,
        WorthUiMountedAllocationEstablishmentDenial,
    > {
        if self.mounted.has_active_presentation_attempt() {
            return Err(WorthUiMountedAllocationEstablishmentDenial::PresentationInFlight);
        }
        let candidates = candidate_inputs(prepared)?;
        prepared
            .commit_candidate_mount_eligibility_admissions(
                candidates
                    .iter()
                    .map(|candidate| candidate.transition)
                    .collect(),
            )
            .map_err(WorthUiMountedAllocationEstablishmentDenial::GraphMountEligibility)?;
        let entries = self.collect_native_replacement_entries(prepared, &candidates)?;
        let uncovered = candidates
            .iter()
            .map(|candidate| candidate.node)
            .collect::<BTreeSet<_>>();
        let partition = disjoint_partition(prepared, entries, uncovered)?;
        prepared
            .admit_candidate_allocation_catalog_delta(partition, Vec::new())
            .map_err(WorthUiMountedAllocationEstablishmentDenial::CandidateCatalogAdmission)
    }

    fn collect_native_replacement_entries(
        &self,
        prepared: &WorthUiPreparedApplicationReplacement,
        candidates: &[NativeReplacementAllocationCandidate],
    ) -> Result<
        Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
        WorthUiMountedAllocationEstablishmentDenial,
    > {
        let graph = prepared.candidate_graph();
        let capability = self.host_session.measurement_capability();
        let collector = self.application.host_measurement_collector();
        let generation = UiEvidenceAuthorityGeneration::new(graph.generation().as_u64());
        let assumptions = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
            capability.capability_report(),
            1,
            2,
            3,
            4,
        );
        let requests = [UiMountedAllocationMeasurementRequest::new(
            UiMeasurementEvidenceFamily::ViewportExtent,
            crate::host::UiHostMeasurementNeed::ViewportExtent(
                worth_ui_host_contract::UiViewportExtentRequest,
            ),
            crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(assumptions),
        )];
        let mut entries = Vec::with_capacity(candidates.len());
        let mut request_ordinal = 0_u64;
        for candidate in candidates {
            let mut inputs = vec![
                crate::evidence::MeasurementEvidenceInput::host_capability_report(
                    capability.capability_report(),
                ),
            ];
            for family in required_host_families(&candidate.policy) {
                let request = requests
                    .iter()
                    .find(|request| request.evidence_family == family)
                    .ok_or(
                        WorthUiMountedAllocationEstablishmentDenial::MissingMeasurementRequest(
                            family,
                        ),
                    )?;
                let identity = 9_000_u64.checked_add(request_ordinal).ok_or(
                    WorthUiMountedAllocationEstablishmentDenial::MeasurementRequestIdentityExhausted,
                )?;
                request_ordinal = request_ordinal.checked_add(1).ok_or(
                    WorthUiMountedAllocationEstablishmentDenial::MeasurementRequestIdentityExhausted,
                )?;
                let result = collector
                    .collect(
                        capability.adapter(),
                        crate::host::UiHostMeasurementCollectionInput {
                            identity: UiMeasurementRequestIdentity::new(identity),
                            evidence_family: request.evidence_family,
                            need: request.need.clone(),
                            capability_report: capability.capability_report(),
                            evidence_generation: generation,
                            normalization_context: request.normalization_context,
                        },
                    )
                    .map_err(WorthUiMountedAllocationEstablishmentDenial::HostMeasurement)?;
                inputs.push(
                    crate::evidence::MeasurementEvidenceInput::host_measurement_result(&result),
                );
            }
            let basis = crate::evidence::admit_measurement_basis(
                candidate.declaration.clone(),
                candidate.node,
                graph.world_profile().clone(),
                generation,
                &candidate.policy,
                &inputs,
            );
            if let Some(denial) = basis.denial_posture().cloned() {
                return Err(
                    WorthUiMountedAllocationEstablishmentDenial::MeasurementBasis {
                        node: candidate.node,
                        denial,
                    },
                );
            }
            entries.push((basis, candidate.selected.clone()));
        }
        Ok(entries)
    }
}

fn candidate_inputs(
    prepared: &WorthUiPreparedApplicationReplacement,
) -> Result<Vec<NativeReplacementAllocationCandidate>, WorthUiMountedAllocationEstablishmentDenial>
{
    let mut candidates = Vec::new();
    for node in prepared.candidate_graph().node_identities() {
        let graph = prepared.candidate_graph();
        let record = graph
            .lookup()
            .graph_node(node)
            .expect("candidate graph node remains addressable");
        let declaration = record.value().declaration_identity().clone();
        let Some(policy) = prepared
            .candidate_declaration_artifacts()
            .iter()
            .find(|artifact| artifact.identity() == &declaration)
            .and_then(|artifact| artifact.graph_handoff().ok())
            .and_then(|handoff| handoff.measurement_policy().admitted().cloned())
        else {
            continue;
        };
        let touch = prepared
            .try_candidate_allocation_touch_for_node(node)
            .map_err(WorthUiMountedAllocationEstablishmentDenial::CandidateTouch)?;
        let selected = prepared.candidate_admission().select_obligations(&touch);
        let prior = record
            .value()
            .participation_posture()
            .axis(crate::graph::UiGraphParticipationAxis::Mounted);
        let transition = graph
            .mount_eligibility_transition_for_node(
                node,
                prior,
                crate::graph::UiGraphAxisParticipation::runtime_mutation(
                    crate::graph::UiGraphParticipationStatus::Admitted,
                ),
            )
            .ok_or(
                WorthUiMountedAllocationEstablishmentDenial::MissingCandidateMountTransition(node),
            )?;
        candidates.push(NativeReplacementAllocationCandidate {
            declaration,
            node,
            selected,
            transition,
            policy,
        });
    }
    if candidates.is_empty() {
        Err(WorthUiMountedAllocationEstablishmentDenial::NoAllocationPlanningNodes)
    } else {
        Ok(candidates)
    }
}

fn required_host_families(
    policy: &crate::declaration::UiDeclaredMeasurementPolicyPosture,
) -> BTreeSet<UiMeasurementEvidenceFamily> {
    let mut families = BTreeSet::new();
    if policy.requires_viewport_extent_observation() {
        families.insert(UiMeasurementEvidenceFamily::ViewportExtent);
    }
    if policy.requires_portal_anchor_observation() {
        families.insert(UiMeasurementEvidenceFamily::PortalAnchorRect);
    }
    for requirement in policy.evidence_requirements() {
        match requirement {
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => {
                families.insert(UiMeasurementEvidenceFamily::FontMetrics);
            }
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
                families.insert(UiMeasurementEvidenceFamily::ScrollContainerViewport);
            }
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => {
                families.insert(UiMeasurementEvidenceFamily::PortalAnchorRect);
            }
        }
    }
    families
}

fn disjoint_partition(
    prepared: &WorthUiPreparedApplicationReplacement,
    mut remaining: Vec<(
        crate::evidence::UiMeasurementBasis,
        crate::obligations::selection::UiSelectedObligationSet,
    )>,
    mut uncovered: BTreeSet<crate::graph::UiGraphNodeIdentity>,
) -> Result<
    Vec<(
        crate::evidence::UiMeasurementBasis,
        crate::obligations::selection::UiSelectedObligationSet,
    )>,
    WorthUiMountedAllocationEstablishmentDenial,
> {
    let mut partition = Vec::new();
    while !uncovered.is_empty() {
        let chosen = remaining
            .iter()
            .enumerate()
            .filter_map(|(index, (basis, selected))| {
                let neighborhood = prepared
                    .admit_candidate_allocation_neighborhood(basis, selected)
                    .ok()?;
                let covered = neighborhood
                    .members()
                    .iter()
                    .map(|member| member.graph_node_identity())
                    .collect::<BTreeSet<_>>();
                covered
                    .iter()
                    .all(|identity| uncovered.contains(identity))
                    .then_some((index, covered))
            })
            .max_by_key(|(_, covered)| covered.len())
            .ok_or(WorthUiMountedAllocationEstablishmentDenial::CandidateAllocationPartition)?;
        for identity in chosen.1 {
            uncovered.remove(&identity);
        }
        partition.push(remaining.swap_remove(chosen.0));
    }
    Ok(partition)
}
