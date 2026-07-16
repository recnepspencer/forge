# worth-store-formal-models

This crate owns the cold, non-authoritative checked semantics for Worth Store
physical protocols. Runtime owner crates remain the only issuers of executable
outcomes and receipts. Certification combines ordinary executed-owner evidence
with this crate's mappings and checker verdicts.

The authority direction is:

```text
runtime owner outcome
  -> read-only owner observation
  -> exhaustive abstraction mapping
  -> checked model action
  -> diagnostic checker verdict
  -> certification adjudication
```

The pinned TLC runner is described in `formal-toolchain.toml` and exercised by
`scripts/ci/verify_worth_store_formal_toolchain.ps1` or the matching `.sh`
command. Protocol model artifacts
land inside their responsibility-shaped `src/protocols/<family>/` directory.
Each controlled mutant extends the corresponding production model in that same
directory and changes one named transition; the certification mutation runner
must reject and localize all eight before closeout can consume its report.
The toolchain smoke model proves only runner reproducibility; it is not protocol
evidence and receives no S.9 protocol-model credit.
