use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

/// Move-only authority to progress graph obligations installed into one exact
/// runtime generation from selection into executable admission.
///
/// The authority is minted only alongside an execution-owned installed package
/// index. Descriptive runtime ordinals or generations cannot recreate it.
///
/// ```compile_fail
/// use worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority;
///
/// let forged = WorthQueryInstalledGraphAdmissionAuthority {
///     runtime_ordinal: 1,
///     generation: 1,
/// };
/// ```
#[derive(Debug)]
pub struct WorthQueryInstalledGraphAdmissionAuthority {
    runtime_ordinal: u64,
    generation: u64,
}

impl WorthQueryInstalledGraphAdmissionAuthority {
    pub(crate) const fn mint(runtime_ordinal: u64, generation: u64) -> Self {
        Self {
            runtime_ordinal,
            generation,
        }
    }

    #[doc(hidden)]
    pub const fn admits(&self, binding: &ApplicationSchemaBindingIdentity) -> bool {
        self.runtime_ordinal == binding.runtime_ordinal() && self.generation == binding.generation()
    }
}
