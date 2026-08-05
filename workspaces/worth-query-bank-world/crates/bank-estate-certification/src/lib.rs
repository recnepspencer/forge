//! External compilation surface for the complete public Bank estate facade.

#![forbid(unsafe_code)]

mod estate_commands;
mod estate_queries;

pub use estate_commands::{
    exercise_estate_commands, exercise_estate_lifecycle, EstateCommandInputs,
    EstateLifecycleInputs, EstateLifecyclePrincipals, EstateLifecycleProgressionOutcome,
};
pub use estate_queries::{
    exercise_emergency_estate_queries, exercise_ordinary_estate_queries, EmergencyEstateQueryInputs,
};
