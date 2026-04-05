---
name: test-auditor
description: Audit milestone tests for adversarial strength, coverage honesty, and spec alignment, then drive iteration until no meaningful gaps remain. Use when added or modified tests need skepticism stronger than ordinary implementation QA.
---

# Test Auditor

Use this skill after the milestone implementation is in place and the main QA surface is stable enough for a focused test review.

The implementation spec is binding. The governing documents are binding:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

Review all tests added or modified for the milestone, and nearby tests only when they are part of the same behavior surface or expose a real gap.

## Audit prompt

```text
Review the tests for this milestone with extreme skepticism.

Assume shallow tests are dangerous because they create false confidence. Evaluate whether the added or modified tests are genuinely adversarial, whether they certify meaningful architectural and domain properties, and whether they leave real gaps against the spec.

Reject decorative coverage. Reject trivial happy-path proof. Reject tests that merely mirror the implementation. Prefer tests that would catch structural dishonesty, semantic drift, and hostile edge conditions.

Report findings first. Then strengthen the test surface until no meaningful gaps remain within scope.
```
