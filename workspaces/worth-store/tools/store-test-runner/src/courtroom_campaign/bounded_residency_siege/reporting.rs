use std::time::Instant;

use super::{
    evidence_projection, oracle::BoundedResidencyCourtroomEvidence,
    report_publication::CourtroomReportSession, timing::BoundedResidencySiegeTimings,
    world::BoundedResidencySiegeWorld,
};

pub(super) fn publish(
    evidence: BoundedResidencyCourtroomEvidence,
    mut timings: BoundedResidencySiegeTimings,
    report_session: CourtroomReportSession,
    world: &BoundedResidencySiegeWorld,
    campaign_started: Instant,
) -> Result<(), String> {
    let started = Instant::now();
    let first_encoding = evidence_projection::encode(&evidence, &timings)?;
    timings.record(
        super::timing::BoundedResidencySiegePhase::ReportEncoding,
        started.elapsed(),
    );
    timings.validate_complete_budget()?;
    drop(first_encoding);
    let encoded = evidence_projection::encode(&evidence, &timings)?;
    let publication = report_session.publish(&encoded)?;
    let completed_wall = campaign_started.elapsed();
    let runner_controlled_ms = timings.validate_completed_campaign(completed_wall)?;
    let publication_elapsed = publication.elapsed();
    publication.accept();
    println!(
        "courtroom:c accepted one C.6 inheritance siege and {} mutants, root {}, in {:.3}s \
         ({}ms runner-controlled); report published in {:.3}s",
        evidence.mutants().len(),
        world.root().display(),
        completed_wall.as_secs_f64(),
        runner_controlled_ms,
        publication_elapsed.as_secs_f64(),
    );
    Ok(())
}
