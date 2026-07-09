use crate::error::{BridgeDeliveryError, BridgeRouteError};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeEvaluationTarget {
    planned_route: BridgePlannedRoute,
}

impl BridgeEvaluationTarget {
    pub(crate) fn new(planned_route: BridgePlannedRoute) -> Self {
        Self { planned_route }
    }

    /// Returns the route identity that produced this evaluation target.
    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.planned_route.route_identity()
    }

    /// Returns the planned route behind this target.
    pub fn planned_route(&self) -> &BridgePlannedRoute {
        &self.planned_route
    }

    pub(crate) fn into_planned_route(self) -> BridgePlannedRoute {
        self.planned_route
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoute {
    target: BridgeEvaluationTarget,
    result: BridgeRouteResult,
}

impl BridgeRoute {
    pub(crate) fn new(planned_route: BridgePlannedRoute, result: BridgeRouteResult) -> Self {
        Self {
            target: BridgeEvaluationTarget::new(planned_route),
            result,
        }
    }

    /// Returns the route identity.
    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.target.route_identity()
    }

    /// Returns the evaluation target produced by this route.
    pub fn target(&self) -> BridgeEvaluationTarget {
        self.target.clone()
    }

    /// Returns the delivery result produced by routing.
    pub fn result(&self) -> &BridgeRouteResult {
        &self.result
    }
}

#[derive(Debug)]
pub enum BridgeStandardRouteError {
    Route(BridgeRouteError),
    Delivery(BridgeDeliveryError),
}

impl std::fmt::Display for BridgeStandardRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Route(error) => write!(f, "{error}"),
            Self::Delivery(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for BridgeStandardRouteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Route(error) => Some(error),
            Self::Delivery(error) => Some(error),
        }
    }
}

impl From<BridgeRouteError> for BridgeStandardRouteError {
    fn from(value: BridgeRouteError) -> Self {
        Self::Route(value)
    }
}

impl From<BridgeDeliveryError> for BridgeStandardRouteError {
    fn from(value: BridgeDeliveryError) -> Self {
        Self::Delivery(value)
    }
}
