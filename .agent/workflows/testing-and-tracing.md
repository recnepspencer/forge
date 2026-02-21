---
description: Comprehensive guide for writing, running, and inspecting Forge kernel tests with tracing
---

---
description: How to run tests, read failures, and debug boolean pipeline issues
---
# Testing & Debugging Workflow
// turbo-all
## Running Tests
**Always release mode.** One command:
```bash
FORGE_LOG=compact cargo test --release -p <whatever you want to test> --nocapture 2>&1 | tail -40
```

when you need more context:
# Run test, capture everything to temp
FORGE_LOG=full cargo test --release -p forge-kernel --lib coplanar_grid_4x4x4 -- --nocapture 2>&1 > /tmp/forge_debug.txt
# Now grep for what you need without flooding context
grep "postprocess\|coplanar\|merge" /tmp/forge_debug.txt
grep "Face#68" /tmp/forge_debug.txt
grep "escalated\|near_boundary" /tmp/forge_debug.txt


this is the output structure:
id:              DecisionId (unique integer)
tier:            Deterministic | Resolved | NearBoundary | PolicyApplied | Escalated
kind:            Exact | PolicyApplied{policy, default_used} | NearBoundary{threshold} | Ambiguous{fallback} | Forced{reason}
margin:          f64 (distance to threshold — lower = more fragile)
entity_scope:    EntityRef (e.g. Face#68, HalfEdge#294, Vertex#12)
span_id:         SpanId (which pipeline phase: split, classify, select, assemble, postprocess)
topology_delta:  entities_created[], entities_deleted[]
context:         one of:
                   Classification { point: [f64;3], result: "Inside"/"Outside"/... }
                   Coincidence { entity_a, entity_b }
                   Tolerance { measured: f64, threshold: f64 }
                   Degeneracy { description: String }
                   PrecisionEscalation { resolved_at, disagreement_magnitude, target_triple }
TraceEvent — the log is a flat list of these:

Decision(TracedDecision)           — a kernel decision
StartSpan { id, parent_id, name } — phase boundary (e.g. "split", "classify")
EndSpan { id, duration_micros }   — phase completed with timing
Print format (what you actually grep):

[decision-{id}] [{tier}] {kind} margin={margin} span-{span_id} entity={entity_scope} | {context}
So you can grep by:

entity=Face#68 — all decisions touching a face
entity=HalfEdge# — all halfedge decisions
span-3 — everything in the classify phase
escalated\|near-boundary — just the risky ones
Classification — all in/out decisions
Degeneracy — all degenerate geometry detections
Stitched — all twin matching decisions
margin=0.00e0 — exact-zero margin decisions

but grep it, because a full log will likely be 10's of thousands of lines. 