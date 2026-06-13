use super::adversarial_evidence::{
    OpenClassLaneAuthorityEvidence, OpenClassStormExtractionEvidence,
};
use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};
use super::lane_set::OpenClassParityLaneSet;
use super::open_class::OpenTopologyClass;
use super::receipt::OpenClassTriadParityReceipt;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;

impl OpenClassTriadParityReceipt {
    pub fn attempt_cross_class_checkpoint_replay(
        &self,
        source: OpenTopologyClass,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        let source_evidence = OpenClassLaneAuthorityEvidence::retained_checkpoint_from_lane_set(
            require_class(self, source)?,
        )
        .ok_or_else(|| missing_lane(source, ProjectionFactParityLane::Retained))?;
        self.attempt_cross_class_checkpoint_replay_evidence(&source_evidence, target)
    }

    pub fn attempt_cross_class_checkpoint_replay_evidence(
        &self,
        source: &OpenClassLaneAuthorityEvidence,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        if source.lane() != ProjectionFactParityLane::Retained {
            return Err(OpenClassTriadParityDenial::with_source(
                OpenClassTriadParityDenialKind::ProjectionConsumptionMismatch,
                source.topology_class(),
                target,
                Some(source.lane()),
                format!(
                    "{} supplied the {} where retained checkpoint evidence is required.",
                    source.topology_class().human_name(),
                    source.lane().human_name()
                ),
            ));
        }
        let target_set = require_class(self, target)?;
        let source_class = source.topology_class();
        if source_class == target && source.topology_identity() == target_set.topology_identity() {
            return Ok(());
        }
        Err(OpenClassTriadParityDenial::with_source(
            OpenClassTriadParityDenialKind::CrossClassCheckpointReplay,
            source_class,
            target,
            Some(ProjectionFactParityLane::Retained),
            format!(
                "Retained checkpoint from {} cannot satisfy {} parity; source retained lane {} targets topology {}.",
                source_class.human_name(),
                target.human_name(),
                source.evidence_identity(),
                target_set.topology_identity()
            ),
        ))
    }

    pub fn attempt_projection_consumed_as_retained(
        &self,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        let target_set = require_class(self, target)?;
        let evidence =
            OpenClassLaneAuthorityEvidence::projection_consumed_from_lane_set(target_set)
                .ok_or_else(|| {
                    missing_lane(target, ProjectionFactParityLane::ProjectionConsumed)
                })?;
        self.attempt_projection_consumed_as_retained_evidence(&evidence, target)
    }

    pub fn attempt_projection_consumed_as_retained_evidence(
        &self,
        evidence: &OpenClassLaneAuthorityEvidence,
        target: OpenTopologyClass,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::with_source(
            OpenClassTriadParityDenialKind::ProjectionConsumptionMismatch,
            evidence.topology_class(),
            target,
            Some(evidence.lane()),
            format!(
                "Projection-consumed facts for {} cannot be replayed as retained checkpoint evidence; projection lane {} is checked at the projection-consumption boundary.",
                evidence.topology_class().human_name(),
                evidence.evidence_identity()
            ),
        ))
    }

    pub fn attempt_storm_extraction_bundle_link(
        &self,
        target: OpenTopologyClass,
        storm_bundle_digest: &str,
    ) -> Result<(), OpenClassTriadParityDenial> {
        self.attempt_storm_extraction_bundle_link_evidence(
            target,
            &OpenClassStormExtractionEvidence::from_digest(storm_bundle_digest),
        )
    }

    pub fn attempt_storm_extraction_bundle_link_evidence(
        &self,
        target: OpenTopologyClass,
        evidence: &OpenClassStormExtractionEvidence,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::StormExtractionUnsupported,
            Some(target),
            format!(
                "Closed storm extraction bundle {} is not valid authority for {}; open classes must keep their own topology parity receipts.",
                evidence.projection_stage_identity(),
                target.human_name()
            ),
        ))
    }

    pub fn attempt_denied_lane_upgrade(
        &self,
        target: OpenTopologyClass,
        lane: ProjectionFactParityLane,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::DeniedLaneUpgrade,
            Some(target),
            format!(
                "{} cannot upgrade a denied parity state through the {}.",
                target.human_name(),
                lane.human_name()
            ),
        ))
    }

    pub fn attempt_missing_lane_evidence(
        &self,
        target: OpenTopologyClass,
        lane: ProjectionFactParityLane,
    ) -> Result<(), OpenClassTriadParityDenial> {
        require_class(self, target)?;
        Err(OpenClassTriadParityDenial::new(
            OpenClassTriadParityDenialKind::MissingLaneEvidence,
            Some(target),
            format!(
                "{} has no options without the {}; parity cannot fall back to a generic integrity mismatch.",
                target.human_name(),
                lane.human_name()
            ),
        ))
    }
}

fn missing_lane(
    topology_class: OpenTopologyClass,
    lane: ProjectionFactParityLane,
) -> OpenClassTriadParityDenial {
    OpenClassTriadParityDenial::new(
        OpenClassTriadParityDenialKind::MissingLaneEvidence,
        Some(topology_class),
        format!(
            "{} has no options without the {}.",
            topology_class.human_name(),
            lane.human_name()
        ),
    )
}

fn require_class(
    receipt: &OpenClassTriadParityReceipt,
    topology_class: OpenTopologyClass,
) -> Result<&OpenClassParityLaneSet, OpenClassTriadParityDenial> {
    receipt
        .require_class_for_attempt(topology_class)
        .ok_or_else(|| {
            OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::MissingOpenClass,
                Some(topology_class),
                format!(
                    "Open-class triad parity is missing {} evidence.",
                    topology_class.human_name()
                ),
            )
        })
}
