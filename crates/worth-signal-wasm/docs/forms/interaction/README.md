# Interaction And Host Facts

Interaction reports describe what the user and host have actually done:
focused, blurred, touched, visited, gone offline, restored credentials, or
changed visibility. These facts can affect messages and readiness without
becoming source or draft mutations.

Host facts are explicit declaration or ingress. The controller does not read
browser globals. If a submit action requires `online` and `credentials`, your
host must report those capabilities; otherwise readiness explains the missing
requirement.

Read next:

- [Focus, Touch, And Visited State](./focus-touch-and-visited-state.md)
- [Input Capabilities](./input-capabilities.md)
- [Host Facts](./host-facts.md)
- [Offline And Host Blockers](./offline-and-host-blockers.md)
- [Inputs And Controls](../inputs/README.md)
