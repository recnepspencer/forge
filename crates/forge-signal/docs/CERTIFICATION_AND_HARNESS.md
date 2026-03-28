# Certification And Harness

This is specialist infrastructure, not the normal day-to-day path.

You reach for this when "looks fine to me" is not good enough.

## Main surfaces

- `SignalScenario`
- `SignalMutationBatch`
- `signal_bench(...)`
- `signal_parity_suite(...)`
- harness profile catalogs

## When to use it

Use the harness when:

- correctness needs structured scenario coverage
- serial versus parallel parity needs to be proven
- replay, diagnostics, and artifact capture need real assertions instead of a
  vibe check

## Position in the product

This surface is real and important, but it should not dominate the first
impression of the library.

Ordinary users should start with:

- [QUICKSTART.md](./QUICKSTART.md)
- [DAILY_WORKFLOWS.md](./DAILY_WORKFLOWS.md)
- [DIAGNOSTICS.md](./DIAGNOSTICS.md)
