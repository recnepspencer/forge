# Development Pipeline

Use this prompt set when a milestone is primarily implementation work:
building a real runtime lane, widening admitted support, closing proof
boundaries, and hardening tests and structure as part of ordinary development.

## Turn Template Map

Point a runner config at these templates:

```json
{
  "boundary_review": "templates/boundary_review.md",
  "plan": "templates/plan.md",
  "implement": "templates/implement.md",
  "review": "templates/review_test_hardening.md",
  "repair": "templates/repair.md",
  "test_review": "templates/test_review.md",
  "test_repair_plan": "templates/test_repair_plan.md",
  "test_repair_implement": "templates/test_repair_implement.md",
  "code_quality_review": "templates/code_quality_review.md",
  "code_quality_repair": "templates/code_quality_repair.md"
}
```

Use this contract:

```json
"contract_template": "templates/_contract_test_hardening.md"
```

## Adaptation Checklist

1. Set `project.name`, `project.cwd`, and `project.spec_file`.
2. Put the governing laws/docs and relevant API entry files in
   `project.context_files`.
3. Use milestone phases whose `acceptance` items describe shipped capability and
   proof, not only cleanup evidence.
4. Use `runner_control.boundary_review_start_phase` when later phases should
   begin with the architectural boundary brief.
5. Keep `code_quality_review` and `code_quality_repair` enabled so structural
   QA remains a hard gate rather than advisory commentary.
