mod catalog;
pub(crate) mod evidence;
mod execution;
mod process_execution;
mod report;
mod sandbox;
mod source_inventory;

use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum MutationCampaignScope {
    All,
    PhysicalWork,
}

impl MutationCampaignScope {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value {
            "all" => Ok(Self::All),
            "physical-work" => Ok(Self::PhysicalWork),
            _ => Err(format!(
                "unknown mutation scope `{value}`; expected `all|physical-work`"
            )),
        }
    }

    fn mutations(self) -> &'static [catalog::ControlledMutation] {
        match self {
            Self::All => catalog::mutations(),
            Self::PhysicalWork => catalog::physical_work_mutations(),
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

pub(super) struct MutationCampaignRequest<'path> {
    pub(super) scope: MutationCampaignScope,
    pub(super) list: bool,
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

pub(super) fn run(
    workspace_root: &Path,
    request: MutationCampaignRequest<'_>,
) -> Result<(), String> {
    validate_request(&request)?;
    let mutations = request.scope.mutations();
    if request.list {
        for mutation in mutations
            .iter()
            .filter(|mutation| request.selected.is_none_or(|id| mutation.id == id))
            .filter(|mutation| request.first.is_none_or(|id| mutation.id >= id))
        {
            println!(
                "{}\t{}\t{}\t{}",
                mutation.id, mutation.predicate, mutation.source, mutation.selector
            );
        }
        return Ok(());
    }
    let mut evidence_session = match request.report {
        Some(path) => Some(report::MutationEvidenceSession::begin(
            path,
            source_inventory::bind(workspace_root)?,
        )?),
        None => None,
    };
    let sandbox = sandbox::MutationSandbox::create(workspace_root)?;
    let mut observations = Vec::new();
    for mutation in mutations
        .iter()
        .filter(|mutation| request.selected.is_none_or(|id| mutation.id == id))
        .filter(|mutation| request.first.is_none_or(|id| mutation.id >= id))
    {
        println!("mutate: {} ({})", mutation.id, mutation.predicate);
        let mut observation = execution::execute(&sandbox, mutation)?;
        if let Some(session) = &mut evidence_session {
            session.retain_binary(&mut observation)?;
        }
        println!("C5_MUTANT_EVIDENCE {}", evidence::encode(&observation)?);
        observations.push(observation);
    }
    if let Some(session) = evidence_session {
        let current_source = source_inventory::bind(workspace_root)?;
        session.publish(&observations, &current_source)?;
        let path = request.report.unwrap();
        println!("mutation report: {}", path.display());
    }
    Ok(())
}

fn validate_request(request: &MutationCampaignRequest<'_>) -> Result<(), String> {
    if request.list && request.report.is_some() {
        return Err("--report requires an executing mutation campaign".into());
    }
    if request.report.is_some() {
        if request.scope != MutationCampaignScope::PhysicalWork {
            return Err(
                "mutation evidence reports require `--mutation-scope physical-work`".into(),
            );
        }
        if request.selected.is_some() || request.first.is_some() {
            return Err(
                "mutation evidence reports require the complete physical-work campaign".into(),
            );
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
mod tests {
    use super::{
        maximum_id, validate_request, validate_selectors, MutationCampaignRequest,
        MutationCampaignScope,
    };

    #[test]
    fn direct_campaign_selection_rejects_an_absent_catalog_id() {
        let absent = maximum_id().checked_add(1).unwrap();

        assert!(validate_selectors(MutationCampaignScope::All, Some(absent), None).is_err());
        assert!(validate_selectors(MutationCampaignScope::All, None, Some(absent)).is_err());
        assert!(validate_selectors(MutationCampaignScope::All, Some(14), None).is_err());
        assert!(validate_selectors(MutationCampaignScope::All, None, Some(14)).is_err());
    }

    #[test]
    fn physical_work_scope_is_the_complete_phase_16_catalog() {
        let ids = MutationCampaignScope::PhysicalWork
            .mutations()
            .iter()
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>();

        assert_eq!(ids, (15..=44).collect::<Vec<_>>());
        assert_eq!(ids.len(), 30);
        assert!(!MutationCampaignScope::PhysicalWork.contains(14));
    }

    #[test]
    fn report_publication_requires_the_complete_physical_work_scope() {
        let report = std::path::Path::new("phase16.json");
        let all = MutationCampaignRequest {
            scope: MutationCampaignScope::All,
            list: false,
            selected: None,
            first: None,
            report: Some(report),
        };
        assert!(validate_request(&all).is_err());

        let partial = MutationCampaignRequest {
            scope: MutationCampaignScope::PhysicalWork,
            selected: Some(15),
            ..all
        };
        assert!(validate_request(&partial).is_err());

        let complete = MutationCampaignRequest {
            scope: MutationCampaignScope::PhysicalWork,
            selected: None,
            ..all
        };
        assert!(validate_request(&complete).is_ok());
    }
}
