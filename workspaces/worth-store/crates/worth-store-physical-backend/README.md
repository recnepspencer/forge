# worth-store-physical-backend

Owns the narrow S.1 physical backend facade: append, read, scan, locate,
publish root manifest, and reopen from persisted bytes.

The backend facade deals in framed bytes and physical references. It does not
own semantic commit legality, artifact meaning, or legacy compatibility policy.

For durability, the backend owns admitted filesystem capability and exact media
mechanics. `PhysicalDurabilityAdmissionBasis` can be issued only from one
qualified media generation with matching file-sync, directory-sync, and
durable-rename claims. Backend receipts describe completed mechanisms; they do
not know mutation grouping, pageLSN policy, checkpoint policy, current-root
authority, or final Store acknowledgment.

Ordinary execution remains behind the Store's C.4 media owner. There is no
public backend runtime that callers may compose into a second durability lane.
