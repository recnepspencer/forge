mod evidence_projection;
mod execution;
mod oracle;
mod protocol;
mod reporting;
mod timing;
mod world;

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
    let mut timings = timing::BoundedResidencySiegeTimings::new();

    let started = Instant::now();
    let mutants = crate::mutation_campaign::load_physical_work_evidence(mutant_report, workspace)?;
    timings.record(
        timing::BoundedResidencySiegePhase::MutationEvidence,
        started.elapsed(),
    );

    let started = Instant::now();
    let world = world::BoundedResidencySiegeWorld::create(target_root)?;
    timings.record(timing::BoundedResidencySiegePhase::World, started.elapsed());

    let binaries = binary_binding::BuiltCourtroomExecutables::build(workspace)?;
    timings.record(
        timing::BoundedResidencySiegePhase::BinaryBuild,
        binaries.cargo_build_elapsed(),
    );
    let binding = binaries.binding_timings();
    timings.record(
        timing::BoundedResidencySiegePhase::SourceInventory,
        binding.source_inventory(),
    );
    timings.record(
        timing::BoundedResidencySiegePhase::PrebuildSourceBinding,
        binding.prebuild_source_binding(),
    );
    timings.record(
        timing::BoundedResidencySiegePhase::PostbuildBinaryBinding,
        binding.postbuild_binary_binding(),
    );
    timings.record(
        timing::BoundedResidencySiegePhase::PostbuildSourceBinding,
        binding.postbuild_source_binding(),
    );
    let observations = execution::observe(&world, &binaries, &mut timings)?;

    let started = Instant::now();
    binaries.verify_source_unchanged()?;
    timings.record(
        timing::BoundedResidencySiegePhase::FinalSourceBinding,
        started.elapsed(),
    );
    let started = Instant::now();
    binaries.verify_executables_unchanged()?;
    timings.record(
        timing::BoundedResidencySiegePhase::ExecutableVerification,
        started.elapsed(),
    );
    let started = Instant::now();
    let rerun = super::run_provenance::rerun("c", target_root, mutant_report, report)?;
    timings.record(
        timing::BoundedResidencySiegePhase::RunProvenance,
        started.elapsed(),
    );
    let started = Instant::now();
    let evidence = oracle::verify(&world, &binaries, observations, mutants, rerun)?;
    timings.record(
        timing::BoundedResidencySiegePhase::OracleVerification,
        started.elapsed(),
    );
    timings.record(
        timing::BoundedResidencySiegePhase::CampaignBeforeReport,
        campaign_started.elapsed(),
    );
    timings.validate_runtime_budget()?;
    reporting::publish(evidence, timings, report_session, &world, campaign_started)
}
