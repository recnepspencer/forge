mod bounded_residency_siege;
mod executable_binding;
mod filesystem_profile_protocol;
mod fresh_reopen;
mod hostile_physical_truth;
mod lifecycle_maelstrom;
mod offline_observation;
mod process_execution;
mod report_publication;
mod run_provenance;

use crate::product::CourtroomSelection;
use std::path::Path;

pub(super) struct CourtroomCampaignRequest<'path> {
    pub(super) courtroom: CourtroomSelection,
    pub(super) list: bool,
    pub(super) target_root: Option<&'path Path>,
    pub(super) mutant_report: Option<&'path Path>,
    pub(super) report: Option<&'path Path>,
}

pub(super) fn run(workspace: &Path, request: CourtroomCampaignRequest<'_>) -> Result<(), String> {
    match request.courtroom {
        CourtroomSelection::A if request.list => {
            println!("a\tlifecycle-maelstrom");
            Ok(())
        }
        CourtroomSelection::A => {
            if request.target_root.is_some() {
                return Err("Courtroom A owns its isolated Store root".into());
            }
            let mutant_report = request
                .mutant_report
                .ok_or_else(|| "Courtroom A requires a mutant report".to_owned())?;
            let report = request
                .report
                .ok_or_else(|| "Courtroom A requires an output report".to_owned())?;
            lifecycle_maelstrom::run(workspace, mutant_report, report)
        }
        CourtroomSelection::B if request.list => {
            for scenario in worth_store::physical_runtime::PhysicalWorkHostileTruthScenario::ALL {
                println!("b\t{}", scenario.label());
            }
            Ok(())
        }
        CourtroomSelection::B => {
            let mutant_report = request
                .mutant_report
                .ok_or_else(|| "Courtroom B requires a mutant report".to_owned())?;
            let report = request
                .report
                .ok_or_else(|| "Courtroom B requires an output report".to_owned())?;
            hostile_physical_truth::run(workspace, request.target_root, mutant_report, report)
        }
        CourtroomSelection::C if request.list => {
            println!("c\tbounded-residency-siege");
            Ok(())
        }
        CourtroomSelection::C => {
            let mutant_report = request
                .mutant_report
                .ok_or_else(|| "Courtroom C requires a mutant report".to_owned())?;
            let report = request
                .report
                .ok_or_else(|| "Courtroom C requires an output report".to_owned())?;
            bounded_residency_siege::run(workspace, request.target_root, mutant_report, report)
        }
    }
}
