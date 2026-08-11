//! Exact association of configured witness rows with definitions and Cargo worlds.

use super::contract::{config_diagnostic, display_key, WorldInventory};
use super::witness_source::WitnessSignature;
use crate::config::{PublicValueReachabilityContract, PublicValueWitness};
use crate::diagnostics::Diagnostic;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Inputs<'a> {
    pub(super) contract: &'a PublicValueReachabilityContract,
    pub(super) worlds: &'a [WorldInventory<'a>],
    pub(super) exports: &'a BTreeSet<String>,
    pub(super) witnesses: &'a BTreeMap<String, &'a PublicValueWitness>,
    pub(super) signatures: &'a BTreeMap<String, WitnessSignature>,
}

pub(super) fn validate(inputs: Inputs<'_>, diagnostics: &mut Vec<Diagnostic>) {
    let configured_worlds = inputs
        .contract
        .worlds
        .iter()
        .map(|world| &world.name)
        .collect::<BTreeSet<_>>();
    let mut validator = Validator {
        inputs,
        configured_worlds,
        diagnostics,
        used_functions: BTreeSet::new(),
    };
    validator.run();
}

struct Validator<'a, 'diagnostics> {
    inputs: Inputs<'a>,
    configured_worlds: BTreeSet<&'a String>,
    diagnostics: &'diagnostics mut Vec<Diagnostic>,
    used_functions: BTreeSet<String>,
}

impl Validator<'_, '_> {
    fn run(&mut self) {
        for (path, witness) in self.inputs.witnesses {
            self.validate_row(path, witness);
        }
        for function in self.inputs.signatures.keys() {
            if !self.used_functions.contains(function.as_str()) {
                self.diagnostics.push(config_diagnostic(
                    self.inputs.contract,
                    format!(
                        "witness function `{function}` is stale because no definition maps to it"
                    ),
                ));
            }
        }
    }

    fn validate_row(&mut self, path: &str, witness: &'_ PublicValueWitness) {
        if !self.inputs.exports.contains(path) {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!("witness `{path}` is stale because it is not an exported public value"),
            ));
        }
        if witness.worlds.is_empty() {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!("witness `{path}` must name at least one configured Cargo world"),
            ));
        }
        let mut row_worlds = BTreeSet::new();
        for world in &witness.worlds {
            self.validate_world(path, witness, world, &mut row_worlds);
        }
        self.validate_signature(path, witness);
    }

    fn validate_world(
        &mut self,
        path: &str,
        witness: &PublicValueWitness,
        world_name: &String,
        row_worlds: &mut BTreeSet<String>,
    ) {
        if !row_worlds.insert(world_name.clone()) || !self.configured_worlds.contains(world_name) {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!("witness `{path}` names duplicate or unknown Cargo world `{world_name}"),
            ));
        }
        let Some(world) = self
            .inputs
            .worlds
            .iter()
            .find(|world| world.target == world_name)
        else {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!("witness `{path}` is stale in Cargo world `{world_name}"),
            ));
            return;
        };
        if !world
            .exports
            .iter()
            .any(|export| display_key(&export.key) == path)
        {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!("witness `{path}` is stale in Cargo world `{world_name}"),
            ));
        }
        let resolved = super::super::type_alias_reachability::resolve_public_type_key(
            world.graph,
            &witness.public_type_path,
        );
        if resolved.as_ref().map(display_key).as_deref() != Some(path) {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!(
                    "witness `{path}` public type `{}` does not resolve to its exact definition in Cargo world `{world_name}`",
                    witness.public_type_path
                ),
            ));
        }
    }

    fn validate_signature(&mut self, path: &str, witness: &'_ PublicValueWitness) {
        let Some(signature) = self.inputs.signatures.get(&witness.function) else {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!(
                    "witness `{path}` names missing function `{}`",
                    witness.function
                ),
            ));
            return;
        };
        self.used_functions.insert(witness.function.clone());
        if signature.posture != witness.posture {
            self.diagnostics.push(config_diagnostic(
                self.inputs.contract,
                format!(
                    "witness `{path}` function `{}` has the wrong posture",
                    witness.function
                ),
            ));
        }
    }
}
