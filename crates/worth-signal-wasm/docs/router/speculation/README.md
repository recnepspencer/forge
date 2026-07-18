# Speculative Navigation

Speculation evaluates a projected route on a real history branch before it can
become visible route truth. It is useful when navigation includes reversible
graph work, conflict-sensitive edits, or a commit preview.

Read in this order:

1. [Speculative Branch Plans](./speculative_branch_plans.md)
2. [Speculative Sessions](./speculative_sessions.md)
3. [Visible Projection](./visible_projection.md)
4. [Dirty Exit](./dirty_exit.md)
5. [Commit Preview](./commit_preview.md)
6. [Discard And Keep Pending](./discard_and_keep_pending.md)
7. [Speculative Outcomes](./speculative_outcomes.md)

Ordinary navigation does not need this machinery. When you do use it, pass a
real branch-history facade; route ids and copied objects are not branch proof.
