use super::BoundedResidencyConfiguration;

const EXACT_WORLD: &str = "\
worth.store.physical-work-courtroom.bounded-residency.configuration.v3
seed=7312955904608109267
inline-record-bytes=3000
inline-records=64
extent-record-bytes=1048576
extent-records=109
total-bytes=6979584
resident-bytes=65536
metadata-bytes=32768
frame-entries=12
resident-frames=8
pinned-frames=4
pin-leases=6
dirty-frames=2
dirty-replacement-bytes=65536
operation-bytes=6815744
checkpoint-memory-bytes=1048576
scope-foreground-read-bytes=2097152
scope-foreground-write-bytes=6815744
scope-recovery-bytes=2359296
scope-scrub-bytes=1835008
scope-maintenance-bytes=1572864
scope-verification-bytes=1048576
scope-blob-bytes=1310720
speculative-prefetch-frames=2
speculative-read-ahead-frames=2
speculative-write-behind-frames=1
";

#[test]
fn strengthened_two_append_world_is_the_only_admitted_profile() {
    let temporary = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temporary.path(), EXACT_WORLD).unwrap();
    let admitted = BoundedResidencyConfiguration::read(temporary.path()).unwrap();

    assert_eq!(admitted.record_count(), 173);
    assert_eq!(admitted.producer_record_count(), 171);
    assert!(admitted.producer_payload_bytes().unwrap() >= 16 * admitted.total_bytes());
    assert_eq!(
        admitted.total_bytes(),
        admitted
            .operation_bytes()
            .saturating_add(admitted.resident_bytes())
            .saturating_add(admitted.metadata_bytes())
            .saturating_add(admitted.dirty_replacement_bytes())
    );
    assert_eq!(admitted.checkpoint_memory_limit().get().get(), 1_048_576);

    for weakened in [
        EXACT_WORLD.replace("extent-records=109", "extent-records=72"),
        EXACT_WORLD.replace("total-bytes=6979584", "total-bytes=6815744"),
        EXACT_WORLD.replace("operation-bytes=6815744", "operation-bytes=4194304"),
        EXACT_WORLD.replace(
            "checkpoint-memory-bytes=1048576",
            "checkpoint-memory-bytes=16777216",
        ),
        EXACT_WORLD.replace(
            "scope-foreground-write-bytes=6815744",
            "scope-foreground-write-bytes=4194304",
        ),
    ] {
        std::fs::write(temporary.path(), weakened).unwrap();
        assert!(BoundedResidencyConfiguration::read(temporary.path()).is_err());
    }
}

#[test]
fn old_schema_is_rejected_before_world_decoding() {
    let temporary = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        temporary.path(),
        "worth.store.physical-work-courtroom.bounded-residency.configuration.v1\n",
    )
    .unwrap();
    assert!(BoundedResidencyConfiguration::read(temporary.path()).is_err());
}

#[test]
fn reopen_retains_the_bounded_checkpoint_memory_policy() {
    let temporary = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temporary.path(), EXACT_WORLD).unwrap();

    let reopened = crate::configuration::ReopenConfiguration::read(temporary.path()).unwrap();
    let crate::configuration::ReopenConfiguration::BoundedResidency(configuration) = reopened
    else {
        panic!("MUTANT_PREDICATE:c7-reopen-bounded-policy-discarded");
    };

    assert_eq!(
        configuration.checkpoint_memory_limit().get().get(),
        1_048_576
    );
}
