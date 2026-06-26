# forge-store-physical-backend

Owns the narrow S.1 physical backend facade: append, read, scan, locate,
publish root manifest, and reopen from persisted bytes.

The backend facade deals in framed bytes and physical references. It does not
own semantic commit legality, artifact meaning, or legacy compatibility policy.
