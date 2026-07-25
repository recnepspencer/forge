use crate::WorthServerOperationFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerRouteAssemblyError {
    MissingCompatMutationRouteFamily {
        operation_name: String,
    },
    MissingCompatReadRouteFamily {
        operation_name: String,
    },
    MissingCompatQueryRouteFamily {
        operation_name: String,
    },
    DuplicateMethodPath {
        method: String,
        path: String,
    },
    OperationNameNotAdmitted {
        family: WorthServerOperationFamily,
        operation_name: String,
    },
}

impl WorthServerRouteAssemblyError {
    pub fn detail(&self) -> String {
        match self {
            Self::MissingCompatMutationRouteFamily { operation_name } => format!(
                "compatibility mutation route family must be enabled before assembling route `{operation_name}`"
            ),
            Self::MissingCompatReadRouteFamily { operation_name } => format!(
                "compatibility read route family must be enabled before assembling route `{operation_name}`"
            ),
            Self::MissingCompatQueryRouteFamily { operation_name } => format!(
                "compatibility query route family must be enabled before assembling structured read route `{operation_name}`"
            ),
            Self::DuplicateMethodPath { method, path } => {
                format!("route assembly rejected duplicate `{method} {path}`")
            }
            Self::OperationNameNotAdmitted {
                family,
                operation_name,
            } => format!(
                "route assembly rejected operation `{operation_name}` because it is not admitted for operation family `{}`",
                family.as_str()
            ),
        }
    }
}
