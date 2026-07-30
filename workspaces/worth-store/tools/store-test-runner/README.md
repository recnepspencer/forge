# WORTH Store test runner

`store-test-runner` owns workspace test selection, causal mutation execution, and
the fresh-process courtrooms. Run commands from `workspaces/worth-store`.

## Running proof products

The runner turns the workspace test catalog into named products such as owner
checks, developer smoke, UI compile proofs, mutation campaigns, and
courtrooms. Use those products instead of assembling overlapping Cargo target
lists by hand.

Run the complete UI compile-proof product:

```text
cargo run -q -p store-test-runner -- ui
```

The UI product includes the runner's own authority compile tests. Windows does
not permit child Cargo to replace the executable that is currently running, so
the ordinary command automatically builds that self-compiling product in the
stable derived cache `target/store-test-runner-ui`. Other products continue to
reuse the ordinary workspace target directory.

Supply `--target-root` when an external build system owns the cache location:

```text
cargo run -q -p store-test-runner -- ui --target-root <TARGET_DIRECTORY>
```

An explicit target root always wins. Do not work around an access-denied build
by retrying it, copying the workspace, or retaining an extracted target tree.
The automatic UI directory is ordinary derived Cargo output: it is reusable
between runs, removable with workspace build output, and never certification
evidence.

## Courtroom C scheduling

Courtroom C has two independent reproducibility inputs:

- The workload seed chooses data and operation content.
- The schedule seed chooses only admissible execution decisions at existing
  harness gates.

Schedule perturbation may change worker start order, equivalent contender
identity, gate release order, and selection between independent ready work. It
must not add sleeps, timing jitter, a second scheduler, or reorder causally
dependent work.

The scheduled and manually dispatched CI workflow derives exactly 16 schedule
seeds from the checked-out revision. Lanes `0` through `15` cover every
combination in the four-decision binary vocabulary. A Courtroom C report records
the selected seed, executed decisions, trace digest, and the complete structured
lane manifest.

Run one revision-derived CI lane:

```text
cargo run -q -p store-test-runner --features physical-work-evidence -- courtrooms --courtroom c --mutant-report <MUTANT_REPORT> --report <COURTROOM_REPORT> --ci-schedule-lane 7
```

Replay a reported failure with the exact `schedule.replay` command from its
report. Explicit replay uses `--schedule-seed <U64>` and is mutually exclusive
with `--ci-schedule-lane`.

## Causal mutation corpus

Generate the complete bounded-residency evidence consumed by Courtroom C:

```text
cargo run -q -p store-test-runner -- mutants --mutation-scope bounded-residency --report <MUTANT_REPORT>
```

Every real production or certification bug fixed after adoption must add the
causal mutation that recreates it to the live mutation catalog. The catalog is a
growing regression corpus; do not replace an existing identity or weaken its
independent test predicate to keep the count fixed.

## Artifact accounting

Keep only the explicitly requested mutation and courtroom JSON reports. The
runner owns and removes its temporary worlds and pending publication files.
Do not retain copied target trees, archives, extracted bundles, or exploratory
temporary files as certification evidence.
