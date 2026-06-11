use super::case::OpenPlanarPostureCase;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenPlanarPostureError {
    MissingOpenTopology,
    TopologyWasNotOpen,
    MissingUnsupportedSurfaceSupport,
    SurfaceSupportWasAdmitted,
    MissingCleanFailBoundary,
    CleanFailDidNotRepresentOpenOrUnbounded,
    CleanFailAttemptedBoundedConversion,
    CleanFailChangedTruth,
    CleanFailDidNotConsumeOpenTopology,
    MismatchedOpenInputKind,
    MismatchedDiagnosticSubject,
    MissingTransformPosture,
    MissingUserResponse,
    UserResponseDidNotMatchOutcome,
    UserResponseDidNotConsumePosture,
    BoundedSurrogateAttempted,
    MismatchedOutcomeCase {
        expected: OpenPlanarPostureCase,
        actual: OpenPlanarPostureCase,
    },
}

impl OpenPlanarPostureError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingOpenTopology => {
                "open planar posture requires a real open topology receipt".to_string()
            }
            Self::TopologyWasNotOpen => {
                "open planar posture cannot be proven by closed or bounded topology".to_string()
            }
            Self::MissingUnsupportedSurfaceSupport => {
                "open planar posture requires typed unsupported surface support evidence"
                    .to_string()
            }
            Self::SurfaceSupportWasAdmitted => {
                "open planar posture must not enter the admitted bounded surface path".to_string()
            }
            Self::MissingCleanFailBoundary => {
                "open planar posture requires a clean-fail boundary receipt".to_string()
            }
            Self::CleanFailDidNotRepresentOpenOrUnbounded => {
                "clean-fail boundary must classify this input as open or unbounded".to_string()
            }
            Self::CleanFailAttemptedBoundedConversion => {
                "open or unbounded planar input cannot be clipped or converted to bounded geometry"
                    .to_string()
            }
            Self::CleanFailChangedTruth => {
                "open planar posture evidence must not change planar truth".to_string()
            }
            Self::CleanFailDidNotConsumeOpenTopology => {
                "clean-fail boundary must consume the open topology receipt identity".to_string()
            }
            Self::MismatchedOpenInputKind => {
                "open posture branch must match the clean-fail open input kind".to_string()
            }
            Self::MismatchedDiagnosticSubject => {
                "open posture diagnostic locality must match the typed outcome branch".to_string()
            }
            Self::MissingTransformPosture => {
                "open planar posture requires movement and rotation posture evidence".to_string()
            }
            Self::MissingUserResponse => {
                "open planar posture requires a user response receipt".to_string()
            }
            Self::UserResponseDidNotMatchOutcome => {
                "open planar posture response must match the typed outcome branch".to_string()
            }
            Self::UserResponseDidNotConsumePosture => {
                "open planar posture response must consume the posture receipt identity".to_string()
            }
            Self::BoundedSurrogateAttempted => {
                "finite bounded surrogate geometry cannot satisfy an open or unbounded workload"
                    .to_string()
            }
            Self::MismatchedOutcomeCase { expected, actual } => format!(
                "open posture expected {} but workload declared {}",
                expected.human_name(),
                actual.human_name()
            ),
        }
    }
}
