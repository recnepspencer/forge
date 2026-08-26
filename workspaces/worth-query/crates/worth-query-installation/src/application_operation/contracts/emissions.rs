//! Installed application-effect emissions, distinct from graph touches and external dispatch.

/// One application effect that the installed operation may emit.
///
/// This is retained program meaning. It grants neither graph-mutation nor
/// external-dispatch authority.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryInstalledApplicationEffectEmission {
    effect: String,
}

impl WorthQueryInstalledApplicationEffectEmission {
    pub const fn effect(&self) -> &str {
        self.effect.as_str()
    }
}

/// The exact application-effect emission ceiling for one installed operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationEmissionContract {
    emissions: Vec<WorthQueryInstalledApplicationEffectEmission>,
}

impl WorthQueryOperationEmissionContract {
    pub fn emissions(&self) -> &[WorthQueryInstalledApplicationEffectEmission] {
        &self.emissions
    }

    pub const fn is_declared(&self) -> bool {
        !self.emissions.is_empty()
    }
}

pub(in crate::application_operation) fn install_portable_effect_emissions(
    effects: &[String],
) -> WorthQueryOperationEmissionContract {
    let emissions = effects
        .iter()
        .cloned()
        .map(|effect| WorthQueryInstalledApplicationEffectEmission { effect })
        .collect::<Vec<_>>();
    WorthQueryOperationEmissionContract { emissions }
}
