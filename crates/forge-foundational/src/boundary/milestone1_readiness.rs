#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1PublicApiSurface {
    name: &'static str,
    owns: &'static str,
    adoption_use: &'static str,
}

impl Milestone1PublicApiSurface {
    pub const fn new(name: &'static str, owns: &'static str, adoption_use: &'static str) -> Self {
        Self {
            name,
            owns,
            adoption_use,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn owns(&self) -> &'static str {
        self.owns
    }

    pub const fn adoption_use(&self) -> &'static str {
        self.adoption_use
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1CompatibilityDebt {
    name: &'static str,
    boundary: &'static str,
    exit_condition: &'static str,
}

impl Milestone1CompatibilityDebt {
    pub const fn new(
        name: &'static str,
        boundary: &'static str,
        exit_condition: &'static str,
    ) -> Self {
        Self {
            name,
            boundary,
            exit_condition,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn boundary(&self) -> &'static str {
        self.boundary
    }

    pub const fn exit_condition(&self) -> &'static str {
        self.exit_condition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1ProofSeed {
    name: &'static str,
    certifies: &'static str,
    evidence: &'static str,
}

impl Milestone1ProofSeed {
    pub const fn new(name: &'static str, certifies: &'static str, evidence: &'static str) -> Self {
        Self {
            name,
            certifies,
            evidence,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn certifies(&self) -> &'static str {
        self.certifies
    }

    pub const fn evidence(&self) -> &'static str {
        self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1MigrationReadinessReport {
    public_api: &'static [Milestone1PublicApiSurface],
    compatibility_debt: &'static [Milestone1CompatibilityDebt],
    proof_seeds: &'static [Milestone1ProofSeed],
}

impl Milestone1MigrationReadinessReport {
    pub const fn public_api(&self) -> &'static [Milestone1PublicApiSurface] {
        self.public_api
    }

    pub const fn compatibility_debt(&self) -> &'static [Milestone1CompatibilityDebt] {
        self.compatibility_debt
    }

    pub const fn proof_seeds(&self) -> &'static [Milestone1ProofSeed] {
        self.proof_seeds
    }
}

pub fn milestone1_migration_readiness_report() -> Milestone1MigrationReadinessReport {
    Milestone1MigrationReadinessReport {
        public_api: milestone1_public_api_inventory(),
        compatibility_debt: milestone1_compatibility_debt_inventory(),
        proof_seeds: milestone1_proof_seed_inventory(),
    }
}

pub const fn milestone1_public_api_inventory() -> &'static [Milestone1PublicApiSurface] {
    &MILESTONE1_PUBLIC_API
}

pub const fn milestone1_compatibility_debt_inventory() -> &'static [Milestone1CompatibilityDebt] {
    &MILESTONE1_COMPATIBILITY_DEBT
}

pub const fn milestone1_proof_seed_inventory() -> &'static [Milestone1ProofSeed] {
    &MILESTONE1_PROOF_SEEDS
}

const MILESTONE1_PUBLIC_API: [Milestone1PublicApiSurface; 8] = [
    Milestone1PublicApiSurface::new(
        "values",
        "canonical Aspec-native scalar and reference value vocabulary",
        "materialize crate-local values at explicit boundaries",
    ),
    Milestone1PublicApiSurface::new(
        "aspect_contracts",
        "aspect shape, mask, absence, equivalence, and evolution law",
        "declare interpretation law before values enter authority",
    ),
    Milestone1PublicApiSurface::new(
        "authoritative_state",
        "contract-admitted record aspect state",
        "exchange authoritative aspect truth without producer-private layout",
    ),
    Milestone1PublicApiSurface::new(
        "authoritative_patches",
        "whole-aspect and field-level set/clear semantics",
        "exchange aspect-state changes without JSON merge folklore",
    ),
    Milestone1PublicApiSurface::new(
        "identity_categories",
        "typed boundary ids, handles, basis ids, epochs, and digest ids",
        "prevent representation-equal ids from becoming semantically interchangeable",
    ),
    Milestone1PublicApiSurface::new(
        "locators",
        "typed aspect, field, mask, artifact, source, and mismatch loci",
        "let diagnostics and support artifacts point at canonical boundary meaning",
    ),
    Milestone1PublicApiSurface::new(
        "compatibility_bridges",
        "explicit JSON-originated lowering into canonical aspect-native meaning",
        "migrate legacy payload boundaries without making JSON authoritative",
    ),
    Milestone1PublicApiSurface::new(
        "digest_preparation",
        "proof-bearing canonical ordering and equality basis",
        "feed Milestone 2 digest algorithms without revisiting Milestone 1 semantics",
    ),
];

const MILESTONE1_COMPATIBILITY_DEBT: [Milestone1CompatibilityDebt; 1] =
    [Milestone1CompatibilityDebt::new(
        "json_compatibility_lowering",
        "serde_json::Value may enter only through JsonCompatibilityAspectInput",
        "adopting crates replace legacy JSON payload authority with native aspect-state construction",
    )];

const MILESTONE1_PROOF_SEEDS: [Milestone1ProofSeed; 8] = [
    Milestone1ProofSeed::new(
        "contract_validation",
        "raw values cannot become admitted values without aspect-contract law",
        "certification/aspects/contracts and ui/contract_validation",
    ),
    Milestone1ProofSeed::new(
        "evolution_classification",
        "old/new contract interpretation carries a proof-bearing classified verdict",
        "certification/aspects/evolution and ui/aspect_evolution",
    ),
    Milestone1ProofSeed::new(
        "authoritative_state_admission",
        "raw values cannot enter authoritative record aspect state",
        "certification/aspects/state and ui/authoritative_state",
    ),
    Milestone1ProofSeed::new(
        "patch_admissibility",
        "patches preserve set/clear distinction and struct-field law",
        "certification/aspects/patches and ui/authoritative_patches",
    ),
    Milestone1ProofSeed::new(
        "mask_mode_typing",
        "projection, mutation, and diagnostic masks are not interchangeable",
        "certification/aspects/masks and ui/mask_admissibility",
    ),
    Milestone1ProofSeed::new(
        "identity_and_locator_categories",
        "representation-equal ids and locators keep distinct meanings",
        "certification/identities, certification/locators, and ui/identity_categories",
    ),
    Milestone1ProofSeed::new(
        "compatibility_lowering",
        "JSON-originated input lowers or fails without becoming authority",
        "certification/compatibility",
    ),
    Milestone1ProofSeed::new(
        "digest_preparation_readiness",
        "state, patch, contract, and mask bases require readiness proof",
        "certification/canonicalization/digest_preparation and ui/digest_preparation",
    ),
];
