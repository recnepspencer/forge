# Certification And Harness

This is deeper test and proof infrastructure, not the normal day-to-day path.

You reach for this when "looks fine to me" is not good enough.

## Main Surfaces

- `SignalScenario`
- `SignalMutationBatch`
- `signal_bench(...)`
- `signal_parity_suite(...)`
- harness profile catalogs

## When to use it

Use the harness when:

- correctness needs structured scenario coverage
- serial versus parallel parity needs to be proven
- replay, diagnostics, and result capture need real assertions instead of a vibe
  check
