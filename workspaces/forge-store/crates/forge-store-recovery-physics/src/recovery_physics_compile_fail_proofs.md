S.4 integrity-vetted WAL frames cannot be constructed from raw bytes:

```compile_fail
use forge_store_recovery_physics::IntegrityVettedWalFrame;

let raw: &[u8] = b"not-integrity-evidence";
let _vetted = IntegrityVettedWalFrame::from(raw);
```

S.4 integrity-vetted records cannot be constructed from copied reports without
an executed S.3 handoff receipt:

```compile_fail
use forge_store_physical_integrity::WalFrameIntegrityReport;
use forge_store_recovery_physics::IntegrityVettedWalFrame;

let report: WalFrameIntegrityReport = todo!();
let _vetted = IntegrityVettedWalFrame::from_integrity_report(&report);
```

Quarantine summaries cannot become sealed S.4 payloads:

```compile_fail
use forge_store_recovery_physics::{QuarantineSummary, S4IntegrityHandoffPayload};

let summary: QuarantineSummary = todo!();
let _payload = S4IntegrityHandoffPayload::from(summary);
```

S.4 checksum basis cannot be constructed from loose algorithm and scope parts:

```compile_fail
use forge_store_physical_integrity::{ChecksumAlgorithmId, ChecksumScopeDeclaration};
use forge_store_recovery_physics::S4ChecksumAlgorithmScopeBasis;

let algorithm = ChecksumAlgorithmId::crc32c();
let scope: ChecksumScopeDeclaration = todo!();
let _basis = S4ChecksumAlgorithmScopeBasis::new(algorithm, scope);
```

S.4 bounded inspection evidence cannot be constructed from raw numeric limits:

```compile_fail
use forge_store_recovery_physics::BoundedInspectionEnvelopeEvidence;

let _evidence = BoundedInspectionEnvelopeEvidence::new(1, 1, 1);
```

S.4 readiness cannot be synthesized from raw fields:

```compile_fail
use forge_store_recovery_physics::S4RecoveryPhysicsIntegrityReadiness;

let _forged = S4RecoveryPhysicsIntegrityReadiness {
    payload: todo!(),
};
```

S.4 unresolved authority damage cannot be synthesized from raw digest labels:

```compile_fail
use forge_store_contracts::StableDigest;
use forge_store_recovery_physics::RecoveryBlockedByIntegrityDamage;

let digest = StableDigest::new("fixture-owned-unresolved-authority").unwrap();
let _damage = RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(digest, None);
```
