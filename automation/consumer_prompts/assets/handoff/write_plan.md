## Cross-provider handoff (oversight -> build)

The implementer runs on a different provider session and cannot see this chat.
After posting the plan above, ALSO write the full plan to the handoff artifact
so the implementer can load it:

- Path: `.runner-handoff/phase-{phase.id}/plan.md`
- Create the `.runner-handoff/phase-{phase.id}/` directory if it does not exist.
- Write the complete plan — directory skeleton, DX target Rust block,
  implementation sequence, and per-step proofs — not a summary.
- This artifact is the contract of record for the implement turn.
