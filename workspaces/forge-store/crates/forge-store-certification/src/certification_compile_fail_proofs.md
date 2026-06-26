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
use forge_store_certification::S3PhysicalIntegrityReadiness;

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
