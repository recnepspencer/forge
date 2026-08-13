# Worth Store Recovery Runtime

This crate owns fresh-process physical recovery orchestration. It accepts only
an existing Store root, qualified recovery media, finite limits, current static
configuration, and a concrete platform authority minted in this process.

It never imports a live ordinary Store runtime, a C.7 recovery handoff, decoded
observer artifacts, or caller-supplied Signal and scheduler instances. Entry
constructs a new recovery session, Signal graph, and bounded C.5.1 scheduling
envelope. Persisted stable Store identity joins the authority world only after
the backend has admitted the existing namespace.

The production entry runs the complete consuming recovery progression:

```text
physical_store_recover <store-root> --bounded-profile=c8-phase2-admission-v1 \
  [--report=<path>]
```

The optional report is a descriptive `store.physical.recovery-report` version
1 envelope. Decoding it never grants Store authority and does not replace the
returned process status or a fresh reopen.
