mod c8_retained_record;
mod catalog;
pub(crate) mod evidence;
mod execution;
mod process_execution;
mod report;
mod source_inventory;
mod source_replacement;
mod target_directory;
mod workspace_snapshot;

use std::path::Path;

use serde::{Deserialize, Serialize};

#[cfg(all(test, feature = "physical-work-evidence"))]
pub(crate) use execution::emit_nested_executable;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum MutationCampaignScope {
    All,
    PhysicalWork,
    BoundedResidency,
    C8Closure,
}

impl MutationCampaignScope {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value {
            "all" => Ok(Self::All),
            "physical-work" => Ok(Self::PhysicalWork),
            "bounded-residency" => Ok(Self::BoundedResidency),
            "c8-closure" => Ok(Self::C8Closure),
            _ => Err(format!(
                "unknown mutation scope `{value}`; expected all|physical-work|bounded-residency|c8-closure"
            )),
        }
    }

    fn mutations(self) -> &'static [catalog::ControlledMutation] {
        match self {
            Self::All => catalog::mutations(),
            Self::PhysicalWork => catalog::physical_work_mutations(),
            Self::BoundedResidency => catalog::bounded_residency_mutations(),
            Self::C8Closure => catalog::c8_closure_mutations(),
        }
    }

    #[cfg(feature = "physical-work-evidence")]
    const fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::PhysicalWork => "physical-work",
            Self::BoundedResidency => "bounded-residency",
            Self::C8Closure => "c8-closure",
        }
    }

    pub(crate) fn contains(self, id: u8) -> bool {
        self.mutations().iter().any(|mutation| mutation.id == id)
    }
}

pub(crate) fn maximum_id() -> u8 {
    catalog::mutations()
        .iter()
        .map(|mutation| mutation.id)
        .max()
        .expect("controlled mutation catalog must not be empty")
}

#[cfg(feature = "physical-work-evidence")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MutationRequirement {
    identity: u16,
    predicate: &'static str,
}

#[cfg(feature = "physical-work-evidence")]
impl MutationRequirement {
    pub(crate) const fn identity(self) -> u16 {
        self.identity
    }

    pub(crate) const fn predicate(self) -> &'static str {
        self.predicate
    }
}

#[cfg(feature = "physical-work-evidence")]
pub(crate) fn bounded_residency_requirements() -> Vec<MutationRequirement> {
    MutationCampaignScope::BoundedResidency
        .mutations()
        .iter()
        .map(|mutation| MutationRequirement {
            identity: mutation.id.into(),
            predicate: mutation.predicate,
        })
        .collect()
}

pub(super) struct MutationCampaignRequest<'path> {
    pub(super) scope: MutationCampaignScope,
    pub(super) list: bool,
    pub(super) preflight: bool,
    pub(super) selected: Option<u8>,
    pub(super) first: Option<u8>,
    pub(super) report: Option<&'path Path>,
}

#[cfg(feature = "physical-work-evidence")]
pub(crate) fn load_physical_work_evidence(
    report: &Path,
    workspace: &Path,
) -> Result<Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>, String> {
    report::load_physical_work_evidence(report, workspace)
}

#[cfg(feature = "physical-work-evidence")]
pub(crate) fn load_bounded_residency_evidence(
    report: &Path,
    workspace: &Path,
) -> Result<Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>, String> {
    report::load_bounded_residency_evidence(report, workspace)
}

#[cfg(feature = "physical-work-evidence")]
#[allow(dead_code)]
pub(crate) fn load_c8_closure_record(
    report: &Path,
    workspace: &Path,
) -> Result<c8_retained_record::RetainedC8CampaignRecord, String> {
    report::load_c8_closure_record(report, workspace)
}

pub(super) fn run(
    workspace_root: &Path,
    request: MutationCampaignRequest<'_>,
) -> Result<(), String> {
    validate_request(&request)?;
    let mutations = request.scope.mutations();
    let selected_mutations = mutations
        .iter()
        .filter(|mutation| request.selected.is_none_or(|id| mutation.id == id))
        .filter(|mutation| request.first.is_none_or(|id| mutation.id >= id))
        .collect::<Vec<_>>();
    if request.list {
        for mutation in selected_mutations {
            println!(
                "{}\t{}\t{}\t{}",
                mutation.id, mutation.predicate, mutation.source, mutation.selector
            );
        }
        return Ok(());
    }
    source_replacement::validate_bindings(workspace_root, &selected_mutations)?;
    println!(
        "mutation source preflight: {} exact bindings",
        selected_mutations.len()
    );
    if request.preflight {
        return Ok(());
    }
    let live_source = source_inventory::bind(workspace_root)?;
    let evidence_session = match request.report {
        Some(path) => Some(report::MutationEvidenceSession::begin(
            path,
            live_source.clone(),
            request.scope,
            workspace_root,
        )?),
        None => None,
    };
    let campaign_target = target_directory::MutationCampaignTarget::allocate(workspace_root)?;
    let campaign_result = run_in_private_snapshot(
        workspace_root,
        &live_source,
        &selected_mutations,
        &campaign_target,
    );
    let target_close = campaign_target.close();
    let observations = finish_campaign(campaign_result, target_close)?;
    if let Some(session) = evidence_session {
        let current_source = source_inventory::bind(workspace_root)?;
        session.publish(&observations, &current_source)?;
        println!(
            "mutation report: {}",
            request
                .report
                .expect("evidence session requires a report path")
                .display()
        );
    }
    Ok(())
}

fn run_in_private_snapshot(
    workspace_root: &Path,
    live_source: &source_inventory::MutationSourceBinding,
    selected_mutations: &[&catalog::ControlledMutation],
    campaign_target: &target_directory::MutationCampaignTarget,
) -> Result<Vec<evidence::MutationObservation>, String> {
    let snapshot_parent = campaign_target
        .path()
        .parent()
        .ok_or_else(|| "mutation campaign target omitted its parent directory".to_owned())?;
    let campaign_source = workspace_snapshot::MutationWorkspaceSnapshot::materialize(
        workspace_root,
        snapshot_parent,
    )?;
    let source_match = (campaign_source.source() == live_source)
        .then_some(())
        .ok_or_else(|| "mutation source snapshot changed the captured source identity".into());
    let mut observations = Vec::new();
    let campaign_result = source_match.and_then(|()| {
        for mutation in selected_mutations {
            println!("mutate: {} ({})", mutation.id, mutation.predicate);
            let observation =
                execution::execute(campaign_source.workspace(), mutation, &campaign_target)?;
            println!("C5_MUTANT_EVIDENCE {}", evidence::encode(&observation)?);
            observations.push(observation);
        }
        Ok(())
    });
    let live_source_result = source_inventory::bind(workspace_root).and_then(|current| {
        (current == *live_source)
            .then_some(())
            .ok_or_else(|| "mutation campaign changed the live workspace source identity".into())
    });
    let snapshot_close = campaign_source.close();
    combine_results([campaign_result, live_source_result, snapshot_close]).map(|()| observations)
}

fn finish_campaign<T>(campaign: Result<T, String>, close: Result<(), String>) -> Result<T, String> {
    match (campaign, close) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
        (Err(campaign), Err(close)) => Err(format!("{campaign}; {close}")),
    }
}

fn combine_results<const N: usize>(results: [Result<(), String>; N]) -> Result<(), String> {
    let errors = results
        .into_iter()
        .filter_map(Result::err)
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

fn validate_request(request: &MutationCampaignRequest<'_>) -> Result<(), String> {
    if request.preflight
        && (request.list
            || request.report.is_some()
            || request.selected.is_some()
            || request.first.is_some())
    {
        return Err(
            "--preflight checks the complete selected scope and rejects listing, reports, and execution selectors"
                .into(),
        );
    }
    if request.list && request.report.is_some() {
        return Err("--report requires an executing mutation campaign".into());
    }
    if request.report.is_some() {
        if matches!(request.scope, MutationCampaignScope::All) {
            return Err("mutation evidence reports require a bounded mutation scope".into());
        }
        if request.selected.is_some() || request.first.is_some() {
            return Err("mutation evidence reports require the complete selected campaign".into());
        }
    }
    validate_selectors(request.scope, request.selected, request.first)
}

fn validate_selectors(
    scope: MutationCampaignScope,
    selected: Option<u8>,
    first: Option<u8>,
) -> Result<(), String> {
    for (name, selector) in [("--mutant", selected), ("--from-mutant", first)] {
        if let Some(id) = selector {
            if !scope.contains(id) {
                return Err(format!(
                    "{name} requires a mutant in the selected mutation scope, got `{id}`"
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
