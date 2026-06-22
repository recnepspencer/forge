# forge-store-physical-integrity

Owns Roadmap 2 S.3: page/frame/chunk checksums, scrub, quarantine records,
typed corruption localization, and damaged-authority versus rebuildable-derived
reports.

Checksums prove physical integrity, not authenticity. Semantic decoders should
only see bytes after this boundary admits the physical frame.
