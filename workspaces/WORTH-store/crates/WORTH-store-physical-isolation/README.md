# worth-store-physical-isolation

Owns Roadmap 2 S.5: physical byte stability while compaction, checkpointing,
reclaim, tier movement, and blob migration interleave with foreground reads.

This crate answers whether bytes remain stable for a physical read plan. It
does not implement semantic MVCC.
