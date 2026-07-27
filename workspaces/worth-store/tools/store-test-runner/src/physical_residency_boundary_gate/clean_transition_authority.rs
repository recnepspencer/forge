use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

mod fixtures;

use fixtures::{
    honest_candidate_publisher, honest_dirty, honest_frame_ports, honest_ownership,
    honest_writeback, honest_writeback_completion, honest_writeback_execution,
};

const OWNERSHIP: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool_ownership.rs";
const DIRTY: &str = "crates/worth-store-buffer-pool/src/physical_residency/lease/dirty.rs";
const WRITEBACK: &str = "crates/worth-store-buffer-pool/src/physical_residency/lease/writeback.rs";
const FRAME_PORTS: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/frame_ports.rs";
const CANDIDATE_PUBLISHER: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_publishers.rs";
const WRITEBACK_COMPLETION: &str =
    "crates/worth-store/src/physical_runtime/work/execution/outcome/residency_writeback.rs";
const WRITEBACK_EXECUTION: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/dirty/writeback/execution.rs";

struct CleanAuthoritySources<'source> {
    ownership: (&'source Path, &'source str),
    dirty: (&'source Path, &'source str),
    writeback: (&'source Path, &'source str),
    frame_ports: (&'source Path, &'source str),
    candidate_publisher: (&'source Path, &'source str),
    writeback_completion: (&'source Path, &'source str),
    writeback_execution: (&'source Path, &'source str),
}

#[test]
fn pool_cleaning_requires_instance_bound_store_owned_authority() {
    let root = workspace_root();
    let sources = [
        (OWNERSHIP, read(&root.join(OWNERSHIP))),
        (DIRTY, read(&root.join(DIRTY))),
        (WRITEBACK, read(&root.join(WRITEBACK))),
        (FRAME_PORTS, read(&root.join(FRAME_PORTS))),
        (CANDIDATE_PUBLISHER, read(&root.join(CANDIDATE_PUBLISHER))),
        (WRITEBACK_COMPLETION, read(&root.join(WRITEBACK_COMPLETION))),
        (WRITEBACK_EXECUTION, read(&root.join(WRITEBACK_EXECUTION))),
    ];
    let sources = sources.map(|(path, source)| {
        (
            root.join(path),
            source.unwrap_or_else(|denial| panic!("{denial}")),
        )
    });
    inspect_clean_authority(CleanAuthoritySources {
        ownership: (&sources[0].0, &sources[0].1),
        dirty: (&sources[1].0, &sources[1].1),
        writeback: (&sources[2].0, &sources[2].1),
        frame_ports: (&sources[3].0, &sources[3].1),
        candidate_publisher: (&sources[4].0, &sources[4].1),
        writeback_completion: (&sources[5].0, &sources[5].1),
        writeback_execution: (&sources[6].0, &sources[6].1),
    })
    .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn clean_authority_gate_kills_zero_argument_transitions() {
    let dirty = honest_dirty().replace("authority: &CandidateFrameCleanAuthority,", "");
    let denial = inspect_clean_authority(honest_sources(&dirty, honest_writeback()))
        .expect_err("zero-argument candidate cleaner must be denied");
    assert!(denial.contains("candidate clean transition"));

    let writeback = honest_writeback().replace("authority: &FrameWritebackCleanAuthority,", "");
    let denial = inspect_clean_authority(honest_sources(honest_dirty(), &writeback))
        .expect_err("zero-argument writeback cleaner must be denied");
    assert!(denial.contains("writeback clean transition"));
}

#[test]
fn clean_authority_gate_kills_forgeable_or_collapsed_authority() {
    let forgeable =
        honest_ownership().replace("owner: Arc<PoolInner>", "pub owner: Arc<PoolInner>");
    let mut sources = honest_sources(honest_dirty(), honest_writeback());
    sources.ownership = (Path::new("ownership.rs"), &forgeable);
    let denial =
        inspect_clean_authority(sources).expect_err("public authority fields must be denied");
    assert!(denial.contains("private owner"));

    let collapsed = honest_writeback().replace(
        "FrameWritebackCleanAuthority",
        "CandidateFrameCleanAuthority",
    );
    let denial = inspect_clean_authority(honest_sources(honest_dirty(), &collapsed))
        .expect_err("candidate authority must not clean writeback");
    assert!(denial.contains("writeback clean transition"));
}

#[test]
fn clean_authority_gate_kills_unsettled_store_consumption() {
    let candidate =
        honest_candidate_publisher().replace("let _settlement = settlement.settlement();", "");
    let mut sources = honest_sources(honest_dirty(), honest_writeback());
    sources.candidate_publisher = (Path::new("candidate.rs"), &candidate);
    let denial =
        inspect_clean_authority(sources).expect_err("unsettled candidate clean must be denied");
    assert!(denial.contains("candidate settlement"));

    let completion = honest_writeback_completion().replace(
        "if !receipt_matches_claim(&self.claim, &self.receipt) { return Err(()); }",
        "",
    );
    let mut sources = honest_sources(honest_dirty(), honest_writeback());
    sources.writeback_completion = (Path::new("completion.rs"), &completion);
    let denial =
        inspect_clean_authority(sources).expect_err("receipt-free writeback clean must be denied");
    assert!(denial.contains("writeback receipt"));
}

fn inspect_clean_authority(sources: CleanAuthoritySources<'_>) -> Result<(), String> {
    inspect_ownership(sources.ownership)?;
    inspect_candidate_transition(sources.dirty)?;
    inspect_writeback_transition(sources.writeback)?;
    inspect_store_ownership(sources.frame_ports)?;
    inspect_candidate_settlement(sources.candidate_publisher)?;
    inspect_writeback_settlement(sources.writeback_completion)?;
    inspect_writeback_handoff(sources.writeback_execution)
}

fn inspect_ownership(source: (&Path, &str)) -> Result<(), String> {
    for authority in [
        "CandidateFrameCleanAuthority",
        "FrameWritebackCleanAuthority",
    ] {
        let declaration = format!("pub struct {authority}");
        let body = required_body(source, &declaration)?;
        if !body.contains("owner: Arc<PoolInner>") || body.contains("pub owner:") {
            return Err(format!(
                "clean authority: `{authority}` must carry one private owner in {}",
                source.0.display()
            ));
        }
        if source.1.contains(&format!("impl Clone for {authority}"))
            || source.1.contains(&format!("impl Default for {authority}"))
            || preceding_declaration(source.1, &declaration).contains("Clone")
        {
            return Err(format!(
                "clean authority: `{authority}` must not be clonable or defaultable in {}",
                source.0.display()
            ));
        }
    }
    if source.1.contains("pub fn new(") {
        return Err(format!(
            "clean authority: capabilities must expose no reusable constructor in {}",
            source.0.display()
        ));
    }
    let open = compact(required_body(source, "pub fn open")?);
    let parts = compact(required_body(source, "pub fn into_parts")?);
    let all = compact(source.1);
    if !open.contains("PhysicalResidencyPool::open(store,limits)?")
        || open.matches("Arc::clone(&pool.inner)").count() != 2
        || !parts.contains("(self.pool,self.candidate_clean,self.writeback_clean)")
        || all.matches("Arc::ptr_eq(&self.owner,owner)").count() != 2
    {
        return Err(format!(
            "clean authority: owner issuance must bind two distinct capabilities to one pool instance in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_candidate_transition(source: (&Path, &str)) -> Result<(), String> {
    let declaration = required_declaration(source, "pub fn complete_candidate_publication")?;
    let body = compact(required_body(
        source,
        "pub fn complete_candidate_publication",
    )?);
    if !declaration.contains("&CandidateFrameCleanAuthority")
        || declaration.contains("FrameWritebackCleanAuthority")
        || !body.contains("authority.authorizes(&lease.owner)")
        || !body.contains("CandidateCleanAuthorityMismatch")
    {
        return Err(format!(
            "clean authority: candidate clean transition lacks its exact instance-bound capability in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_writeback_transition(source: (&Path, &str)) -> Result<(), String> {
    let declaration = required_declaration(source, "pub fn complete_writeback")?;
    let body = compact(required_body(source, "pub fn complete_writeback")?);
    if !declaration.contains("&FrameWritebackCleanAuthority")
        || declaration.contains("CandidateFrameCleanAuthority")
        || !body.contains("authority.authorizes(&self.owner)")
        || !body.contains("WritebackCleanAuthorityMismatch")
    {
        return Err(format!(
            "clean authority: writeback clean transition lacks its exact instance-bound capability in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_store_ownership(source: (&Path, &str)) -> Result<(), String> {
    let ports = required_body(source, "struct RecordFramePorts")?;
    let bounded = compact(required_body(source, "fn bounded")?);
    if !ports.contains("writeback_clean: Arc<FrameWritebackCleanAuthority>")
        || ports.contains("pub writeback_clean:")
        || !bounded.contains("PhysicalResidencyPoolOwner::open(store,limits)?.into_parts()")
        || !bounded.contains("candidate_clean")
        || !bounded.contains("writeback_clean")
    {
        return Err(format!(
            "clean authority: Store frame ports must privately retain owner-issued capabilities in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_candidate_settlement(source: (&Path, &str)) -> Result<(), String> {
    let resident = required_body(source, "struct BoundedResidentCandidateFrame")?;
    let body = compact(required_body(source, "fn publish_clean")?);
    if !resident.contains("candidate_clean: Arc<CandidateFrameCleanAuthority>")
        || !ordered(
            &body,
            "settlement.settlement()",
            "complete_candidate_publication(&self.candidate_clean)",
        )
    {
        return Err(format!(
            "clean authority: candidate settlement must precede exact capability consumption in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_writeback_settlement(source: (&Path, &str)) -> Result<(), String> {
    let declaration = required_declaration(source, "fn publish_clean")?;
    let body = compact(required_body(source, "fn publish_clean")?);
    if !declaration.contains("&FrameWritebackCleanAuthority")
        || !ordered(
            &body,
            "receipt_matches_claim(&self.claim,&self.receipt)",
            "self.claim.complete_writeback(authority)",
        )
    {
        return Err(format!(
            "clean authority: exact writeback receipt must precede capability consumption in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn inspect_writeback_handoff(source: (&Path, &str)) -> Result<(), String> {
    let body = compact(required_body(source, "fn execute")?);
    if !ordered(
        &body,
        "ifsettled_success",
        "publish_clean(self.frame_ports.writeback_clean_authority())",
    ) {
        return Err(format!(
            "clean authority: settled writeback execution must present Store-owned authority in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn ordered(source: &str, first: &str, second: &str) -> bool {
    source
        .find(first)
        .zip(source.find(second))
        .is_some_and(|(first, second)| first < second)
}

fn compact(source: &str) -> String {
    source.split_whitespace().collect()
}

fn preceding_declaration<'source>(source: &'source str, declaration: &str) -> &'source str {
    let Some(end) = source.find(declaration) else {
        return "";
    };
    &source[end.saturating_sub(96)..end]
}

fn required_declaration<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    let start = source
        .1
        .find(signature)
        .ok_or_else(|| missing(source, signature))?;
    let tail = &source.1[start..];
    let end = tail.find('{').ok_or_else(|| missing(source, signature))?;
    Ok(&tail[..end])
}

fn required_body<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    let start = source
        .1
        .find(signature)
        .ok_or_else(|| missing(source, signature))?;
    let tail = &source.1[start..];
    let body_start = tail.find('{').ok_or_else(|| missing(source, signature))?;
    let mut depth = 0_u32;
    for (offset, byte) in tail[body_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&tail[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(missing(source, signature))
}

fn missing(source: (&Path, &str), signature: &str) -> String {
    format!(
        "clean authority: `{signature}` missing or malformed in {}",
        source.0.display()
    )
}

fn honest_sources<'source>(
    dirty: &'source str,
    writeback: &'source str,
) -> CleanAuthoritySources<'source> {
    CleanAuthoritySources {
        ownership: (Path::new("ownership.rs"), honest_ownership()),
        dirty: (Path::new("dirty.rs"), dirty),
        writeback: (Path::new("writeback.rs"), writeback),
        frame_ports: (Path::new("ports.rs"), honest_frame_ports()),
        candidate_publisher: (Path::new("candidate.rs"), honest_candidate_publisher()),
        writeback_completion: (Path::new("completion.rs"), honest_writeback_completion()),
        writeback_execution: (Path::new("execution.rs"), honest_writeback_execution()),
    }
}
