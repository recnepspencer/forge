use std::fmt;

#[derive(Debug)]
pub enum ProofProductUnavailable {
    ExplicitOwnerRequired,
    UnknownOwner(String),
    OwnerBoundaryViolation {
        owner: String,
        reached_target: String,
    },
    NamedProfileRequired(String),
    ExplicitSeedRequired(String),
    NamedBackendRequired(String),
    UnknownBackend {
        product: String,
        backend: String,
    },
    UnsupportedRequestOption {
        product: String,
        option: String,
    },
    UnknownProofProfile {
        product: String,
        profile: String,
    },
    MissingRequiredProofProduct(String),
    ScenarioTopology(String),
    UnsupportedHost {
        product: String,
        required: String,
        actual: String,
    },
    NoReachableProof {
        product: String,
    },
    RepositoryObservation(String),
}

impl fmt::Display for ProofProductUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExplicitOwnerRequired => write!(formatter, "store-owner requires -p <package>"),
            Self::UnknownOwner(owner) => write!(formatter, "unknown owner package: {owner}"),
            Self::OwnerBoundaryViolation {
                owner,
                reached_target,
            } => write!(
                formatter,
                "store-owner:{owner} reached non-owner target {reached_target}"
            ),
            Self::NamedProfileRequired(product) => {
                write!(formatter, "{product} requires --profile <proof-profile>")
            }
            Self::ExplicitSeedRequired(product) => {
                write!(formatter, "{product} requires --seed <u64>")
            }
            Self::NamedBackendRequired(product) => {
                write!(formatter, "{product} requires --backend <backend-profile>")
            }
            Self::UnknownBackend { product, backend } => write!(
                formatter,
                "{product} does not recognize backend {backend:?}"
            ),
            Self::UnsupportedRequestOption { product, option } => {
                write!(formatter, "{product} does not admit option {option}")
            }
            Self::UnknownProofProfile { product, profile } => write!(
                formatter,
                "{product} does not recognize proof profile {profile:?}"
            ),
            Self::MissingRequiredProofProduct(product) => write!(
                formatter,
                "required proof product has no reachable proof: {product}"
            ),
            Self::ScenarioTopology(reason) => {
                write!(formatter, "scenario topology is invalid: {reason}")
            }
            Self::UnsupportedHost {
                product,
                required,
                actual,
            } => write!(
                formatter,
                "{product} requires {required}; current host is {actual}"
            ),
            Self::NoReachableProof { product } => {
                write!(formatter, "{product} selects no reachable proof")
            }
            Self::RepositoryObservation(reason) => formatter.write_str(reason),
        }
    }
}

impl std::error::Error for ProofProductUnavailable {}
