# Gate 8.2 Turn 2 — Boundary Brief And Plan

## Stage 1: Boundary Brief

### Truth entering
- Gate 8.1 aftermath + Declared external effect on NotifyDeath declaration (bank-domain).
- bank-external-rail: real separate-process TCP boundary (entry met).
- Turn-1 Query surfaces: postures, correlation, outbox helper, classify_transport_fault — production-dead.
- Idempotency co-commit is the mechanical precedent for outbox.

### This slice owns
- Production dispatch after durable commit that calls the real rail and classify_transport_fault.
- Outbox schema + co-commit into MutationIntent when Declared; zero when None.
- Threading InstalledExternalEffectContract onto the bank application commit path.
- Real Runtime* time rename (owner leaves authorization/).
- Carry unresolved-commit evidence past bank discard where host maps outcomes.
- Exact external_dispatch counters; CDC non-use stated.

### Adjacent continues to own
- Recovery handle (8.3), undo/redo, publication cutover.
- Rail process truth (Bank ledger) — Query never reads it except via TCP outcomes.

### Weaker representations that must die
- Authorization* public time names / aliases.
- Unit Indeterminate discard at bank-server if evidence is required for gate (carry at least through production path that classification uses).
- Dead classify_transport_fault.

### Critical dirty edge
- Bank NotifyDeath Declared aftermath installs in unit tests only; compiled application contracts carry no aftermath. Without binding Declared into the application op used by compare_and_commit, outbox never co-commits and e2e cannot traverse production.

### Downstream
- E2E in bank-server NotifyDeath family tests spawning real rail.

## Stage 2: Plan

1. Move time_source/time_basis → domain_computation/runtime_time/; rename all types to WorthQueryRuntime*; delete Authorization* names and aliases; update facade + bank-server.
2. Add provider_dispatch_outbox schema layout; wire into PrimaryGraphLayout.
3. Thread Option<InstalledExternalEffectContract> (or aftermath) onto compiled application operation contracts / bank NotifyDeath install; pass into register_application_attempt; push outbox create intent iff Declared.
4. Production dispatch module: after Committed progression, if outbox present, call rail over TCP with configured endpoint + fault script (test injects fault via runtime config / host-published rail target), classify_transport_fault, attach posture + with_external_dispatch_work (1/1/0 for new event; 0 for classify).
5. Host/bank map Indeterminate(_) keeping evidence available for inspection where needed; bank may retain unit status for UX but Query outcome must expose evidence.
6. bank-server e2e tests: spawn rail, configure Query dispatch to that addr, drive NotifyDeath (or declared-external op) through production commit+dispatch for each fault + succeed twin; DisburseEstate (no external) proves O3/O4 zero.
7. Verify: e2e, rail 7, aftermath 13, consumers 313/37/22, linecap, boundary, agent-context.

CDC: do NOT use Relational CDC for dispatch delivery; no second change stream.
