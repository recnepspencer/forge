use crate::application_ability::{
    WorthQueryAbilityInstallationDenial, WorthQueryAbilityInstallationDenialKind,
    WorthQueryInstalledAbility,
};

use super::super::super::WorthQueryInstalledPackageIndex;

impl WorthQueryInstalledPackageIndex {
    pub fn validate_ability<Schema, Ability, Scope>(
        &self,
        installed: &WorthQueryInstalledAbility<Schema, Ability, Scope>,
    ) -> Result<(), WorthQueryAbilityInstallationDenial> {
        let identity = installed.binding_identity();
        if identity.runtime_ordinal() != self.runtime_ordinal() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::ForeignRuntime,
                installed,
            ));
        }
        if identity.generation() != self.generation().ordinal() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::StaleGeneration,
                installed,
            ));
        }
        let schema = self
            .application_schemas
            .get(&(
                installed.owner().to_string(),
                installed.schema_name().to_string(),
            ))
            .ok_or_else(|| {
                ability_denial(
                    WorthQueryAbilityInstallationDenialKind::SchemaMeaningChanged,
                    installed,
                )
            })?;
        let package = self.domain(installed.owner()).map_err(|_| {
            ability_denial(
                WorthQueryAbilityInstallationDenialKind::PackageIdentityChanged,
                installed,
            )
        })?;
        if package.package_identity().digest() != identity.package_identity() {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::PackageIdentityChanged,
                installed,
            ));
        }
        if !installed.authority_matches(&package) {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::AuthorityMismatch,
                installed,
            ));
        }
        if !installed.meaning_matches(schema.declaration().members()) {
            return Err(ability_denial(
                WorthQueryAbilityInstallationDenialKind::AbilityMeaningChanged,
                installed,
            ));
        }
        Ok(())
    }
}

fn ability_denial<Schema, Ability, Scope>(
    kind: WorthQueryAbilityInstallationDenialKind,
    installed: &WorthQueryInstalledAbility<Schema, Ability, Scope>,
) -> WorthQueryAbilityInstallationDenial {
    WorthQueryAbilityInstallationDenial::new(kind, installed.ability())
}
