use super::super::*;
use super::mutation_ownership_process::{
    encode_hex, ensure_stable_identity, parse_owned, spawn_contender, ProcessContender,
    PROCESS_ROLE_ENV, PROCESS_ROOT_ENV,
};

#[test]
fn absent_root_scaffold_converges_before_exactly_one_live_lease() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("first owner");
    let contender =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect_err("second owner must contend");

    assert_eq!(
        contender,
        FilesystemMediaOwnerAdmissionDenial::Ownership(MutationOwnershipDenial::Contended)
    );
    assert_eq!(immediate_names(&root), ["families", "namespace", "staging"]);
    assert_eq!(immediate_names(&root.join("namespace")), ["mutation.lock"]);
    assert_ne!(owner.families().identity(), owner.staging().identity());
    assert!(owner.begin_mutation().is_ok());
    let owner_observation = owner.mutation_owner();
    drop(owner);
    let observation = std::fs::read_to_string(root.join("namespace/mutation.lock"))
        .expect("read diagnostic owner observation");
    let process = observation
        .lines()
        .find_map(|line| line.strip_prefix("process="))
        .expect("process observation");
    assert_eq!(process.len(), 10);
    assert_eq!(process.parse::<u32>().unwrap(), std::process::id());
    assert!(observation.contains(&format!(
        "runtime={}",
        encode_hex(&owner_observation.owner().bytes())
    )));
    assert!(observation.contains(&format!(
        "attempt={}",
        encode_hex(&owner_observation.attempt().bytes())
    )));
}

#[test]
fn dropping_owner_releases_os_lease_without_deleting_lock_target() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let first = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("first owner");
    let first_observation = first.mutation_owner();
    let first_counters = first.counter_observer();
    drop(first);
    assert_eq!(first_counters.snapshot().ownership_releases(), 1);

    assert!(root.join("namespace/mutation.lock").is_file());
    let successor =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect("successor owner");
    assert_ne!(first_observation.owner(), successor.identity());
    assert_ne!(
        first_observation.attempt(),
        successor.mutation_owner().attempt()
    );
}

#[test]
fn explicit_close_reports_release_and_allows_successor() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("owner");
    let counters = owner.counter_observer();
    let live = counters.snapshot();
    assert_eq!(live.directory_opens(), 4);
    assert_eq!(live.directory_closes(), 0);
    assert_eq!(live.live_directory_handles(), 4);
    assert_eq!(live.peak_directory_handles(), 4);

    assert_eq!(owner.close(), OwnershipReleaseOutcome::Released);
    let closed = counters.snapshot();
    assert_eq!(closed.ownership_releases(), 1);
    assert_eq!(closed.directory_opens(), 4);
    assert_eq!(closed.directory_closes(), 4);
    assert_eq!(closed.live_directory_handles(), 0);
    assert_eq!(closed.peak_directory_handles(), 4);
    let successor =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect("successor after explicit release");
    drop(successor);
}

#[test]
fn invalidated_live_lease_makes_existing_mutation_handles_inert() {
    use worth_store_physical_format::store_namespace::{
        NamespaceInitializationAttempt, StagedNamespaceName,
    };

    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("owner");
    let attempt =
        NamespaceInitializationAttempt::from_nonzero_bytes([31; 16]).expect("nonzero attempt");
    let path = owner.staged_identity_path(&StagedNamespaceName::for_identity(attempt));
    let handle = match owner.create_new(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => handle,
        other => panic!("create mutation fixture: {other:?}"),
    };

    owner.invalidate_mutation_authority();
    assert_eq!(
        handle
            .positioned_write(PositionedWriteRequest::new(0, b"must-not-land"))
            .effect_status(),
        MediaEffectStatus::DeniedBeforeEffect
    );
    assert!(matches!(
        handle.synchronize_state(),
        FileStateSynchronizationOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
    assert!(matches!(
        owner.synchronize_directory_publication(owner.staging().handle()),
        DirectoryPublicationSynchronizationOutcome::Failed(failure)
            if failure.effect_status() == MediaEffectStatus::DeniedBeforeEffect
    ));
    assert_eq!(
        std::fs::metadata(root.join(path.as_path()))
            .expect("fixture metadata")
            .len(),
        0
    );
}

#[test]
fn invalidation_preserves_an_admitted_fact_and_precedes_every_later_one() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("owner");
    let _admitted = owner.begin_mutation().expect("initial mutation authority");
    owner.invalidate_mutation_authority();

    assert!(matches!(
        owner.begin_mutation(),
        Err(FilesystemMediaOwnerAdmissionDenial::Ownership(
            MutationOwnershipDenial::OwnershipLost
        ))
    ));
}

#[test]
fn stale_lock_metadata_neither_grants_nor_blocks_authority() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("owner");
    drop(owner);
    let mut valid_stale_observation = std::fs::read(root.join("namespace/mutation.lock"))
        .expect("capture structurally valid stale metadata");
    assert!(valid_stale_observation.starts_with(b"version=1\nprocess="));
    valid_stale_observation.extend_from_slice(b"timestamp=plausible-current\n");
    std::fs::write(
        root.join("namespace/mutation.lock"),
        &valid_stale_observation,
    )
    .expect("restore copied valid stale diagnostic bytes");

    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("metadata cannot block a free OS lock");
    let contender =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect_err("metadata cannot grant a contended OS lock");
    assert_eq!(
        contender,
        FilesystemMediaOwnerAdmissionDenial::Ownership(MutationOwnershipDenial::Contended)
    );
    drop(owner);
}

#[test]
fn eight_processes_contend_then_process_death_releases_one_stable_namespace() {
    if std::env::var_os(PROCESS_ROLE_ENV).is_some() {
        return;
    }
    let parent = tempfile::tempdir().expect("process test parent");
    let root = parent.path().join("store");
    let mut children = (0..8)
        .map(|index| spawn_contender(&root, index))
        .collect::<Vec<_>>();

    for contender in &mut children {
        use std::io::Write;
        contender
            .stdin
            .as_mut()
            .expect("child stdin")
            .write_all(&[1])
            .expect("release start barrier");
    }
    let results = children
        .iter_mut()
        .map(ProcessContender::read_result)
        .collect::<Vec<_>>();
    let winners = results
        .iter()
        .enumerate()
        .filter(|(_, result)| result.starts_with("OWNED "))
        .collect::<Vec<_>>();
    assert_eq!(winners.len(), 1, "results: {results:?}");
    assert_eq!(
        results
            .iter()
            .filter(|result| result.as_str() == "CONTENDED")
            .count(),
        7,
        "results: {results:?}"
    );
    let winner_index = winners[0].0;
    let first = parse_owned(&results[winner_index]);
    assert_eq!(first.process, children[winner_index].child.id());
    children[winner_index]
        .child
        .kill()
        .expect("kill winning process");
    for (index, contender) in children.iter_mut().enumerate() {
        drop(contender.stdin.take());
        let status = contender.child.wait().expect("wait contender");
        if index != winner_index {
            assert!(status.success(), "contender {index} failed: {status}");
        }
    }

    let mut successor = spawn_contender(&root, 9);
    use std::io::Write;
    successor
        .stdin
        .as_mut()
        .expect("successor stdin")
        .write_all(&[1])
        .expect("release successor");
    let result = successor.read_result();
    let next = parse_owned(&result);
    assert_eq!(next.process, successor.child.id());
    assert_ne!(first.process, next.process);
    assert_ne!(first.owner, next.owner);
    assert_ne!(first.attempt, next.attempt);
    assert_eq!(first.stable, next.stable);
    drop(successor.stdin.take());
    assert!(successor.child.wait().expect("wait successor").success());
    assert_eq!(
        immediate_names(&root.join("namespace")),
        ["identity", "mutation.lock"]
    );
    assert!(immediate_names(&root.join("families")).is_empty());
    assert!(immediate_names(&root.join("staging")).is_empty());
}

#[test]
fn spawned_child_cannot_inherit_and_extend_the_mutation_lease() {
    if std::env::var_os(PROCESS_ROLE_ENV).is_some() {
        return;
    }
    use std::process::Stdio;

    let parent = tempfile::tempdir().expect("inheritance test parent");
    let root = parent.path().join("store");
    let owner = FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
        .expect("owner");
    let mut child = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .args([
            "--exact",
            "filesystem_media::tests::mutation_ownership::mutation_ownership_process_role",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(PROCESS_ROLE_ENV, "idle-child")
        .env(PROCESS_ROOT_ENV, &root)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn unrelated child");
    drop(owner);

    let successor =
        FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test())
            .expect("child must not inherit the lease");
    drop(successor);
    drop(child.stdin.take());
    assert!(child.wait().expect("wait idle child").success());
}

#[test]
fn mutation_ownership_process_role() {
    let Some(role) = std::env::var_os(PROCESS_ROLE_ENV) else {
        return;
    };
    use std::io::{Read, Write};

    if role == "idle-child" {
        let mut release = [0_u8; 1];
        let _ = std::io::stdin().read(&mut release);
        return;
    }

    let root = std::path::PathBuf::from(
        std::env::var_os(PROCESS_ROOT_ENV).expect("process root environment"),
    );
    let mut start = [0_u8; 1];
    std::io::stdin()
        .read_exact(&mut start)
        .expect("start barrier closed unexpectedly");
    match FilesystemMediaOwner::admit(&root, FilesystemMediaAdmissionAuthority::for_test()) {
        Ok(owner) => {
            let stable = ensure_stable_identity(&owner);
            println!(
                "OWNED {} {} {} {}",
                owner.mutation_owner().process_id(),
                encode_hex(&owner.identity().bytes()),
                encode_hex(&owner.mutation_owner().attempt().bytes()),
                encode_hex(&stable)
            );
            std::io::stdout().flush().expect("flush owner result");
            let mut release = [0_u8; 1];
            let _ = std::io::stdin().read(&mut release);
        }
        Err(FilesystemMediaOwnerAdmissionDenial::Ownership(MutationOwnershipDenial::Contended)) => {
            println!("CONTENDED");
            std::io::stdout().flush().expect("flush contention result");
        }
        Err(error) => panic!("unexpected process owner denial: {error:?}"),
    }
}

fn immediate_names(root: &std::path::Path) -> Vec<String> {
    let mut names = std::fs::read_dir(root)
        .expect("read directory")
        .map(|entry| {
            entry
                .expect("directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}
