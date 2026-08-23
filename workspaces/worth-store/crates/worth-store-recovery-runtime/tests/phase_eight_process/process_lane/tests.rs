use super::close_owned_lane;

#[test]
fn explicit_close_rejects_a_substituted_owner_token_without_removing_the_lane() {
    let temporary = tempfile::tempdir().unwrap();
    let lane = temporary.path().join("lane");
    let owner = lane.join(".owner");
    std::fs::create_dir(&lane).unwrap();
    std::fs::write(&owner, "actual-owner").unwrap();

    let error = match close_owned_lane(&lane, &owner, "substituted-owner") {
        Err(error) => error,
        Ok(()) => panic!("MUTANT_PREDICATE:c8-process-lane-owner-substitution"),
    };

    assert!(error.contains("owner token changed"), "{error}");
    assert!(owner.is_file());
    assert!(lane.is_dir());
    close_owned_lane(&lane, &owner, "actual-owner").unwrap();
    assert!(!lane.exists());
}

#[test]
fn explicit_close_removes_both_owner_and_lane() {
    let temporary = tempfile::tempdir().unwrap();
    let lane = temporary.path().join("lane");
    let owner = lane.join(".owner");
    std::fs::create_dir(&lane).unwrap();
    std::fs::write(&owner, "owner").unwrap();

    close_owned_lane(&lane, &owner, "owner").unwrap();

    assert!(!owner.exists());
    assert!(!lane.exists());
}
