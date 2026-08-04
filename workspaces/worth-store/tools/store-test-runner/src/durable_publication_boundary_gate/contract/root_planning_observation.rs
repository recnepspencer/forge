use super::super::read_repository_document;

const OBSERVATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           record_serving/publication/append_observation.rs";
const CANDIDATE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         record_serving/publication/root_candidate/candidate.rs";
const CURRENT_ROOT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                            durability/publication/current_root_owner.rs";

#[test]
fn completed_root_carries_shared_root_planning_observation_without_member_attribution() {
    inspect(&sources()).unwrap();
}

#[test]
fn root_planning_observation_contract_rejects_dropped_or_member_laundered_evidence() {
    let source = sources();

    let mut dropped = source.clone();
    dropped.candidate = dropped
        .candidate
        .replace("self.plan.observation", "PublicationObservation::default()");
    assert!(inspect(&dropped).is_err());

    let mut hidden = source;
    hidden.current_root = hidden.current_root.replace(
        "pub const fn root_planning_observation(",
        "fn hidden_observation(",
    );
    assert!(inspect(&hidden).is_err());
}

#[derive(Clone)]
struct Sources {
    observation: String,
    candidate: String,
    current_root: String,
}

fn sources() -> Sources {
    Sources {
        observation: read_repository_document(OBSERVATION).expect("read planning observation"),
        candidate: read_repository_document(CANDIDATE).expect("read root candidate"),
        current_root: read_repository_document(CURRENT_ROOT).expect("read current-root owner"),
    }
}

fn inspect(source: &Sources) -> Result<(), &'static str> {
    for required in [
        "pub struct RecordRootPlanningObservation",
        "manifest_blocks_read: u64",
        "manifest_comparisons: u64",
        "manifest_bytes_read: u64",
    ] {
        if !source.observation.contains(required) {
            return Err("root planning observation lost exact manifest discovery evidence");
        }
    }
    for required in [
        "RecordRootPlanningObservation::from_publication(",
        "self.plan.observation",
    ] {
        if !source.candidate.contains(required) {
            return Err("root candidate dropped shared planning evidence");
        }
    }
    for required in [
        "root_planning_observation: crate::physical_runtime::RecordRootPlanningObservation",
        "candidate.into_root_parts()",
        "pub const fn root_planning_observation(",
        "self.root_planning_observation",
    ] {
        if !source.current_root.contains(required) {
            return Err("completed current root does not expose exact shared planning evidence");
        }
    }
    if source
        .current_root
        .contains("RootPublicationPhysicalMutationMember::new(")
        && source.current_root.contains("root_planning_observation")
    {
        return Err("shared root planning evidence was laundered into one settled member");
    }
    Ok(())
}
