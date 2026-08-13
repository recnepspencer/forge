# Physical Recovery And Reopen

## What This Feature Is

Physical recovery lets an operator reopen an existing Worth Store after its
writer process is gone. The recovery command reads persisted physical truth,
builds a non-current generation, publishes it durably, reopens it through a
fresh handle, and returns one terminal result.

## Why You Use It

- restart a Store after process death;
- inspect why a bounded recovery was refused or blocked;
- create a portable runtime report and an independent artifact observation.

It is not backup restore, point-in-time recovery, semantic transaction repair,
or permission to choose an arbitrary older root.

## Stable Entry Points

- `physical_store_recover`
- `WorthStoreRecovery::recover`
- `RecoveryReportEnvelope`
- `physical_store_offline_observer c8-recovery-observe`
- `RecoveryObserverReport`

Application code should use the runtime facade. Lower Store coordination,
backend media, and recovery-physics types are not alternate entry points.

## Core Mental Model

Persisted selectors, checkpoints, manifests, page or extent frames, and WAL are
the source of truth. Recovery derives a plan from them under finite limits.
The runtime report describes what that process concluded. The offline observer
walks the directory independently and describes what it read. Neither report
can authorize writes or be admitted as Store truth.

## How It Executes

The runtime consumes these phases in order: admitted, discovered, selected,
planned, staged, namespace-durable, freshly reopened, then handed off. Cleanup
runs only after the fresh reopen and may defer without invalidating recovered
success. A refusal performs no recovery work; a blocked or indeterminate result
retains exact completed work and effect evidence.

## Small Example

```text
physical_store_recover D:\stores\orders \
  --bounded-profile=c8-phase2-admission-v1 \
  --report=D:\reports\orders-recovery-v1.bin
```

This is the smallest honest operator call: it names an existing Store root, a
finite built-in profile, and an optional descriptive report destination.

## Real Example

Run recovery in one process, then inspect the resulting directory from another:

```text
physical_store_recover D:\stores\orders \
  --bounded-profile=c8-phase2-admission-v1 \
  --report=D:\reports\runtime-v1.bin

physical_store_offline_observer c8-recovery-observe \
  D:\stores\orders D:\reports\observer-v1.bin \
  1000000 100000 1000000 4294967295
```

The first process owns recovery authority and effects. The second owns only a
bounded read-only observation. Comparing the two reports belongs to a
certification or operator tool; the observer must not decide recovery success.

## How It Relates To Other Features

C.4 remains the only physical effect executor. C.5.1 schedules recovery work.
C.7 remains the ordinary durability and checkpoint publisher. The offline
verifier is an independent inspection boundary, not a Store runtime.

## Inspection And Debugging

`RecoveryReportDecodeDenial` distinguishes malformed bytes, wrong protocol
family, unsupported version, and digest damage. The observer has the equivalent
typed denials plus directory-entry, directory, artifact, byte, media, path, and
file-type limits. Version 1 of each protocol accepts exactly version 1.

## Anti-Patterns

- Do not feed either report back into Store admission.
- Do not call recovery while the ordinary writer is live.
- Do not bypass the facade through backend media or Store coordination types.
- Do not treat cleanup deferral as failed recovery.

## Current Limits

The shipped CLI uses one fixed bounded profile. The offline observer inventories
bytes and paths; it does not localize or repair corruption. Cross-version report
migration is unsupported because each initial compatibility window is exactly
version 1.

## Related Docs

- [Physical Durability And Checkpoints](physical-durability-and-checkpoints.md)
- [C.8 Fresh-Process Recovery Specification](physical-reconstruction-c8-fresh-process-recovery-and-reopen.md)
- [Physical Foundation Reconstruction Roadmap](physical-foundation-reconstruction-roadmap.md)
