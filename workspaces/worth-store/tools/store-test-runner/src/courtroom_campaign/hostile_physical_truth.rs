mod artifact_oracle;
mod evidence_projection;
mod oracle;
#[cfg(test)]
mod oracle_tests;
mod scenario;
mod timing;
mod world;
mod writer_protocol;

use std::path::Path;
use std::time::Instant;

use super::executable_binding as binary_binding;
use super::fresh_reopen as reopen_protocol;
use super::offline_observation as offline_protocol;
use super::process_execution;
use super::report_publication;

pub(super) fn run(
    workspace: &Path,
    target_root: Option<&Path>,
    mutant_report: &Path,
    report: &Path,
) -> Result<(), String> {
    let campaign_started = Instant::now();
    let report_session = report_publication::CourtroomReportSession::begin(report)?;
    let mut timings = timing::CampaignTimings::new();
    let started = Instant::now();
    let mutants = crate::mutation_campaign::load_physical_work_evidence(mutant_report, workspace)?;
    timings.record_campaign(timing::CampaignPhase::MutationEvidence, started.elapsed());

    let started = Instant::now();
    let world = world::CampaignWorld::create(target_root)?;
    timings.record_campaign(timing::CampaignPhase::World, started.elapsed());

    let binaries = binary_binding::BuiltCourtroomExecutables::build(workspace)?;
    timings.record_campaign(
        timing::CampaignPhase::BinaryBuild,
        binaries.cargo_build_elapsed(),
    );
    let binding = binaries.binding_timings();
    timings.record_campaign(
        timing::CampaignPhase::SourceInventory,
        binding.source_inventory(),
    );
    timings.record_campaign(
        timing::CampaignPhase::PrebuildSourceBinding,
        binding.prebuild_source_binding(),
    );
    timings.record_campaign(
        timing::CampaignPhase::PostbuildBinaryBinding,
        binding.postbuild_binary_binding(),
    );
    timings.record_campaign(
        timing::CampaignPhase::PostbuildSourceBinding,
        binding.postbuild_source_binding(),
    );
    let started = Instant::now();
    let rerun = super::run_provenance::rerun(super::run_provenance::CourtroomRerunRequest {
        courtroom: "b",
        target_root,
        controlled_case_report: mutant_report,
        report,
        schedule_seed: None,
        termination_point: None,
    })?;
    timings.record_campaign(timing::CampaignPhase::RunProvenance, started.elapsed());
    let cases = scenario::run_all(&world, &binaries, &rerun, &mut timings)?;
    let started = Instant::now();
    binaries.verify_source_unchanged()?;
    timings.record_campaign(timing::CampaignPhase::FinalSourceBinding, started.elapsed());
    let started = Instant::now();
    binaries.verify_executables_unchanged()?;
    timings.record_campaign(
        timing::CampaignPhase::ExecutableVerification,
        started.elapsed(),
    );
    let started = Instant::now();
    let evidence = worth_store::physical_runtime::PhysicalWorkHostileTruthCampaignEvidence::new(
        cases, mutants,
    );
    if !evidence.verdict().accepted() {
        return Err(format!(
            "Courtroom B evidence rejected: {:?}",
            evidence.verdict().findings()
        ));
    }
    timings.record_campaign(
        timing::CampaignPhase::CampaignVerification,
        started.elapsed(),
    );
    timings.record_campaign(
        timing::CampaignPhase::CampaignBeforeReport,
        campaign_started.elapsed(),
    );
    timings.validate_runtime_budget()?;
    let started = Instant::now();
    let first_encoding =
        evidence_projection::encode(&evidence, &timings, binaries.runner().binding())?;
    timings.record_campaign(timing::CampaignPhase::ReportEncoding, started.elapsed());
    timings.validate_complete_budget()?;
    drop(first_encoding);
    let encoded = evidence_projection::encode(&evidence, &timings, binaries.runner().binding())?;
    let publication = report_session.publish(&encoded)?;
    let completed_wall = campaign_started.elapsed();
    let runner_controlled_ms = timings.validate_completed_campaign(completed_wall)?;
    let publication_elapsed = publication.elapsed();
    publication.accept();
    println!(
        "courtroom:b accepted {} cases and {} mutants with {}-byte payloads, runner {}, writer {}, observer {}, root {}, in {:.3}s ({}ms runner-controlled); report published in {:.3}s",
        evidence.cases().len(),
        evidence.mutants().len(),
        world.payload_bytes(),
        binaries.runner().path().display(),
        binaries.writer().path().display(),
        binaries.observer().path().display(),
        world.root().display(),
        completed_wall.as_secs_f64(),
        runner_controlled_ms,
        publication_elapsed.as_secs_f64(),
    );
    Ok(())
}
