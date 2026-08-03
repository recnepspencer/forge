mod catalog;
pub(crate) mod evidence;
mod execution;
mod process_execution;
mod report;
mod source_inventory;
mod source_replacement;

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
}

impl MutationCampaignScope {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value {
            "all" => Ok(Self::All),
            "physical-work" => Ok(Self::PhysicalWork),
            "bounded-residency" => Ok(Self::BoundedResidency),
            _ => Err(format!(
                "unknown mutation scope `{value}`; expected \
                 `all|physical-work|bounded-residency`"
            )),
        }
    }

    fn mutations(self) -> &'static [catalog::ControlledMutation] {
        match self {
            Self::All => catalog::mutations(),
            Self::PhysicalWork => catalog::physical_work_mutations(),
            Self::BoundedResidency => catalog::bounded_residency_mutations(),
        }
    }

    #[cfg(feature = "physical-work-evidence")]
    const fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::PhysicalWork => "physical-work",
            Self::BoundedResidency => "bounded-residency",
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
    let evidence_session = match request.report {
        Some(path) => Some(report::MutationEvidenceSession::begin(
            path,
            source_inventory::bind(workspace_root)?,
            request.scope,
        )?),
        None => None,
    };
    let mut observations = Vec::new();
    for mutation in selected_mutations {
        println!("mutate: {} ({})", mutation.id, mutation.predicate);
        let observation = execution::execute(workspace_root, mutation)?;
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
        if request.scope == MutationCampaignScope::All {
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
    fn bounded_residency_scope_contains_inherited_c6_and_complete_c7_corpus() {
        let ids = MutationCampaignScope::BoundedResidency
            .mutations()
            .iter()
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>();

        let expected = super::catalog::physical_work_mutations()
            .iter()
            .filter(|mutation| matches!(mutation.id, 42..=44))
            .chain(super::catalog::physical_reconstruction_c6_mutations())
            .chain(super::catalog::physical_reconstruction_c7_mutations())
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>();
        assert_eq!(ids, expected);
        assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(ids.last(), Some(&127));
    }

    #[test]
    fn c7_regression_corpus_is_append_only_and_contiguous() {
        let ids = super::catalog::physical_reconstruction_c7_mutations()
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>();

        assert_eq!(ids, (79..=127).collect::<Vec<_>>());
        assert!(MutationCampaignScope::All.contains(79));
        assert!(MutationCampaignScope::All.contains(118));
        assert!(MutationCampaignScope::All.contains(119));
        assert!(MutationCampaignScope::All.contains(120));
        assert!(MutationCampaignScope::All.contains(121));
        assert!(MutationCampaignScope::All.contains(122));
        assert!(MutationCampaignScope::All.contains(123));
        assert!(MutationCampaignScope::All.contains(124));
        assert!(MutationCampaignScope::All.contains(125));
        assert!(MutationCampaignScope::All.contains(126));
        assert!(MutationCampaignScope::All.contains(127));
    }

    #[test]
    fn ci_certification_preserves_the_required_six_mutation_categories() {
        let predicates = MutationCampaignScope::BoundedResidency
            .mutations()
            .iter()
            .map(|mutation| mutation.predicate)
            .collect::<std::collections::BTreeSet<_>>();
        for required in [
            "whole-store-allocation",
            "pinned-eviction",
            "writeback-clean-without-exact-receipt",
            "duplicate-source-load",
            "speculative-kind-budget-bypass",
            "physical-work-topology-bypass",
        ] {
            assert!(
                predicates.contains(required),
                "CI mutation floor omitted `{required}`"
            );
        }
    }

    #[test]
    fn report_publication_requires_a_complete_bounded_scope() {
        let report = std::path::Path::new("phase16.json");
        let all = MutationCampaignRequest {
            scope: MutationCampaignScope::All,
            list: false,
            preflight: false,
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
        assert!(validate_request(&MutationCampaignRequest {
            scope: MutationCampaignScope::BoundedResidency,
            ..complete
        })
        .is_ok());
    }

    #[test]
    fn preflight_is_a_complete_non_executing_scope_mode() {
        let complete = MutationCampaignRequest {
            scope: MutationCampaignScope::BoundedResidency,
            list: false,
            preflight: true,
            selected: None,
            first: None,
            report: None,
        };
        assert!(validate_request(&complete).is_ok());

        for invalid in [
            MutationCampaignRequest {
                list: true,
                ..complete
            },
            MutationCampaignRequest {
                selected: Some(42),
                ..complete
            },
            MutationCampaignRequest {
                report: Some(std::path::Path::new("report.json")),
                ..complete
            },
        ] {
            assert!(validate_request(&invalid).is_err());
        }
    }
}
