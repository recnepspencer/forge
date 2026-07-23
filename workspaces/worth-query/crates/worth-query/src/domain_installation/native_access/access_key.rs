use crate::projection_consumption::ProjectionFactFieldPath;
use worth_foundational::facade::AspectValuePosture;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryNativeFactLane {
    Display,
    Derived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeAccessKey {
    runtime_authority: u64,
    installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    capability_identity: u64,
    selection_identity: u64,
    contract_key: worth_foundational::facade::AspectKey,
    contract_identity: worth_foundational::facade::AspectIdentity,
    contract_revision: worth_foundational::facade::AspectContractRevision,
    field_path: ProjectionFactFieldPath,
    expected_shape: AspectValuePosture,
    absence: worth_foundational::facade::AbsenceLaw,
    lane: WorthQueryNativeFactLane,
    lane_slot: usize,
    lane_width: usize,
}

impl WorthQueryNativeAccessKey {
    pub(super) fn mint(
        runtime_authority: u64,
        installation_generation: super::super::WorthQueryDomainInstallationGeneration,
        capability_identity: u64,
        selection_identity: u64,
        contract: &worth_foundational::facade::AspectContract,
        field_path: ProjectionFactFieldPath,
        expected_shape: AspectValuePosture,
        absence: worth_foundational::facade::AbsenceLaw,
        lane: WorthQueryNativeFactLane,
        lane_slot: usize,
        lane_width: usize,
    ) -> Self {
        Self {
            runtime_authority,
            installation_generation,
            capability_identity,
            selection_identity,
            contract_key: contract.key().clone(),
            contract_identity: contract.identity(),
            contract_revision: contract.revision(),
            field_path,
            expected_shape,
            absence,
            lane,
            lane_slot,
            lane_width,
        }
    }

    pub fn contract_key(&self) -> &worth_foundational::facade::AspectKey {
        &self.contract_key
    }

    pub fn contract_identity(&self) -> worth_foundational::facade::AspectIdentity {
        self.contract_identity
    }

    pub fn contract_revision(&self) -> worth_foundational::facade::AspectContractRevision {
        self.contract_revision
    }

    pub fn field_path(&self) -> &ProjectionFactFieldPath {
        &self.field_path
    }

    pub fn expected_shape(&self) -> AspectValuePosture {
        self.expected_shape
    }

    pub fn lane(&self) -> WorthQueryNativeFactLane {
        self.lane
    }

    pub fn absence_posture(&self) -> worth_foundational::facade::AbsenceLaw {
        self.absence
    }

    pub(super) fn runtime_authority(&self) -> u64 {
        self.runtime_authority
    }

    pub(super) fn installation_generation(
        &self,
    ) -> super::super::WorthQueryDomainInstallationGeneration {
        self.installation_generation
    }

    pub(super) fn capability_identity(&self) -> u64 {
        self.capability_identity
    }

    pub(super) fn selection_identity(&self) -> u64 {
        self.selection_identity
    }

    pub(super) fn lane_slot(&self) -> usize {
        self.lane_slot
    }

    pub(super) fn lane_width(&self) -> usize {
        self.lane_width
    }
}
