Store certification vocabulary.

Raw Store digests cannot satisfy canonical artifact evidence:

```compile_fail
use forge_store_certification::PhysicalFoundationEvidenceBundle;
use forge_store_contracts::{StableArtifactId, StableDigest};
use forge_store_readiness::FoundationalVocabularyAdoptionMap;

let adoption = FoundationalVocabularyAdoptionMap::s1_all_public_lanes().unwrap();
let raw_digest = StableDigest::new("sha256:raw-store-digest").unwrap();
let _builder = PhysicalFoundationEvidenceBundle::builder(adoption)
    .with_canonical_artifact_digest(
        StableArtifactId::new("artifact_digest").unwrap(),
        raw_digest,
    );
```

Public callers cannot skip the scenario harness progression:

```compile_fail
use forge_store_certification::{
    PhysicalProofOracleKind, PhysicalScenarioDefinition, PhysicalScenarioExecution,
    PhysicalScenarioQualityHarness, PhysicalStoryStep, PhysicalSubstrateLane,
};

let definition = PhysicalScenarioDefinition::story("direct_execution_forge")
    .physical_substrate_lane(PhysicalSubstrateLane::HappyAuthority)
    .proves_law("external callers must not mint execution directly")
    .step(PhysicalStoryStep::GivenCleanPhysicalStore)
    .requires_oracle(PhysicalProofOracleKind::ScenarioPlanOwnsStrategy)
    .define()
    .unwrap();
let harness = PhysicalScenarioQualityHarness::roadmap_2();
let plan = harness.lower(definition).unwrap();

let _forged = PhysicalScenarioExecution::from_plan(plan);
```

Raw digests cannot be supplied as binary format evidence:

```compile_fail
use forge_store_certification::BinaryPhysicalFormatEvidence;
use forge_store_contracts::StableDigest;
use forge_store_physical_format::PhysicalBinaryEncodingWitness;

let witness = PhysicalBinaryEncodingWitness::s1_canonical().unwrap();
let digest = StableDigest::new("sha256:raw-binary-format").unwrap();
let _evidence = BinaryPhysicalFormatEvidence::from_witness(&witness, digest);
```

S.2 readiness is minted only by admitted S.1 physical substrate closeout:

```compile_fail
use forge_store_readiness::S2PhysicalSubstrateReadiness;
use forge_store_contracts::ROADMAP_2_S1_SCOPE;

let _forged = S2PhysicalSubstrateReadiness {
    scope: ROADMAP_2_S1_SCOPE,
    facts: todo!(),
    sealed: true,
};
```

Certification closeout cannot mint `S2PhysicalSubstrateReadiness` directly; use `forge_store_readiness`:

```compile_fail
use forge_store_certification::{
    certify_physical_page_segment_extent_substrate, PhysicalPageSegmentExtentSubstrateCloseout,
};

let closeout: PhysicalPageSegmentExtentSubstrateCloseout =
    certify_physical_page_segment_extent_substrate().unwrap();
let _readiness = closeout.into_s2_readiness();
```

S.6 production readiness closure is owned by `forge_store_readiness`, not certification:

```compile_fail
use forge_store_certification::close_s6_production_readiness;
```

S.6 IO/QoS readiness is minted by `forge_store_physical_isolation`, not certification:

```compile_fail
use forge_store_certification::materialize_s6_io_qos_isolation_readiness;
```

Raw closeout evidence descriptors cannot be assembled outside certification:

```compile_fail
use forge_store_certification::PhysicalPageSegmentExtentSubstrateEvidence;

let _forged = PhysicalPageSegmentExtentSubstrateEvidence::new(
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
    unimplemented!(),
);
```

Raw closeout runs cannot be assembled outside certification:

```compile_fail
use forge_store_certification::PhysicalPageSegmentExtentSubstrateRun;
use forge_store_contracts::StableArtifactId;

let _forged = PhysicalPageSegmentExtentSubstrateRun::new(
    StableArtifactId::new("synthetic-closeout").unwrap(),
    unimplemented!(),
);
```

S.3 physical integrity readiness cannot be minted from raw payload fields:

```compile_fail
use forge_store_readiness::S3PhysicalIntegrityReadiness;

let _forged = S3PhysicalIntegrityReadiness {
    s2_readiness: todo!(),
    payload: todo!(),
};
```

S.3 scenario planned-work evidence cannot satisfy executed integrity evidence:

```compile_fail
use forge_store_certification::PhysicalScenarioPlannedWorkBoundaryReport;
use forge_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
};

let planned: PhysicalScenarioPlannedWorkBoundaryReport = todo!();
let _ = PhysicalIntegrityEvidenceAuthority::store_local().materialize(
    planned,
    PhysicalIntegrityEvidenceProfile::full(),
);
```

Raw JSON cannot satisfy page-header native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let raw = serde_json::Value::Null;
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::PhysicalPageHeader,
)
.with_page_header_witness(raw);
```

Raw JSON cannot satisfy aspect-boundary native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let raw = serde_json::Value::Null;
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectBoundaryFact,
)
.with_aspect_boundary_fact(raw);
```

Raw JSON cannot satisfy aspect-patch native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let raw = serde_json::Value::Null;
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
)
.with_aspect_patch_boundary_fact(raw);
```

String text cannot satisfy page-header native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let text = String::from("store.physical.page.header");
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::PhysicalPageHeader,
)
.with_page_header_witness(text);
```

String text cannot satisfy aspect-boundary native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let text = String::from("store.aspect.boundary.fact");
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectBoundaryFact,
)
.with_aspect_boundary_fact(text);
```

String text cannot satisfy aspect-patch native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

let text = String::from("store.aspect.patch.boundary.fact");
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
)
.with_aspect_patch_boundary_fact(text);
```

Terminal projection text cannot satisfy page-header native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily, StoreTerminalProjectionText,
};

let text: StoreTerminalProjectionText = todo!();
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::PhysicalPageHeader,
)
.with_page_header_witness(text);
```

Terminal projection text cannot satisfy aspect-boundary native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily, StoreTerminalProjectionText,
};

let text: StoreTerminalProjectionText = todo!();
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectBoundaryFact,
)
.with_aspect_boundary_fact(text);
```

Terminal projection text cannot satisfy aspect-patch native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily, StoreTerminalProjectionText,
};

let text: StoreTerminalProjectionText = todo!();
let _ = StoreCanonicalBasisConstruction::for_family(
    StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
)
.with_aspect_patch_boundary_fact(text);
```

Generic Serialize inputs cannot satisfy page-header native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

fn try_generic_serialize<T: serde::Serialize>(value: T) {
    let _ = StoreCanonicalBasisConstruction::for_family(
        StoreCanonicalBasisFamily::PhysicalPageHeader,
    )
    .with_page_header_witness(value);
}
```

Generic Serialize inputs cannot satisfy aspect-boundary native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

fn try_generic_serialize<T: serde::Serialize>(value: T) {
    let _ = StoreCanonicalBasisConstruction::for_family(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
    )
    .with_aspect_boundary_fact(value);
}
```

Generic Serialize inputs cannot satisfy aspect-patch native Store canonical basis construction:

```compile_fail
use forge_store_aspect_native::{
    StoreCanonicalBasisConstruction, StoreCanonicalBasisFamily,
};

fn try_generic_serialize<T: serde::Serialize>(value: T) {
    let _ = StoreCanonicalBasisConstruction::for_family(
        StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
    )
    .with_aspect_patch_boundary_fact(value);
}
```

Digest strings cannot construct Store aspect identity authority:

```compile_fail
use forge_store_aspect_native::StoreAspectIdentity;

let digest_text = String::from("sha256:aspect-identity");
let _identity = StoreAspectIdentity::from_digest_text(digest_text);
```

Digest strings cannot construct Store recovery source authority:

```compile_fail
use forge_store_recovery_physics::PhysicalRecoverySource;

let digest_text = String::from("sha256:recovery-source");
let _source = PhysicalRecoverySource::from_digest_text(digest_text);
```

Digest strings cannot construct Store checkpoint authority:

```compile_fail
use forge_store_recovery_physics::CheckpointValidityDecision;

let digest_text = String::from("sha256:checkpoint");
let _checkpoint = CheckpointValidityDecision::from_digest_text(digest_text);
```

Digest strings cannot construct Store page authority:

```compile_fail
use forge_store_physical_format::PhysicalPageId;

let digest_text = String::from("sha256:page");
let _page = PhysicalPageId::from_digest_text(digest_text);
```

Digest strings cannot construct Store WAL authority:

```compile_fail
use forge_store_physical_integrity::WalFrameIntegrityAuthority;

let digest_text = String::from("sha256:wal");
let _wal = WalFrameIntegrityAuthority::from_digest_text(digest_text);
```

Digest strings cannot construct Store certification authority:

```compile_fail
use forge_store_certification::StoreCertificationProgram;

let digest_text = String::from("sha256:certification");
let _program = StoreCertificationProgram::from_digest_text(digest_text);
```
