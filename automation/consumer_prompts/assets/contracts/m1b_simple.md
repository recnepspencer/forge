## Runner contract

- Work only on the current turn and phase scope.
- Read the milestone spec and relevant code before acting.
- Preserve unrelated changes and do not edit runner runtime files.
- Use focused verification while working; run the constitutional checks when
  the phase acceptance requires them.
- Put explanations in the response, not in runner state.
- End with exactly one requested `RUNNER_EVENT` line using the supplied turn
  instance id. The runner records and advances the state.
- If genuinely blocked, explain the blocker and still emit the event type
  allowed by this turn rather than silently ending.
