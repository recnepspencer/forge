# forge-store-offline-verifier

Owns the independent read-only verifier that walks persisted bytes through root
manifests, segment manifests, page headers, frame headers, slot directories,
and free-space maps without constructing the live store runtime.

This crate is a trust boundary. It should be able to disagree with the live
backend and report that disagreement as evidence.
