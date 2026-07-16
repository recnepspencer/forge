use crate::ProtocolFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedFrontierIdentity {
    Durability,
    Visibility,
    Reachability,
    Quarantine,
    Admission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossProtocolLocalization {
    first_protocol: ProtocolFamily,
    second_protocol: ProtocolFamily,
    frontier: SharedFrontierIdentity,
    illegal_edge: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossProtocolLocalizationDenial {
    SameProtocolFamily,
    SharedFrontierIsNotALocalProtocol,
    IllegalEdgeRequired,
}

impl CrossProtocolLocalization {
    pub fn diagnostic(
        first_protocol: ProtocolFamily,
        second_protocol: ProtocolFamily,
        frontier: SharedFrontierIdentity,
        illegal_edge: impl Into<String>,
    ) -> Result<Self, CrossProtocolLocalizationDenial> {
        if first_protocol == second_protocol {
            return Err(CrossProtocolLocalizationDenial::SameProtocolFamily);
        }
        if matches!(first_protocol, ProtocolFamily::SharedFrontiers)
            || matches!(second_protocol, ProtocolFamily::SharedFrontiers)
        {
            return Err(CrossProtocolLocalizationDenial::SharedFrontierIsNotALocalProtocol);
        }
        let illegal_edge = illegal_edge.into();
        if illegal_edge.is_empty() {
            return Err(CrossProtocolLocalizationDenial::IllegalEdgeRequired);
        }
        Ok(Self {
            first_protocol,
            second_protocol,
            frontier,
            illegal_edge,
        })
    }

    pub const fn first_protocol(&self) -> ProtocolFamily {
        self.first_protocol
    }

    pub const fn second_protocol(&self) -> ProtocolFamily {
        self.second_protocol
    }

    pub const fn frontier(&self) -> SharedFrontierIdentity {
        self.frontier
    }

    pub fn illegal_edge(&self) -> &str {
        &self.illegal_edge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localization_requires_two_local_protocol_families() {
        assert_eq!(
            CrossProtocolLocalization::diagnostic(
                ProtocolFamily::LeaseReclaim,
                ProtocolFamily::LeaseReclaim,
                SharedFrontierIdentity::Reachability,
                "reclaim -> reuse",
            ),
            Err(CrossProtocolLocalizationDenial::SameProtocolFamily)
        );
        assert_eq!(
            CrossProtocolLocalization::diagnostic(
                ProtocolFamily::SharedFrontiers,
                ProtocolFamily::LeaseReclaim,
                SharedFrontierIdentity::Reachability,
                "reclaim -> reuse",
            ),
            Err(CrossProtocolLocalizationDenial::SharedFrontierIsNotALocalProtocol)
        );
    }
}
