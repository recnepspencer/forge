# forge-store-test-support

Owns shared fixtures, builders, fake byte stores, and assertion helpers used by
multiple Forge Store rebuild crates.

Test support follows the same boundary rules as production. Helpers live here
only when more than one crate needs the same testing concept for the same
reason.
