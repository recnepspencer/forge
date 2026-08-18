# Exact Branch References

`worth-foundational` owns the descriptive vocabulary shared by Relational and
Signal when a mutable branch reference crosses a crate boundary. It does not
own a branch table, currentness, a lock, a retention lease, or an authority
proof.

## The grammar

An exact observation has exactly three structural axes:

```text
FoundationalBranchReferenceObservation<TargetBasis>
    = branch identity
    + FoundationalBranchTarget<TargetBasis>
    + reference generation
```

`FoundationalBranchTarget` is explicit:

- `Empty` means the branch has no committed target yet.
- `Basis(value)` means the branch targets one immutable owner descriptor.

An owner descriptor implements `FoundationalBranchTargetBasis`. Its canonical
encoding supplies a domain tag, a non-zero schema version, and deterministic
bytes. The descriptor may carry an owner runtime or graph identity when that
identity is part of the owner's meaning. A digest alone is not an owner
identity.

`FoundationalBranchForkBasis<T>` retains the complete source observation. It
does not accept a branch name plus an epoch, and it cannot manufacture a child
reference. `FoundationalBranchComparisonBasis<T>` retains the complete
expected observation for a conditional movement. `FoundationalBranchReferenceMovement<T>`
describes before, after, and a structural kind (`Fork`, `Truth`, `Metadata`, or
`Lifecycle`) without claiming that an owner performed the movement.

## Generations and mismatch

`FoundationalBranchReferenceGeneration::initial()` is zero. Owners use
`checked_advance()` for every successful reference movement, including a
metadata-only movement. Overflow returns
`FoundationalBranchReferenceGenerationAdvanceDenial::Overflow`; it never wraps
or saturates.

Comparing an expected observation with an observed one returns every differing
axis in deterministic order:

1. `BranchIdentity`
2. `TargetBasis`
3. `ReferenceGeneration`

The mismatch retains both complete observations. This makes cross-runtime,
equal-ordinal, equal-digest, empty-versus-basis, and stale-generation twins
visible to a blind consumer.

## Canonical encoding and transport

`FoundationalBranchReferenceObservation::canonical_encoding()` is a stable,
versioned descriptive byte sequence. It includes a format tag, branch identity,
the explicit target variant, the target's domain/version/bytes when present,
and the generation. It is suitable for deterministic comparison and carriage;
it is not a proof, digest authority, freshness check, or currentness token.

The exact grammar derives `Serialize` and `Deserialize` for descriptors that
provide those serde implementations. Deserialization weakens freshness. The
owner must read the descriptor through its own readmission surface before any
operation.

## Candidate vocabulary is separate

Milestone 5's non-authoritative epoch/equivalence facts are named
`FoundationalBranchCandidateForkBasis`,
`FoundationalBranchCandidateObservationBasis`,
`FoundationalBranchCandidateForkObservationBasis`, and
`FoundationalBranchCandidateComparisonBasis`. They remain valid for candidate
and staged artifacts, but cannot be converted into an exact reference by a
compatibility constructor. In particular, `EquivalenceBasisId` and
`BoundaryEpoch` do not implement `FoundationalBranchTargetBasis`.

For candidate and staged branch work, read
[Branch-Local Candidates And Staged Branches](./branch-local-candidates-and-staged-branches.md).
For an operational branch, the owner must provide the exact descriptor and
then issue its own admitted basis and proof-bearing authority.
