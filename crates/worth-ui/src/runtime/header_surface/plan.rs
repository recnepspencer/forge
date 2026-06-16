use crate::capability::{CapabilitySnapshot, CommandId};
use crate::runtime::{WorthUiProjectionDependencySet, WorthUiRuntimeFactId};

use super::{
    WorthUiHeaderFrameReceipt, WorthUiHeaderMenuCommand, WorthUiHeaderMenuGroup,
    WorthUiHeaderMenuProjectionRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderMenuPlan {
    receipt: WorthUiHeaderFrameReceipt,
    projection_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderMenuPlanDenial {
    EmptyProjectionSet,
    MissingProjection(String),
    EmptyProjection(String),
    ProjectionReferencesMissingCommand {
        projection_id: String,
        command_id: String,
    },
}

impl WorthUiHeaderMenuPlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        requests: impl IntoIterator<Item = WorthUiHeaderMenuProjectionRequest>,
    ) -> Result<Self, WorthUiHeaderMenuPlanDenial> {
        let requests: Vec<_> = requests.into_iter().collect();
        if requests.is_empty() {
            return Err(WorthUiHeaderMenuPlanDenial::EmptyProjectionSet);
        }

        let mut groups = Vec::with_capacity(requests.len());
        let mut dependencies = WorthUiProjectionDependencySet::empty();
        for request in requests {
            let (group, group_dependencies) = group_from_projection(snapshot, &request)?;
            dependencies = dependencies.merge(&group_dependencies);
            groups.push(group);
        }

        let projection_digest = digest_groups(&groups);
        let projected_command_count = groups.iter().map(|group| group.commands().len()).sum();
        Ok(Self {
            receipt: WorthUiHeaderFrameReceipt::new(groups, projected_command_count),
            projection_digest,
            dependencies,
        })
    }

    pub fn execute_frame(&self) -> &WorthUiHeaderFrameReceipt {
        &self.receipt
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
}

fn group_from_projection(
    snapshot: &CapabilitySnapshot,
    request: &WorthUiHeaderMenuProjectionRequest,
) -> Result<(WorthUiHeaderMenuGroup, WorthUiProjectionDependencySet), WorthUiHeaderMenuPlanDenial> {
    let projection = snapshot
        .command_projections()
        .get(request.projection_id())
        .ok_or_else(|| {
            WorthUiHeaderMenuPlanDenial::MissingProjection(
                request.projection_id().as_str().to_owned(),
            )
        })?;
    let projection_id = request.projection_id().as_str().to_owned();
    let mut dependencies = WorthUiProjectionDependencySet::empty()
        .depends_on(WorthUiRuntimeFactId::command_projection(
            request.projection_id(),
        ))
        .depends_on(WorthUiRuntimeFactId::command_projection_interaction_policy(
            request.projection_id(),
        ));
    let commands = projection
        .command_references()
        .iter()
        .map(|reference| {
            dependencies = dependencies
                .clone()
                .depends_on(WorthUiRuntimeFactId::command(reference.command_id()));
            command_from_snapshot(snapshot, &projection_id, reference.command_id())
        })
        .collect::<Result<Vec<_>, _>>()?;

    if commands.is_empty() {
        return Err(WorthUiHeaderMenuPlanDenial::EmptyProjection(projection_id));
    }

    Ok((
        WorthUiHeaderMenuGroup::new(
            request.title(),
            request.projection_id().as_str(),
            projection.selection_mode(),
            commands,
        ),
        dependencies,
    ))
}

fn command_from_snapshot(
    snapshot: &CapabilitySnapshot,
    projection_id: &str,
    command_id: &CommandId,
) -> Result<WorthUiHeaderMenuCommand, WorthUiHeaderMenuPlanDenial> {
    let command = snapshot.commands().get(command_id).ok_or_else(|| {
        WorthUiHeaderMenuPlanDenial::ProjectionReferencesMissingCommand {
            projection_id: projection_id.to_owned(),
            command_id: command_id.as_str().to_owned(),
        }
    })?;
    Ok(WorthUiHeaderMenuCommand::new(
        command.id().as_str(),
        command.label(),
        command
            .default_shortcut_reference()
            .map(std::borrow::ToOwned::to_owned),
    ))
}

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
