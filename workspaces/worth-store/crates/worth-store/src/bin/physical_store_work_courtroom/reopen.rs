use std::io::Write;

use worth_store::physical_runtime::{
    PhysicalRecordOpen, RecordCountLimit, RecordScanDenial, RecordScanOutcome, RecordScanRequest,
    ServingPhysicalRuntime,
};

use super::arguments::ReopenInvocation;

pub(super) fn run(invocation: ReopenInvocation) -> Result<(), String> {
    super::configuration::validate_supported(&invocation.configuration)?;
    let (format, _, access) = super::configuration::record_configuration();
    let media = super::admission::admit_media(&invocation.root, None)?;
    let serving = super::admission::require_serving(
        media.open_record_store(PhysicalRecordOpen::new(format, access)),
        "record-store reopen",
    )?;
    let residue = serving.observed_non_authoritative_residue();
    let recovery_damaged = serving.physical_recovery_evidence_damaged();
    let recovery_count = serving.physical_recovery_obligations().len();
    let inspection_required = residue || recovery_damaged || recovery_count != 0;
    let records = if inspection_required {
        let scan = serving.records().scan(
            RecordScanRequest::from_start()
                .with_batch_limit(RecordCountLimit::new(17).expect("nonzero batch limit")),
        );
        if !matches!(
            scan,
            Err(error) if error.denial() == RecordScanDenial::ServingRequiresInspection
        ) {
            return Err("inspection posture did not fence record scan".to_owned());
        }
        0
    } else {
        count_records(&serving)?
    };
    let generation = serving
        .observer()
        .acquisition_snapshot()
        .map_err(|failure| format!("reopen observation failed: {failure:?}"))?
        .root_generation();
    println!(
        "C5_1_COURTROOM_REOPEN {} {} {} {generation} {records} {residue} \
         {recovery_damaged} {recovery_count} {inspection_required}",
        std::process::id(),
        hex(&serving.store_identity().bytes()),
        serving.runtime_identity().get(),
    );
    std::io::stdout()
        .flush()
        .map_err(|error| format!("reopen marker failed: {error}"))?;
    serving.close();
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn count_records(serving: &ServingPhysicalRuntime) -> Result<usize, String> {
    let mut scan = serving
        .records()
        .scan(RecordScanRequest::from_start())
        .map_err(|failure| format!("reopen scan denied: {failure:?}"))?;
    let mut scratch = vec![0_u8; 64 * 1024];
    let mut count = 0;
    loop {
        match scan
            .read_next_into(&mut scratch)
            .map_err(|failure| format!("reopen scan failed: {failure:?}"))?
        {
            RecordScanOutcome::Batch(batch) => count += batch.records().len(),
            RecordScanOutcome::Completed(_) => return Ok(count),
        }
    }
}
