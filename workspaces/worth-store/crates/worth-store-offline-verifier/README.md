# worth-store-offline-verifier

Owns the independent read-only verifier that walks persisted bytes through root
manifests, segment manifests, page headers, frame headers, slot directories,
and free-space maps without constructing the live store runtime.

This crate is a trust boundary. It should be able to disagree with the live
backend and report that disagreement as evidence.

For C.8 artifact inventory, use:

```text
physical_store_offline_observer c8-recovery-observe <store-root> <report-output> \
  <maximum-directory-entries> <maximum-directories> \
  <maximum-artifacts> <maximum-bytes>
```

The command emits a `store.physical.recovery-observer-report` version 1
envelope. It performs a bounded, deterministic read-only walk. The report is
observer evidence, not a recovery decision and never Store authority.
