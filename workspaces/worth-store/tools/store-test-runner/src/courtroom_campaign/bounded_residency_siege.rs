mod c7_crash_campaign;
mod evidence_projection;
mod execution;
#[cfg(test)]
mod mutation_proofs;
mod oracle;
mod protocol;
mod reporting;
mod schedule;
mod timing;
mod world;

use std::path::Path;
use std::time::Instant;

use crate::arguments::CiScheduleLane;

use super::executable_binding as binary_binding;
use super::fresh_reopen as reopen_protocol;
use super::offline_observation as offline_protocol;
use super::process_execution;
use super::report_publication;

#[derive(Clone, Copy)]
pub(super) struct BoundedResidencySiegeRequest<'path> {
    pub(super) target_root: Option<&'path Path>,
    pub(super) controlled_case_report: &'path Path,
    pub(super) report: &'path Path,
    pub(super) schedule_seed: Option<u64>,
    pub(super) ci_schedule_lane: Option<CiScheduleLane>,
    pub(super) termination_point: Option<&'path str>,
}

struct BoundedResidencyScheduleSelection {
    source: worth_store::physical_runtime::PhysicalWorkSourceBinding,
    source_schedule: schedule::SourceClosureScheduleSeeds,
    schedule: schedule::SchedulePerturbationPlan,
    termination_points: Vec<schedule::C7DurabilityCrashSeam>,
}

struct PreparedBoundedResidencySiege<'path> {
    request: BoundedResidencySiegeRequest<'path>,
    campaign_started: Instant,
    report_session: report_publication::CourtroomReportSession,
    source_schedule: schedule::SourceClosureScheduleSeeds,
    schedule: schedule::SchedulePerturbationPlan,
    termination_points: Vec<schedule::C7DurabilityCrashSeam>,
    timings: timing::BoundedResidencySiegeTimings,
    controlled_cases: Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>,
    world: world::BoundedResidencySiegeWorld,
    binaries: binary_binding::BuiltCourtroomExecutables,
    rerun: worth_store::physical_runtime::PhysicalWorkRerunEvidence,
}

pub(super) fn run(
    workspace: &Path,
    request: BoundedResidencySiegeRequest<'_>,
) -> Result<(), String> {
    PreparedBoundedResidencySiege::prepare(workspace, request)?.execute()
}

impl<'path> PreparedBoundedResidencySiege<'path> {
    fn prepare(
        workspace: &Path,
        request: BoundedResidencySiegeRequest<'path>,
    ) -> Result<Self, String> {
        let selection = BoundedResidencyScheduleSelection::derive(workspace, request)?;
        let campaign_started = Instant::now();
        let report_session = report_publication::CourtroomReportSession::begin(request.report)?;
        let mut timings = timing::BoundedResidencySiegeTimings::new();
        let controlled_cases = load_controlled_cases(workspace, request, &mut timings)?;
        let world = create_world(request.target_root, &mut timings)?;
        let binaries = build_binaries(workspace, &selection.source, &mut timings)?;
        let rerun = bind_rerun(request, &selection, &mut timings)?;
        print_replay(&selection, &rerun);
        Ok(Self {
            request,
            campaign_started,
            report_session,
            source_schedule: selection.source_schedule,
            schedule: selection.schedule,
            termination_points: selection.termination_points,
            timings,
            controlled_cases,
            world,
            binaries,
            rerun,
        })
    }

    fn execute(mut self) -> Result<(), String> {
        let observations = execution::observe(
            &self.world,
            &self.binaries,
            &self.schedule,
            &mut self.timings,
        )?;
        let started = Instant::now();
        let termination_campaign =
            c7_crash_campaign::execute(c7_crash_campaign::C7CrashCampaignRequest {
                target_root: self.request.target_root,
                mutant_report: self.request.controlled_case_report,
                report: self.request.report,
                binaries: &self.binaries,
                seams: &self.termination_points,
                schedule_seed: self.schedule.seed().value(),
                checkpoint_order: self.schedule.durability_checkpoint_order(),
            })?;
        self.timings
            .record_case_campaign(started.elapsed(), self.termination_points.len());
        verify_final_bindings(&self.binaries, &mut self.timings)?;
        let started = Instant::now();
        let evidence = oracle::verify(oracle::BoundedResidencyCourtroomProofRequest {
            world: &self.world,
            binaries: &self.binaries,
            observations,
            controlled_cases: self.controlled_cases,
            rerun: self.rerun,
            schedule: self.schedule,
            source_schedule: self.source_schedule,
            termination_campaign,
        })?;
        self.timings.record(
            timing::BoundedResidencySiegePhase::OracleVerification,
            started.elapsed(),
        );
        self.timings.record(
            timing::BoundedResidencySiegePhase::CampaignBeforeReport,
            self.campaign_started.elapsed(),
        );
        self.timings.validate_runtime_budget()?;
        reporting::publish(
            evidence,
            self.timings,
            self.report_session,
            &self.world,
            self.campaign_started,
        )
    }
}

impl BoundedResidencyScheduleSelection {
    fn derive(workspace: &Path, request: BoundedResidencySiegeRequest<'_>) -> Result<Self, String> {
        let source = binary_binding::bind_source_closure(workspace)?;
        let source_schedule = schedule::SourceClosureScheduleSeeds::derive(source.digest().bytes())
            .map_err(|denial| format!("source-closure schedule derivation denied: {denial:?}"))?;
        let schedule = match (request.schedule_seed, request.ci_schedule_lane) {
            (Some(seed), None) => schedule::SchedulePerturbationPlan::derive(
                schedule::SchedulePerturbationSeed::from_u64(seed),
            ),
            (None, Some(lane)) => schedule::SchedulePerturbationPlan::derive(
                source_schedule
                    .seed(lane.index())
                    .expect("CI lane admission enforces the canonical 16-lane range"),
            ),
            (None, None) => schedule::SchedulePerturbationPlan::canonical(),
            (Some(_), Some(_)) => {
                return Err("schedule seed and CI schedule lane are mutually exclusive".into())
            }
        };
        let termination_points = selected_crash_seams(
            &source_schedule,
            request.ci_schedule_lane,
            request.termination_point,
        )?;
        Ok(Self {
            source,
            source_schedule,
            schedule,
            termination_points,
        })
    }
}

fn load_controlled_cases(
    workspace: &Path,
    request: BoundedResidencySiegeRequest<'_>,
    timings: &mut timing::BoundedResidencySiegeTimings,
) -> Result<Vec<worth_store::physical_runtime::PhysicalWorkMutantLocalization>, String> {
    let started = Instant::now();
    let cases = crate::mutation_campaign::load_bounded_residency_evidence(
        request.controlled_case_report,
        workspace,
    )?;
    timings.record(
        timing::BoundedResidencySiegePhase::MutationEvidence,
        started.elapsed(),
    );
    Ok(cases)
}

fn create_world(
    target_root: Option<&Path>,
    timings: &mut timing::BoundedResidencySiegeTimings,
) -> Result<world::BoundedResidencySiegeWorld, String> {
    let started = Instant::now();
    let world = world::BoundedResidencySiegeWorld::create(target_root)?;
    timings.record(timing::BoundedResidencySiegePhase::World, started.elapsed());
    Ok(world)
}

fn build_binaries(
    workspace: &Path,
    source: &worth_store::physical_runtime::PhysicalWorkSourceBinding,
    timings: &mut timing::BoundedResidencySiegeTimings,
) -> Result<binary_binding::BuiltCourtroomExecutables, String> {
    let binaries = binary_binding::BuiltCourtroomExecutables::build(workspace)?;
    binaries.require_source_binding(source)?;
    timings.record(
        timing::BoundedResidencySiegePhase::BinaryBuild,
        binaries.cargo_build_elapsed(),
    );
    let binding = binaries.binding_timings();
    timings.record(
        timing::BoundedResidencySiegePhase::SourceInventory,
        binding.source_inventory(),
    );
    timings.record_source_binding(
        timing::BoundedResidencySiegePhase::PrebuildSourceBinding,
        binding.prebuild_source_binding(),
        binaries.source_workload(),
    )?;
    timings.record(
        timing::BoundedResidencySiegePhase::PostbuildBinaryBinding,
        binding.postbuild_binary_binding(),
    );
    timings.record_source_binding(
        timing::BoundedResidencySiegePhase::PostbuildSourceBinding,
        binding.postbuild_source_binding(),
        binaries.source_workload(),
    )?;
    Ok(binaries)
}

fn bind_rerun(
    request: BoundedResidencySiegeRequest<'_>,
    selection: &BoundedResidencyScheduleSelection,
    timings: &mut timing::BoundedResidencySiegeTimings,
) -> Result<worth_store::physical_runtime::PhysicalWorkRerunEvidence, String> {
    let started = Instant::now();
    let rerun = super::run_provenance::rerun(super::run_provenance::CourtroomRerunRequest {
        courtroom: "c",
        target_root: request.target_root,
        controlled_case_report: request.controlled_case_report,
        report: request.report,
        schedule_seed: Some(selection.schedule.seed().value()),
        termination_point: (selection.termination_points.len() == 1)
            .then(|| selection.termination_points[0].label()),
    })?;
    timings.record(
        timing::BoundedResidencySiegePhase::RunProvenance,
        started.elapsed(),
    );
    Ok(rerun)
}

fn print_replay(
    selection: &BoundedResidencyScheduleSelection,
    rerun: &worth_store::physical_runtime::PhysicalWorkRerunEvidence,
) {
    println!(
        "courtroom:c schedule-seed {} crash-seams {} replay {}",
        selection.schedule.seed().value(),
        selection
            .termination_points
            .iter()
            .map(|seam| seam.label())
            .collect::<Vec<_>>()
            .join(","),
        super::run_provenance::display(rerun),
    );
}

fn verify_final_bindings(
    binaries: &binary_binding::BuiltCourtroomExecutables,
    timings: &mut timing::BoundedResidencySiegeTimings,
) -> Result<(), String> {
    let started = Instant::now();
    let final_source_workload = binaries.verify_source_unchanged()?;
    timings.record_source_binding(
        timing::BoundedResidencySiegePhase::FinalSourceBinding,
        started.elapsed(),
        final_source_workload,
    )?;
    let started = Instant::now();
    binaries.verify_executables_unchanged()?;
    timings.record(
        timing::BoundedResidencySiegePhase::ExecutableVerification,
        started.elapsed(),
    );
    Ok(())
}

fn selected_crash_seams(
    source_schedule: &schedule::SourceClosureScheduleSeeds,
    ci_schedule_lane: Option<CiScheduleLane>,
    explicit: Option<&str>,
) -> Result<Vec<schedule::C7DurabilityCrashSeam>, String> {
    if let Some(lane) = ci_schedule_lane {
        return Ok(vec![source_schedule
            .crash_seam(lane.index())
            .expect("CI lane admission enforces the canonical 16-lane range")]);
    }
    if let Some(label) = explicit {
        return schedule::C7DurabilityCrashSeam::parse(label)
            .map(|seam| vec![seam])
            .ok_or_else(|| format!("unknown C7 crash seam `{label}`"));
    }
    Ok(schedule::C7DurabilityCrashSeam::ALL.to_vec())
}
