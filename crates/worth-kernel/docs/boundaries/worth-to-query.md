<!-- worth-doc
crate: worth-kernel
kind: boundary
id: worth-to-query
query_integration_required: false
query_proof_required: false
touches_query: true
-->

# Worth To Query

## Boundary

This boundary explains how Worth runtime-backed work enters Query and how
Worth proves that it consumed Query honestly.

## Allowed Upstream Inputs

- kernel workflow and workload declarations
- admitted feature inputs that still need runtime execution
- kernel-side proof and closeout requests about Query consumption

## Required Downstream Outputs

- Query runtime execution through `ForgeQueryWorkspace` and its public families
- Query support posture and admission facts
- Query-owned proof posture for anti-bypass, evidence identity, support
  pinning, and reference-consumer adoption

## Stable Entry Points

- Query runtime entry: `ForgeQueryWorkspace`
- kernel Query-proof surface: `worth_kernel::query_adoption`

## Query Usage

Worth must treat Query as the ordinary runtime layer, not as a helper:

- runtime-backed work enters through Query public surfaces
- support posture comes from Query admission and support rows
- Query-owned proof lanes stay Query-owned:
  - evidence-report identity
  - hard-prohibition registry and boundary audit
  - support snapshot and support pinning
  - reference-consumer adoption closure

## Forbidden Bypasses

- direct lower-runtime plumbing passed off as ordinary kernel DX
- Worth-local grep audits where Query owns the hard-prohibition lane
- Worth-local support-family bookkeeping where Query owns support pinning
- Worth-local Query proof digests where Query owns evidence-report identity

## Binding Artifacts Or Receipts

The important artifacts at this boundary are:

- Query public support and admission reports
- Query-owned evidence identities and adoption reports
- kernel-side Query-adoption closeout and honesty reports

## Inspection And Debugging

Inspect this boundary when you need to answer:

- did Worth use an admitted Query family?
- did Worth keep Query proof posture on the Query-owned lane?
- did a support or anti-bypass proof regress?

## Anti-Patterns

- documenting Query as an implementation detail under kernel ownership
- treating support from autocomplete as real support
- proving Query consumption with local synthetic scaffolding after Query 9.8

## Related Docs

- [Kernel Overview](../foundations/kernel-overview.md)
- [Construction Results And Diagnostics](../features/construction-results-and-diagnostics.md)
