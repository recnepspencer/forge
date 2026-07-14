
S.3 entry witnesses cannot be synthesized by external callers:

```compile_fail
use worth_store_physical_integrity::IntegrityEntryWitness;

let _forged = IntegrityEntryWitness {
    basis: todo!(),
};
```

Raw byte slices cannot become protected physical byte views:

```compile_fail
use worth_store_physical_integrity::ProtectedPhysicalByteView;

let raw = b"unprotected";
let _view = ProtectedPhysicalByteView::from_raw_bytes(raw);
```

External callers cannot forge stronger protected-byte access through public fields:

```compile_fail
use worth_store_physical_integrity::ProtectedPhysicalByteView;

let _view = ProtectedPhysicalByteView {
    bytes: b"field-forged",
};
```

Expired protected views cannot be widened into S.3 admission lifetime:

```compile_fail
use worth_store_buffer_pool::PinnedFrameView;
use worth_store_physical_integrity::ProtectedPhysicalByteView;

fn widen<'short>(
    view: &'short PinnedFrameView<'short>,
) -> ProtectedPhysicalByteView<'static> {
    ProtectedPhysicalByteView::from_pinned_frame(view)
}
```

Backend-private handles cannot enter S.3 integrity admission:

```compile_fail
use worth_store_physical_integrity::ProtectedPhysicalByteView;

struct BackendPrivateHandle;

let handle = BackendPrivateHandle;
let _view = ProtectedPhysicalByteView::from_backend_private_handle(handle);
```

File paths cannot enter S.3 integrity admission as byte authority:

```compile_fail
use worth_store_physical_integrity::IntegrityEntryRequest;
use std::path::Path;

let path = Path::new("store/page-1.bin");
let _request = IntegrityEntryRequest::new(path);
```

Pinned frame views must be explicitly lowered to protected physical byte views:

```compile_fail
use worth_store_buffer_pool::PinnedFrameView;
use worth_store_physical_integrity::IntegrityEntryRequest;

fn unlowered_request<'lease>(view: PinnedFrameView<'lease>) {
    let _request = IntegrityEntryRequest::new(view);
}
```

Copied readiness payloads cannot enter integrity admission:

```compile_fail
use worth_store_physical_integrity::IntegrityEntryAdmission;
use worth_store_readiness::S3PhysicalIntegrityReadinessPayload;

let copied_report: S3PhysicalIntegrityReadinessPayload = todo!();
let _admission = IntegrityEntryAdmission::from_s3_readiness(copied_report);
```

Copied typed readiness reports cannot be replayed into multiple admissions:

```compile_fail
use worth_store_physical_integrity::IntegrityEntryAdmission;
use worth_store_readiness::S3PhysicalIntegrityReadiness;

fn replay_copied_readiness(readiness: S3PhysicalIntegrityReadiness) {
    let _first = IntegrityEntryAdmission::from_s3_readiness(readiness);
    let _second = IntegrityEntryAdmission::from_s3_readiness(readiness);
}
```

Checksum declarations are sealed proof values:

```compile_fail
use worth_store_physical_integrity::ChecksumAlgorithmDeclaration;

let _forged = ChecksumAlgorithmDeclaration {
    basis: todo!(),
    foundational_evidence_identity: todo!(),
};
```

Artifact digests cannot substitute for checksum algorithm declarations:

```compile_fail
use worth_store_contracts::StableDigest;
use worth_store_physical_integrity::ChecksumAlgorithmId;

let digest = StableDigest::new("sha256:not-a-page-checksum").unwrap();
let _ = ChecksumAlgorithmId::admit_claim(digest);
```

Raw strings cannot enter admitted checksum declarations directly:

```compile_fail
use worth_store_physical_integrity::ChecksumAlgorithmDeclaration;

let _ = ChecksumAlgorithmDeclaration::declare("crc32c", todo!(), todo!());
```

Logical decoders cannot consume raw protected physical views:

```compile_fail
use worth_store_physical_integrity::{LogicalDecodeGate, ProtectedPhysicalByteView};

fn semantic_decode(_: LogicalDecodeGate<'_>) {}

fn raw_view_cannot_decode(view: ProtectedPhysicalByteView<'_>) {
    semantic_decode(view);
}
```

Logical decoders cannot consume checksum-planned forms:

```compile_fail
use worth_store_physical_integrity::{
    ChecksumAlgorithmDeclaration, LogicalDecodeGate,
};

fn semantic_decode(_: LogicalDecodeGate<'_>) {}

fn checksum_plan_cannot_decode(plan: ChecksumAlgorithmDeclaration) {
    semantic_decode(plan);
}
```

Declared checksum values cannot satisfy semantic decode admission:

```compile_fail
use worth_store_physical_integrity::{DeclaredPhysicalChecksum, LogicalDecodeGate};

fn semantic_decode(_: LogicalDecodeGate<'_>) {}

let checksum = DeclaredPhysicalChecksum::new(7);
semantic_decode(checksum);
```

External callers cannot synthesize logical decode gates from public fields:

```compile_fail
use worth_store_physical_integrity::LogicalDecodeGate;

let _gate = LogicalDecodeGate {
    bytes: b"unchecked",
    witness: todo!(),
    counters: todo!(),
};
```

External callers cannot synthesize integrity-checked frames:

```compile_fail
use worth_store_physical_integrity::IntegrityCheckedFrame;

let _frame = IntegrityCheckedFrame {
    view: todo!(),
    witness: todo!(),
    checksum: todo!(),
    counters: todo!(),
    evidence: todo!(),
};
```

External callers cannot synthesize physical scope admissions:

```compile_fail
use worth_store_physical_integrity::PhysicalScopeAdmission;

let _admission = PhysicalScopeAdmission {
    checked: todo!(),
    basis: todo!(),
};
```

External callers cannot synthesize family validator inputs:

```compile_fail
use worth_store_physical_integrity::ScopedPhysicalValidatorInput;

let _input = ScopedPhysicalValidatorInput {
    admission: todo!(),
    family: todo!(),
};
```

Family validators cannot consume integrity-checked frames before scope admission:

```compile_fail
use worth_store_physical_integrity::{
    IntegrityCheckedFrame, ScopedPhysicalValidatorInput,
};

fn validate_wal_frame(_: ScopedPhysicalValidatorInput<'_>) {}

let checked: IntegrityCheckedFrame<'_> = todo!();
validate_wal_frame(checked);
```

Physical container inspection cannot consume raw byte slices:

```compile_fail
use worth_store_physical_integrity::PhysicalContainerIntegrity;

let raw = b"raw-physical-bytes";
let _ = PhysicalContainerIntegrity::inspect_page(raw);
```

Physical container inspection cannot consume checksum-checked forms before
physical scope admission:

```compile_fail
use worth_store_physical_integrity::{
    IntegrityCheckedPage, PhysicalContainerIntegrity,
};

let checked: IntegrityCheckedPage<'_> = todo!();
let _ = PhysicalContainerIntegrity::inspect_page(checked);
```

Frame container inspection cannot consume raw byte slices:

```compile_fail
use worth_store_physical_integrity::PhysicalContainerIntegrity;

let raw = b"raw-frame-bytes";
let _ = PhysicalContainerIntegrity::inspect_frame(raw);
```

Frame container inspection cannot consume checksum-checked forms before
physical scope admission:

```compile_fail
use worth_store_physical_integrity::{
    IntegrityCheckedFrame, PhysicalContainerIntegrity,
};

let checked: IntegrityCheckedFrame<'_> = todo!();
let _ = PhysicalContainerIntegrity::inspect_frame(checked);
```

External callers cannot forge rebuildable derived damage:

```compile_fail
use worth_store_physical_integrity::RebuildableDerivedDamage;

let _forged = RebuildableDerivedDamage {
    damaged_scope: todo!(),
    prerequisites: todo!(),
    rebuild_input: todo!(),
};
```

Derived rebuild inputs cannot satisfy APIs requiring intact authority:

```compile_fail
use worth_store_physical_integrity::{DerivedRebuildInput, ManifestIntegrityReport};

fn requires_authority(_: ManifestIntegrityReport) {}

let derived: DerivedRebuildInput = todo!();
requires_authority(derived);
```

Copied rebuildable damage cannot re-enter derived-index classification as an
intact authority basis:

```compile_fail
use worth_store_physical_integrity::{
    DerivedIndexIntegrityInspectionRequest, RebuildableDerivedDamage,
    ScopedPhysicalValidatorInput,
};

let input: ScopedPhysicalValidatorInput<'_> = todo!();
let copied_damage: RebuildableDerivedDamage = todo!();

let _request = DerivedIndexIntegrityInspectionRequest::from_admitted_scope(
    input,
    copied_damage,
);
```

Derived damage reports cannot be treated as integrity-checked physical pages:

```compile_fail
use worth_store_physical_integrity::{IndexPageIntegrityReport, IntegrityCheckedPage};

fn requires_checked_page(_: IntegrityCheckedPage<'_>) {}

let report: IndexPageIntegrityReport = todo!();
requires_checked_page(report);
```

WAL-frame callers cannot declare checksum failure instead of presenting
physically admitted WAL bytes:

```compile_fail
use worth_store_physical_integrity::{
    ChecksumAlgorithmId, ScopedPhysicalValidatorInput, WalFrameIntegrityInspectionRequest,
};

let input: ScopedPhysicalValidatorInput<'_> = todo!();
let _request = WalFrameIntegrityInspectionRequest::with_checksum_failure(
    input,
    ChecksumAlgorithmId::crc32c(),
);
```

Chunk integrity streaming windows cannot be minted from caller-declared raw
byte counts:

```compile_fail
use worth_store_physical_integrity::ChunkIntegrityStreamingWindow;

let _window = ChunkIntegrityStreamingWindow::bounded(4096, 1024);
```

S.2 resident-frame tokens cannot substitute for S.1 durable physical scope:

```compile_fail
use worth_store_buffer_pool::ResidentFrameToken;
use worth_store_physical_format::PhysicalReferenceScope;

fn requires_durable_scope(_: PhysicalReferenceScope) {}

let resident_token: ResidentFrameToken = todo!();
requires_durable_scope(resident_token);
```

S.2 resident-frame tokens cannot substitute for S.1 durable generation owners:

```compile_fail
use worth_store_buffer_pool::ResidentFrameToken;
use worth_store_physical_format::PhysicalGenerationOwner;
use worth_store_physical_integrity::GenerationIntegrityReport;

let durable_owner: PhysicalGenerationOwner = todo!();
let resident_token: ResidentFrameToken = todo!();

let _ = GenerationIntegrityReport::compare(durable_owner, resident_token);
```

Quarantine records cannot be synthesized from copied fields:

```compile_fail
use worth_store_physical_integrity::QuarantineRecord;

let _forged = QuarantineRecord {
    locality: todo!(),
    damage_classification: todo!(),
    receipt: todo!(),
    lifecycle_posture: todo!(),
    handoff_posture: todo!(),
};
```

Executed quarantine findings cannot be synthesized from copied locality and
damage fields:

```compile_fail
use worth_store_physical_integrity::ExecutedQuarantineFinding;

let _forged = ExecutedQuarantineFinding {
    locality: todo!(),
    damage_classification: todo!(),
};
```

Quarantine receipts cannot be synthesized from copied receipt basis:

```compile_fail
use worth_store_physical_integrity::QuarantineReceipt;

let _forged = QuarantineReceipt {
    basis: todo!(),
};
```

Foundational quarantine receipt basis cannot be synthesized from copied digest
fields:

```compile_fail
use worth_store_physical_integrity::FoundationalQuarantineReceiptBasis;

let _forged = FoundationalQuarantineReceiptBasis {
    receipt_kind: todo!(),
    digest: todo!(),
};
```

Raw path strings cannot mint quarantine seal requests:

```compile_fail
use worth_store_physical_integrity::QuarantineSealRequest;

let _request = QuarantineSealRequest::from_raw_path("fixture/page-7");
```

Derived rebuild inputs cannot satisfy quarantine executed-finding authority:

```compile_fail
use worth_store_physical_integrity::{DerivedRebuildInput, QuarantineSealRequest};

let derived_only: DerivedRebuildInput = todo!();
let _request = QuarantineSealRequest::from_executed_finding(derived_only);
```

Foundational S.3 evidence bundles cannot satisfy checked-page authority:

```compile_fail
use worth_store_physical_integrity::{IntegrityCheckedPage, PhysicalIntegrityEvidenceBundle};

fn requires_checked_page(_: IntegrityCheckedPage<'_>) {}

let evidence: PhysicalIntegrityEvidenceBundle = todo!();
requires_checked_page(evidence);
```

Proof-compatible S.3 evidence reports cannot satisfy quarantine record authority:

```compile_fail
use worth_store_physical_integrity::{IntegrityProofProgressionReport, QuarantineRecord};

fn requires_quarantine_record(_: QuarantineRecord) {}

let report: IntegrityProofProgressionReport = todo!();
requires_quarantine_record(report);
```

Planned work role claims cannot satisfy Store executed evidence APIs:

```compile_fail
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile, ScrubPlan,
    StorePlannedWorkBoundaryReport,
};

let plan: ScrubPlan<'_> = todo!();
let planned = StorePlannedWorkBoundaryReport::from_scrub_plan(&plan);
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    planned,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Support-only role claims cannot satisfy Store executed evidence APIs:

```compile_fail
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    StoreSupportOnlyBoundaryClaim,
};

let support: StoreSupportOnlyBoundaryClaim = todo!();
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    support,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Derived-projection role claims cannot satisfy Store executed evidence APIs:

```compile_fail
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    StoreDerivedProjectionBoundaryClaim,
};

let derived: StoreDerivedProjectionBoundaryClaim = todo!();
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    derived,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Receipt-evidence role claims cannot satisfy Store executed evidence APIs:

```compile_fail
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    StoreReceiptEvidenceBoundaryClaim,
};

let receipt: StoreReceiptEvidenceBoundaryClaim = todo!();
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    receipt,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Raw strings and log excerpts cannot satisfy Store executed evidence APIs:

```compile_fail
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
};

let log_excerpt = "page 7 looked clean in the operator log";
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    log_excerpt,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Copied quarantine fields cannot mint S.3 evidence bundles:

```compile_fail
use worth_store_physical_integrity::PhysicalIntegrityEvidenceBundle;

let _forged = PhysicalIntegrityEvidenceBundle {
    category: todo!(),
    role: todo!(),
    outcome: todo!(),
    locality: todo!(),
    counters: todo!(),
    denial_count: 0,
    optional_forensic_material_count: 0,
    diagnostic: todo!(),
    provenance: todo!(),
    performance: todo!(),
    receipt: todo!(),
    store_claim: todo!(),
    materialization_path: todo!(),
};
```
