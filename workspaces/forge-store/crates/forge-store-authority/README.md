# forge-store-authority

Owns Roadmap 1 Milestone 1 concepts: canonical commit envelopes, version DAG
records, branch heads, ordered parent metadata, and authoritative artifact
identity.

This crate preserves the rule that `forge-relational` owns truth semantics and
Forge Store owns durable survival. It must not decide page layout, buffering,
WAL physics, or physical backend policy.
