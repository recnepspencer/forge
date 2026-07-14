# Diagnostics And Explanation Ontology

This folder documents the Milestone 6 diagnostics surface in
`worth-foundational`.

Use these docs when you need to answer questions like:

- How do I describe why a transition or artifact was accepted, denied, or only
  partially supported?
- How do I surface missing, redacted, deferred, or reconstructable evidence
  without faking certainty?
- How do I compare diagnostic bundles across independent producers?
- When do I stay descriptive, and when do I move into the stronger certified
  diagnostics lane?

Read the docs in this order if you are new to the surface:

1. [Diagnostic Primitives And Categories](./diagnostic-primitives-and-categories.md)
2. [Diagnostic Outcomes, Subjects, And Rows](./diagnostic-outcomes-subjects-and-rows.md)
3. [Diagnostic Materialization And Support Reports](./diagnostic-materialization-and-support-reports.md)
4. [Diagnostic Canonical Basis And Comparison](./diagnostic-canonical-basis-and-comparison.md)
5. [Certified Diagnostic Bundles And Attachments](./certified-diagnostic-bundles-and-attachments.md)
6. [Diagnostic Production Readiness](./diagnostic-production-readiness.md)

Capability order matters.

- Start with primitives and categories so code, scope, severity, artifact kind,
  delivery class, availability, denial class, breach class, and evidence
  posture all mean one thing.
- Build rows on top of those primitives so decision, failure, comparison,
  support, and provenance-ready evidence stay distinct.
- Plan and materialize reports and bundles only after row meaning is settled.
- Canonicalize and compare only after bundle meaning is settled.
- Use certified bundles only when you need a stronger proof-bearing claim.
- Use the readiness artifact when you need the exact machine-checkable closure
  contract for the milestone.

These docs are feature-first on purpose. They are not milestone notes, closeout
notes, or test-tour notes. If a capability shipped, it has a home here.
