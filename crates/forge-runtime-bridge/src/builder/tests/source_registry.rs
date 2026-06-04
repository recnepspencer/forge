use super::support::*;
use crate::error::BridgeBuildErrorKind;
use crate::facade::RuntimeBridgeBuilder;
use crate::source::{BridgeSourceCapability, BridgeSourceCapabilitySet};

#[test]
fn build_rejects_duplicate_source_declarations() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("duplicate source declarations should fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::DuplicateSourceDeclaration
    );
}

#[test]
fn build_rejects_source_declarations_without_source_adapter() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("source declarations without source adapter should fail");

    assert_eq!(error.kind(), BridgeBuildErrorKind::MissingSourceAdapter);
}

#[test]
fn build_rejects_multiple_source_adapters() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("multiple source adapters should fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::BuilderConfigurationConflict
    );
}

#[test]
fn build_source_registry_digest_is_order_invariant() {
    let first = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ]),
        })
        .register_source(source_declaration(
            "source:a",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_source(source_declaration(
            "source:b",
            "snapshot-b",
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("first builder order should succeed");

    let second = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:b",
            "snapshot-b",
            vec![
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::SnapshotRead,
            ],
        ))
        .register_source(source_declaration(
            "source:a",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("second builder order should succeed");

    assert_eq!(
        first.source_registry().digest(),
        second.source_registry().digest()
    );
}

#[test]
fn build_rejects_source_capability_mismatch_before_runtime_construction() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile-history",
            "snapshot-a",
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("unsupported source capability should fail before runtime construction");

    assert_eq!(error.kind(), BridgeBuildErrorKind::SourceCapabilityMismatch);
}
