# worth-store-buffer-pool

Owns Roadmap 2 S.2: bounded resident memory, page leases, pin/unpin, dirty
tracking, eviction, read-ahead, write-behind, and allocation envelopes.

This crate may know physical page identities. It must not heap-load the whole
store or reconstruct semantic domain objects.
