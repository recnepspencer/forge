#[derive(Clone, Copy)]
pub enum Enforcement {
    Rustc(CompilerFixture),
    BoundaryCheck(BoundaryFixture),
}

#[derive(Clone, Copy)]
pub enum CompilerFixture {
    Plain,
    GovernedAuthorityMismatch,
}

#[derive(Clone, Copy)]
pub enum BoundaryFixture {
    QueryAudience {
        band: &'static str,
        facade: &'static str,
        item: &'static str,
    },
    Entry {
        dependency: EntryDependency,
    },
}

#[derive(Clone, Copy)]
pub enum EntryDependency {
    Proof,
    Replay,
}

pub struct Specimen {
    pub path: &'static str,
    pub obligation: &'static str,
    pub enforcement: Enforcement,
    pub fragments: &'static [&'static str],
    pub facade_pairs: &'static [(&'static str, &'static str)],
}

pub const SPECIMENS: &[Specimen] = &[
    Specimen {
        path: "forged_authority.rs",
        obligation: "concrete authority values cannot be substituted",
        enforcement: Enforcement::Rustc(CompilerFixture::GovernedAuthorityMismatch),
        fragments: &[
            "error[E0308]",
            "expected `AuthorityWitness<EntryAdmission>`",
            "found `AuthorityWitness<ForgedAuthority>`",
            "expected `CapabilityWitness<EntryExecution>`",
            "found `CapabilityWitness<ForgedCapability>`",
            "expected `Proof<EntryAdmissionGranted, ...>`",
            "found `Proof<ForgedFact, ForgedAuthority>`",
        ],
        facade_pairs: &[],
    },
    Specimen {
        path: "deep_import_past_facade.rs",
        obligation: "private implementation modules stay unreachable",
        enforcement: Enforcement::Rustc(CompilerFixture::Plain),
        fragments: &["error[E0603]", "module `identity` is private"],
        facade_pairs: &[],
    },
    Specimen {
        path: "band_guard_wrong_band.rs",
        obligation: "macro expansion is confined to its declared band",
        enforcement: Enforcement::Rustc(CompilerFixture::Plain),
        fragments: &[
            "error[E0080]",
            "worth_proof::band_guard! rejected package",
            "legal package prefixes: worth-entry-, worthy-entry-",
        ],
        facade_pairs: &[],
    },
    Specimen {
        path: "decl_facade_in_schema_band.rs",
        obligation: "declaration facade is entry-band only",
        enforcement: Enforcement::BoundaryCheck(BoundaryFixture::QueryAudience {
            band: "schema",
            facade: "worth-query-decl",
            item: "CanonicalQueryArtifact",
        }),
        fragments: &["BC3002_WRONG_QUERY_AUDIENCE", "worth-query-decl"],
        facade_pairs: &[("worth-query-decl", "CanonicalQueryArtifact")],
    },
    Specimen {
        path: "host_facade_in_derived_band.rs",
        obligation: "host facade is entry-band only",
        enforcement: Enforcement::BoundaryCheck(BoundaryFixture::QueryAudience {
            band: "derived",
            facade: "worth-query-host",
            item: "runtime",
        }),
        fragments: &["BC3002_WRONG_QUERY_AUDIENCE", "worth-query-host"],
        facade_pairs: &[
            ("worth-query-host", "domain"),
            ("worth-query-host", "runtime"),
        ],
    },
    Specimen {
        path: "replay_facade_in_ordinary_band.rs",
        obligation: "replay facade is certification-band only",
        enforcement: Enforcement::BoundaryCheck(BoundaryFixture::Entry {
            dependency: EntryDependency::Replay,
        }),
        fragments: &["BC4001_ORDINARY_REPLAY_IMPORT", "cert-only"],
        facade_pairs: &[("worth-query-replay", "ScopedReplayBasis")],
    },
    Specimen {
        path: "generic_authority_bound_public_surface.rs",
        obligation: "governed surfaces require concrete platform authority",
        enforcement: Enforcement::BoundaryCheck(BoundaryFixture::Entry {
            dependency: EntryDependency::Proof,
        }),
        fragments: &["BC7001_AUTHORITY_SEALING", "concrete"],
        facade_pairs: &[],
    },
];
