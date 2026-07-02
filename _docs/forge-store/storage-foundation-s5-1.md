# S.5.1: Cryptographic Boundary Seeds And Tenant Scope Metadata

## Goal

Introduce the cryptographic, authenticity, and tenant-scope metadata that would
have been cheapest to reserve during `S.1`, `S.3`, and `S.4`, without
rewriting those closed milestone scopes or pretending they already shipped it.

## Why This Milestone Exists

`S.5` establishes physically stable reads. Before Roadmap 2 proceeds into I/O
QoS, blobs, backup, repair, and full security, Store needs typed metadata,
admission, and proof surfaces for key scope, tenant scope, authenticity class,
and encrypted-frame compatibility. These are not full encryption, identity, or
compliance features. They are the structural seeds that make later security
work impossible to forget or bolt on dishonestly.

## Governing Summaries

- `MENTALITY.md` protects hard-problem-first design: security metadata that
  affects page identity, frame admission, backup, blobs, and repair must be
  made structural before later features multiply.
- `arch_laws.md` protects proof-bearing construction: key scope, tenant scope,
  authenticity class, and encrypted-frame readiness must become typed proofs,
  not comments or raw labels.
- `composition_laws.md` protects responsibility boundaries: cryptographic
  metadata, tenant scope, authenticity admission, and terminal projections must
  live in named modules instead of becoming helper residue inside page, WAL, or
  blob code.
- `domain_structure_laws.md` protects authority topology: semantic truth,
  physical byte survival, authenticity evidence, and tenant blast radius must
  remain separate structural spaces.
- `perf_laws.md` protects visible cost: security metadata must carry exact
  counter surfaces so encryption/authenticity readiness does not hide broad
  scans, unbounded rewrap work, or per-page allocation surprises.

## Adversarial Constraint

Later Store work must not be able to introduce encrypted pages, authenticated
frames, tenant-scoped blobs, backup capsules, PITR bundles, or repair actions
through raw strings, ambient deployment assumptions, or terminal JSON
projections. A page, frame, WAL record, blob chunk, backup bundle, export
capsule, or repair plan that lacks typed key scope, tenant scope, authenticity
class, and key-version posture must fail admission before later milestones can
claim security, tenant isolation, or auditability.

## Product Decision Lock

Store does not become an identity provider. Store consumes typed admission
evidence from external identity systems where needed, but Store owns durable
cryptographic authority boundaries: key scope, tenant scope, authenticity
evidence, key-version posture, backup/export custody, and tenant-scoped repair
blast radius.

## Phase Plan

### Phase 1: Security Scope Vocabulary And Authority Separation

Freeze the vocabulary that distinguishes semantic authority, physical byte
authority, authenticity authority, key custody, and tenant blast-radius scope.

**Relevant subsystems**

- `forge-store-physical-format`
- `forge-store-proof`
- `forge-store-certification`
- `forge-foundational`
- `forge-proof`

**Relevant APIs**

- Store physical witnesses from `S.1` through `S.5`
- Foundational aspect-native value and canonical-basis surfaces
- Proof receipt, evidence, suite, and certification vocabulary

**Warnings**

- Do not model key ids, tenant ids, or authenticity classes as raw strings.
- Do not let semantic artifact identity double as cryptographic authority.
- Do not introduce actual encryption algorithms here; this phase freezes
  typed scope and authority categories.

**Test requirements**

- Adversarial equivalence: two physically identical page witnesses with
  different tenant scopes or key scopes are not equivalent for security
  admission even if their semantic commit basis matches.
- Adversarial rejection: raw string tenant labels, terminal JSON key labels, and
  unclassified authenticity labels cannot satisfy any cryptographic scope API.
- Authority-boundary compile-fail: semantic commit ids cannot be passed where
  current key-scope or tenant-scope witnesses are required.

**Engineering decisions**

- Introduce distinct Store-owned types for key scope, key version, tenant
  scope, authenticity class, and cryptographic custody posture.
- Carry Foundational canonical-basis vocabulary at evidence boundaries, while
  keeping physical cryptographic authority Store-owned.
- Represent unavailable cryptographic capability as typed unsupported posture,
  not as omitted fields.

**Open questions**

- Exact external identity admission formats are deferred to `S.11`; this
  milestone only creates Store-owned durable scope vocabulary.

### Phase 2: Physical Metadata Backfill For Pages, Frames, WAL, And Manifests

Add typed security metadata compatibility to the physical structures already
introduced by earlier milestones.

**Relevant subsystems**

- physical page headers
- frame headers
- WAL/checkpoint records
- segment and root manifests
- physical root admission

**Relevant APIs**

- page/frame header constructors
- WAL record framing
- manifest/root publication witnesses
- S.5 stable read-plan admission

**Warnings**

- Do not rewrite S.1, S.3, or S.4 history. This phase explicitly backfills the
  metadata they now need to carry forward.
- Do not make metadata optional in ordinary platform-grade paths; unsupported
  capability must be explicit.
- Do not use metadata as authenticity proof until later phases admit the proof.

**Test requirements**

- Adversarial parity: page, frame, WAL, and manifest witnesses preserve their
  existing physical identity while additionally carrying typed key scope,
  tenant scope, authenticity class, and key-version posture.
- Adversarial rejection: a stale page/frame witness or WAL record missing
  security metadata cannot be admitted into a platform-grade physical read or
  recovery lane.
- Drift localization: mismatched tenant scope between page header and manifest
  is reported as scope drift before logical decode.

**Engineering decisions**

- Metadata belongs in Store physical witnesses and canonical basis rows, not in
  serde/JSON projections.
- S.5 stable read plans must preserve security scope when they protect physical
  reads.
- Frame and WAL compatibility must reserve enough structure for later
  encryption/authentication without choosing algorithms now.

**Open questions**

- Exact binary layout expansion strategy may be selected by implementation,
  but the public witness vocabulary may not remain absent.

### Phase 3: Authenticity Distinct From Integrity

Make authenticity admission structurally separate from checksums and physical
corruption detection.

**Relevant subsystems**

- S.3 physical integrity reports
- scrub and quarantine evidence
- frame/page admission
- certification evidence rows

**Relevant APIs**

- checksum validation reports
- physical quarantine reports
- proof evidence rows for corruption and drift

**Warnings**

- A checksum match is not authenticity success.
- A content digest is not proof that the bytes came from the admitted key
  scope, tenant scope, or custody posture.
- Do not let later operator tooling infer authenticity from "no corruption."

**Test requirements**

- Adversarial equivalence: a page can be checksum-valid while authenticity is
  unavailable, unsupported, or failed, and the result must remain
  machine-distinguishable.
- Adversarial rejection: authenticity-required lanes reject checksum-valid
  bytes when the authenticity witness is absent, stale, wrong-scope, or
  unsupported.
- Certification localization: reports distinguish `corrupt`, `authenticity
  failed`, `authenticity unavailable`, and `authenticity unsupported`.

**Engineering decisions**

- Add typed authenticity result categories independent of integrity categories.
- Carry exact counters for checksum-valid/authenticity-failed and
  checksum-valid/authenticity-unavailable cases.
- Keep authenticity evidence policy-switchable without changing physical decode
  results.

**Open questions**

- Algorithm choice and MAC/signature mechanics remain `S.11` work.

### Phase 4: Blob, Chunk, Backup, Export, And Repair Readiness Seeds

Make later blob, backup, export, and repair work consume typed security scope
instead of retrofitting it into their artifact models.

**Relevant subsystems**

- S.7 blob chunk metadata
- S.10 backup, PITR, disaster recovery, and forensics
- S.11 security and key lifecycle
- Roadmap 1 Milestones 14, 20, and 22

**Relevant APIs**

- chunk-tree manifest plans
- backup/PITR bundle declarations
- export/import capsule declarations
- repair/quarantine plan declarations

**Warnings**

- Do not dedupe blob chunks across tenant/key scopes unless the later security
  policy explicitly admits that equivalence.
- Do not let backup/export capsules omit key-scope and custody posture.
- Do not let repair plans cross tenant blast-radius boundaries by default.

**Test requirements**

- Adversarial equivalence: identical blob content under different tenant or key
  scopes does not collapse into a shared physical claim unless an admitted
  dedupe policy proves it safe.
- Adversarial rejection: backup/export/repair plans without typed key custody,
  tenant scope, and authenticity posture cannot enter platform-grade lanes.
- Blast-radius proof: a tenant-scoped repair plan cannot reference physical
  regions outside its admitted tenant scope.

**Engineering decisions**

- Add readiness witnesses for blob chunks, backup bundles, export capsules, and
  repair plans that carry key scope, tenant scope, authenticity class, and
  custody posture.
- Use Proof evidence for readiness receipts; keep Store-owned physical witness
  types as the authority.
- Make dedupe policy require explicit equivalence basis rather than digest-only
  equality when tenant or key scope differs.

**Open questions**

- Whether cross-tenant encrypted dedupe is ever allowed remains a later product
  and security decision; this milestone must make the unsafe default
  unrepresentable.

### Phase 5: Harness And Certification Readiness

Extend the S.4.5/S.5 harness path so security-scope failures can be modeled
before full S.11 encryption exists.

**Relevant subsystems**

- S.4.5 simulation harness
- S.5 physical isolation scenarios
- Store certification crate
- Proof suite/evidence vocabulary

**Relevant APIs**

- scenario authoring APIs
- schedule lowering
- observer/oracle/evidence surfaces
- transcript and counter evidence

**Warnings**

- This is not S.11 certification. It is security-scope readiness that prevents
  later milestones from ignoring the metadata.
- Do not add log-based proof or JSON-shaped verdicts.
- Do not make the harness decide identity-provider semantics.

**Test requirements**

- Adversarial simulation: hostile interleavings preserve key scope and tenant
  scope across stable read plans, root swaps, checkpoint publication, and
  repair-read admission.
- Adversarial rejection: simulated stale key version, wrong tenant scope,
  missing authenticity requirement, and replayed custody posture produce typed
  denials before logical decode.
- Counter proof: exact counters cover security-scope admissions, denials,
  stale-key rejections, tenant-scope drifts, authenticity-unavailable results,
  and unsupported-capability denials.

**Engineering decisions**

- Add `S6SecurityScopeReadiness` or equivalent handoff so `S.6` and later
  milestones cannot forget security metadata.
- The JSON runner state remains progress-only; security evidence must live in
  typed Store/Proof artifacts.
- S.11 must consume this readiness artifact rather than inventing metadata from
  scratch.

**Open questions**

- Final naming of the handoff type can follow implementation topology, but its
  responsibility must remain security-scope readiness for later physical work.

## Must Ship

- Store-owned key scope, key version, tenant scope, authenticity class, and
  custody posture vocabulary.
- Physical metadata compatibility for pages, frames, WAL/checkpoint records,
  manifests, stable read plans, blob chunks, backup/export bundles, and repair
  plans.
- Distinct integrity versus authenticity result categories and counters.
- Typed readiness witnesses for later blob, backup, export, repair, and S.11
  security work.
- Harness scenarios and certification evidence proving security-scope metadata
  survives physical interleavings and rejects stale, missing, or wrong-scope
  inputs.

## Must Preserve

- Store owns physical byte survival and cryptographic boundary evidence.
- `forge-relational` owns semantic truth, MVCC, identity semantics, and
  transaction meaning.
- External identity systems may provide admission evidence, but Store does not
  become an identity provider.
- S.1, S.3, and S.4 are not rewritten retroactively; their missing security
  metadata is backfilled here as explicit follow-on foundation work.
- JSON remains confined to terminal projection or hostile/readmission lanes.

## Acceptance Evidence

- Compile-fail tests proving raw strings, semantic ids, JSON projections, and
  lower-authority digests cannot satisfy key-scope, tenant-scope,
  authenticity, or custody APIs.
- Runtime and simulation tests proving stable read plans, manifests,
  WAL/checkpoint records, blob readiness, backup readiness, and repair
  readiness carry security scope through admitted physical paths.
- Adversarial tests proving wrong tenant, stale key version, missing
  authenticity requirement, unsupported secure posture, and cross-scope dedupe
  are rejected with typed diagnostics.
- Exact counters for security-scope admissions, key-version observations,
  tenant-scope drifts, authenticity failures/unavailable results,
  unsupported-capability denials, and cross-scope repair rejections.
- `S.6` and `S.11` handoff artifacts proving later milestones consume the
  security-scope foundation rather than inventing parallel metadata.

## Sequencing Notes

`S.5.1` belongs immediately after `S.5` because physical read stability must
carry security scope before I/O QoS, blob chunks, backup/PITR, repair, and full
security work depend on those physical paths. It is intentionally before `S.6`
and `S.7`; otherwise encrypted/authenticated I/O and blob metadata become
retrofit work.
