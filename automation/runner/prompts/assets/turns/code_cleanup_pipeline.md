# Code Cleanup Pipeline

Use this prompt set when a milestone is primarily architectural cleanup,
proof-flow consolidation, directory topology repair, facade narrowing, or
domain-structure cleanup.

## Turn Template Map

Point a runner config at these binding references:

```json
{
  "boundary_review": { "assembly_id": "turns/code_cleanup_audit" },
  "plan": { "assembly_id": "turns/code_cleanup_plan" },
  "implement": { "assembly_id": "turns/code_cleanup_implement" },
  "review": { "assembly_id": "turns/code_cleanup_review" },
  "repair": { "assembly_id": "turns/code_cleanup_repair" },
  "test_review": { "assembly_id": "turns/code_cleanup_evidence_review" },
  "test_repair_plan": { "assembly_id": "turns/code_cleanup_evidence_plan" },
  "test_repair_implement": { "assembly_id": "turns/code_cleanup_evidence_implement" },
  "code_quality_review": { "assembly_id": "turns/code_cleanup_final_review" },
  "code_quality_repair": { "assembly_id": "turns/code_cleanup_final_repair" }
}
```

Use this contract:

```json
"contract_template": { "asset_id": "contracts/code_cleanup" }
```

## Adaptation Checklist

To adapt this pipeline to another worktree:

1. Set `project.name`, `project.cwd`, and `project.spec_file`.
2. Put the governing laws/docs and relevant crate/module entry files in
   `project.context_files`.
3. Turn the cleanup spec into phases whose `acceptance` items describe cleanup
   evidence, not feature acceptance.
4. In each phase, use `instructions` to name the desired structural direction.
5. In each phase, use `qa_focus` to name the most likely cleanup failure modes
   for that subsystem.
6. Keep `boundary_review` enabled so each phase starts with structural audit
   before planning.

## Good Phase Acceptance Evidence

Examples:

- final directory skeleton documented and implemented
- public facade diff shows narrower lifecycle-shaped exports
- proof flow reads as named transition steps
- overloaded functions are decomposed into orchestration plus semantic steps
- certification/test support authority is visibly separate from production law
- construction boundary has compile-fail proof when sealing changed
- runtime hostile lane remains covered when behavior was touched
- focused `cargo check` or module tests pass for touched crates

