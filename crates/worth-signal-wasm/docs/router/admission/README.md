# Route Admission

Matching says a route fits. Admission says it may become route truth now. The
router consumes facts supplied at the boundary and returns one typed outcome:
`admitted`, `redirect`, `notFound`, `forbidden`, `unavailable`, or `denied`.

Start with [Admit](./admit.md), then use:

- [Prerequisites](./prerequisites.md) for declared decisions
- [Admission Facts](./admission_facts.md) for explicit inputs
- [Route Outcomes](./route_outcomes.md) for result handling
- [Access Policy](./access_policy.md) for typed host/resource/graph sources
- [Forbidden, Unavailable, And Denied](./forbidden_unavailable_denied.md) when
  failure meaning matters

An admission callback may decide. It should not fetch, navigate, render, or
silently read an ambient auth store.
