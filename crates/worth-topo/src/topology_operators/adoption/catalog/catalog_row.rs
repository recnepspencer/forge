use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus, ForgeQueryGraphTouchSelector,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyOperatorGraphObligationLoweringPath {
    GraphComposition,
    ContributionOrchestration,
    AuthoritativeCommandBatch,
    ScalarMutation,
    RelationalInvariantBackstop,
}

impl TopologyOperatorGraphObligationLoweringPath {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphComposition => "graph-composition",
            Self::ContributionOrchestration => "contribution-orchestration",
            Self::AuthoritativeCommandBatch => "authoritative-command-batch",
            Self::ScalarMutation => "scalar-mutation",
            Self::RelationalInvariantBackstop => "relational-invariant-backstop",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyOperatorGraphObligationAdoptionStatus {
    Covered,
    Residue,
}

impl TopologyOperatorGraphObligationAdoptionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::Residue => "residue",
        }
    }
}

#[derive(Clone, Debug)]
pub struct TopologyOperatorGraphObligationCatalogRow {
    operator_family: &'static str,
    touch_meaning: &'static str,
    descriptor_source: &'static str,
    lowering_path: TopologyOperatorGraphObligationLoweringPath,
    adoption_status: TopologyOperatorGraphObligationAdoptionStatus,
    residue_class: Option<&'static str>,
    registration: Option<ForgeQueryGraphObligationRegistration>,
}

impl TopologyOperatorGraphObligationCatalogRow {
    pub(crate) fn covered(
        operator_family: &'static str,
        touch_meaning: &'static str,
        descriptor_source: &'static str,
        lowering_path: TopologyOperatorGraphObligationLoweringPath,
        registration: ForgeQueryGraphObligationRegistration,
    ) -> Self {
        Self {
            operator_family,
            touch_meaning,
            descriptor_source,
            lowering_path,
            adoption_status: TopologyOperatorGraphObligationAdoptionStatus::Covered,
            residue_class: None,
            registration: Some(registration),
        }
    }

    pub(crate) fn residue(
        operator_family: &'static str,
        touch_meaning: &'static str,
        descriptor_source: &'static str,
        lowering_path: TopologyOperatorGraphObligationLoweringPath,
        residue_class: &'static str,
    ) -> Self {
        Self {
            operator_family,
            touch_meaning,
            descriptor_source,
            lowering_path,
            adoption_status: TopologyOperatorGraphObligationAdoptionStatus::Residue,
            residue_class: Some(residue_class),
            registration: None,
        }
    }

    pub fn operator_family(&self) -> &'static str {
        self.operator_family
    }

    pub fn touch_meaning(&self) -> &'static str {
        self.touch_meaning
    }

    pub fn descriptor_source(&self) -> &'static str {
        self.descriptor_source
    }

    pub fn lowering_path(&self) -> TopologyOperatorGraphObligationLoweringPath {
        self.lowering_path
    }

    pub fn adoption_status(&self) -> TopologyOperatorGraphObligationAdoptionStatus {
        self.adoption_status
    }

    pub fn residue_class(&self) -> Option<&'static str> {
        self.residue_class
    }

    pub fn registration(&self) -> Option<&ForgeQueryGraphObligationRegistration> {
        self.registration.as_ref()
    }

    pub fn registration_kind(&self) -> Option<ForgeQueryGraphObligationKind> {
        self.registration.as_ref().map(|value| value.kind())
    }

    pub fn support_lane(&self) -> Option<ForgeQueryGraphObligationSupportLane> {
        self.registration
            .as_ref()
            .map(|value| value.support_posture().lane())
    }

    pub fn support_status(&self) -> Option<ForgeQueryGraphObligationSupportStatus> {
        self.registration
            .as_ref()
            .map(|value| value.support_posture().status())
    }

    pub fn touch_selector(&self) -> Option<&ForgeQueryGraphTouchSelector> {
        self.registration
            .as_ref()
            .map(|value| value.touch_selector())
    }

    pub fn operating_world_selector(
        &self,
    ) -> Option<ForgeQueryGraphObligationOperatingWorldSelector> {
        self.registration
            .as_ref()
            .map(|value| value.operating_world_selector())
    }
}
