use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiDropdownAppearanceRequest, WorthUiDropdownProjectionPlan,
    WorthUiDropdownProjectionPlanDenial, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily,
    WorthUiProjectionIdentity, WorthUiProjectionPlanContract,
};

use super::{
    WorthUiHeaderFrameReceipt, WorthUiHeaderMenuGroup, WorthUiHeaderMenuProjectionRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuPlan {
    receipt: WorthUiHeaderFrameReceipt,
    projection_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
    dropdown_plans: Vec<WorthUiDropdownProjectionPlan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderMenuPlanDenial {
    EmptyProjectionSet,
    Dropdown(WorthUiDropdownProjectionPlanDenial),
}

impl WorthUiHeaderMenuPlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        requests: impl IntoIterator<Item = WorthUiHeaderMenuProjectionRequest>,
        appearance_request: WorthUiDropdownAppearanceRequest,
    ) -> Result<Self, WorthUiHeaderMenuPlanDenial> {
        Self::build(snapshot, requests, appearance_request, None)
    }

    fn build(
        snapshot: &CapabilitySnapshot,
        requests: impl IntoIterator<Item = WorthUiHeaderMenuProjectionRequest>,
        appearance_request: WorthUiDropdownAppearanceRequest,
        previous: Option<&WorthUiHeaderMenuPlan>,
    ) -> Result<Self, WorthUiHeaderMenuPlanDenial> {
        let requests: Vec<_> = requests.into_iter().collect();
        if requests.is_empty() {
            return Err(WorthUiHeaderMenuPlanDenial::EmptyProjectionSet);
        }

        let mut groups = Vec::with_capacity(requests.len());
        let mut dropdown_plans = Vec::with_capacity(requests.len());
        let mut dependencies = WorthUiProjectionDependencySet::empty();
        for request in requests {
            let dropdown_request = request.to_dropdown_request(appearance_request.clone());
            let dropdown_plan = match previous.and_then(|plan| {
                plan.dropdown_plans.iter().find(|candidate| {
                    candidate.execute_frame().projection_id() == request.projection_id().as_str()
                })
            }) {
                Some(previous_dropdown) => WorthUiDropdownProjectionPlan::rebuild_from_snapshot(
                    snapshot,
                    dropdown_request,
                    Some(previous_dropdown.execute_frame().selection_state()),
                ),
                None => WorthUiDropdownProjectionPlan::from_snapshot(snapshot, dropdown_request),
            }
            .map_err(WorthUiHeaderMenuPlanDenial::Dropdown)?;
            dependencies = dependencies.merge(dropdown_plan.dependencies());
            groups.push(WorthUiHeaderMenuGroup::new(
                request.title(),
                dropdown_plan.execute_frame().clone(),
            ));
            dropdown_plans.push(dropdown_plan);
        }

        let projection_digest = digest_groups(&groups);
        let projected_command_count = groups.iter().map(|group| group.commands().len()).sum();
        Ok(Self {
            receipt: WorthUiHeaderFrameReceipt::new(groups, projected_command_count),
            projection_digest,
            dependencies,
            dropdown_plans,
        })
    }

    pub fn execute_frame(&self) -> &WorthUiHeaderFrameReceipt {
        &self.receipt
    }

    pub(crate) fn from_dropdown_plans(
        groups: Vec<WorthUiHeaderMenuGroup>,
        dropdown_plans: Vec<WorthUiDropdownProjectionPlan>,
    ) -> Self {
        let dependencies = dropdown_plans.iter().fold(
            WorthUiProjectionDependencySet::empty(),
            |dependencies, plan| dependencies.merge(plan.dependencies()),
        );
        let projection_digest = digest_groups(&groups);
        let projected_command_count = groups.iter().map(|group| group.commands().len()).sum();
        Self {
            receipt: WorthUiHeaderFrameReceipt::new(groups, projected_command_count),
            projection_digest,
            dependencies,
            dropdown_plans,
        }
    }

    pub fn groups(&self) -> &[WorthUiHeaderMenuGroup] {
        self.receipt.groups()
    }

    pub fn projected_command_count(&self) -> usize {
        self.receipt.projected_command_count()
    }

    pub fn projection_digest(&self) -> u64 {
        self.projection_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    pub(crate) fn dropdown_plans(&self) -> &[WorthUiDropdownProjectionPlan] {
        &self.dropdown_plans
    }
}

impl WorthUiProjectionPlanContract for WorthUiHeaderMenuPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("worth-ui.header.menu")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderMenu
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.projection_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed for WorthUiHeaderMenuPlan {}

fn digest_groups(groups: &[WorthUiHeaderMenuGroup]) -> u64 {
    groups.iter().fold(0xcbf2_9ce4_8422_2325, |digest, group| {
        group.commands().iter().fold(
            fold_bytes(
                fold_bytes(
                    fold_bytes(digest, group.projection_id().as_bytes()),
                    group.selection_mode().token().as_bytes(),
                ),
                group.title().as_bytes(),
            ),
            |command_digest, command| {
                let with_id = fold_bytes(command_digest, command.command_id().as_bytes());
                let with_label = fold_bytes(with_id, command.label().as_bytes());
                match command.shortcut() {
                    Some(shortcut) => fold_bytes(with_label, shortcut.as_bytes()),
                    None => fold_bytes(with_label, b"none"),
                }
            },
        )
    })
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
