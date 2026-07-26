# worth-store

This is the thin public facade for the rebuilt Worth Store workspace.

It should re-export stable public APIs and compose lower crates. It should not
become the place where authority, physical format, recovery, compatibility,
maintenance, or certification logic is implemented.

## Physical Residency Failures

`PhysicalRecordResidencyFailure` is the Store-owned facade for a lower physical
residency denial. `kind()` supplies a broad policy class; `reason()` supplies
the exact causal reason and retains small actionable fields such as bounded-load
limits or candidate cardinality. Callers use the reason to distinguish a bad
declaration or payload from live identity contention, pin/dirty posture,
writeback-claim contention, receipt mismatch, lifecycle closure, or pressure.

The reason vocabulary carries no pool control, retry authority, Signal handle,
scheduler receipt, backend authority, `worth-proof` witness, Foundational
value, or semantic residency claim. Pressure follows its dedicated
`PhysicalRecordPressureEvidence` path, and a terminated frame load follows its
typed terminal projection.
