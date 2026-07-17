# Router Glossary

These terms name distinct stages. Keeping them distinct prevents the classic
router bug where “the URL matched” quietly becomes “the user may see it.”

- **Authority**: the one source allowed to decide a fact. The browser owns the
  raw location; the router owns its normalized route interpretation.
- **Raw location**: a local URL received from a load, link, history event, or
  external host event before normalization.
- **Canonical URL**: the normalized path, ordered search entries, and decoded
  hash used for stable equality.
- **Route reference**: the typed handle returned by `signals.router.define(...)`.
- **Route location**: one route reference filled with concrete params, search,
  and hash values.
- **Projection**: structural matching. It answers “what route, layouts, outlets,
  and declared capabilities match this URL?”
- **Projected candidate**: that structural match before access checks run.
- **Admission**: the decision that answers whether a projected candidate may
  become route truth now.
- **Route outcome**: `admitted`, `redirect`, `notFound`, `forbidden`,
  `unavailable`, or `denied`.
- **Recovery**: a declared fallback from a non-admitted stale location to a
  nearest valid route. It is not a catch-all redirect.
- **Provenance**: the retained explanation of where a result came from—its
  prerequisite decisions, recovery trail, browser boundary, or restore source.
- **Browser ingress**: a typed envelope for a browser-owned location event.
- **Browser writeback**: a typed envelope describing a requested push, replace,
  or external escape. The host still performs the browser side effect.
- **History story**: retained router reports used to derive current, back,
  breadcrumb, inspection, and auditability views. It is not the browser's
  native history stack.
- **Visible continuity**: keeping the current route visible while a target is
  pending. It does not make the pending target admitted truth.
- **Restore boundary**: an exact runtime snapshot carried with a route-history
  entry. Without one, exact restore is unavailable.
- **Verification package**: stable digests that tie an artifact to its inputs.
  It is evidence about truth, not another owner of truth.

See [Router Overview](./index.md) for the full flow.
