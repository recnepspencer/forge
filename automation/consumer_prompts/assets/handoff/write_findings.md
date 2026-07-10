## Cross-provider handoff (oversight -> build)

The repair turn runs on a different provider session and cannot see this chat.
If you are failing the phase, WRITE your detailed findings to the handoff
artifact so the repair agent has the real, load-bearing detail:

- Path: `.runner-handoff/phase-{phase.id}/findings.md`
- Write the full blocking finding set with file/line specifics and root cause,
  not just the projection summary.
- Overwrite any prior contents; the latest review wins.
