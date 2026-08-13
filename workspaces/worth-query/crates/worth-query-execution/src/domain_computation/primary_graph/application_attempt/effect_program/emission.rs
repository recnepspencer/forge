use worth_query_declaration::facade::application_schema::{
    ApplicationEffectPayload, ApplicationEffectRef, ApplicationExternalEffectPayload,
    OperationEmits,
};
use worth_query_installation::facade::ApplicationOperationProgramTarget;

use super::{
    denial, WorthQueryApplicationEffectProgramBuilder, WorthQueryApplicationEmission,
    WorthQueryApplicationRealizedEffect,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
};

impl<Schema, Operation, Input, Scope>
    WorthQueryApplicationEffectProgramBuilder<Schema, Operation, Input, Scope>
{
    /// An installed effect declaration is not enough; the operation itself
    /// must carry the compile-time emit capability:
    ///
    /// ```compile_fail
    /// use worth_query_declaration::facade::application_schema::ApplicationEffectRef;
    /// use worth_query_execution::facade::primary_graph::WorthQueryApplicationEffectProgramBuilder;
    ///
    /// struct Schema;
    /// struct Operation;
    /// struct Input;
    /// struct Scope;
    /// struct UndeclaredEffect;
    ///
    /// fn cannot_emit_undeclared_effect(
    ///     builder: &mut WorthQueryApplicationEffectProgramBuilder<
    ///         Schema, Operation, Input, Scope,
    ///     >,
    /// ) {
    ///     let effect =
    ///         ApplicationEffectRef::<Schema, UndeclaredEffect, String>::from_schema_identifier(
    ///             "UndeclaredEffect",
    ///         );
    ///     builder.emit(effect, "payload".to_owned()).unwrap();
    /// }
    /// ```
    pub fn emit<Effect, Payload>(
        &mut self,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        payload: Payload,
    ) -> Result<(), WorthQueryApplicationAttemptDenial>
    where
        Effect: OperationEmits<Operation>,
        Payload: ApplicationEffectPayload,
    {
        self.admit_program_target(&ApplicationOperationProgramTarget::Emit {
            effect: effect.name().to_string(),
        })?;
        let Some(retained_bytes) = self
            .emission_retained_bytes
            .checked_add(payload.retained_bytes())
        else {
            return Err(retained_bytes_denial(effect.name()));
        };
        if retained_bytes > self.emission_retained_bytes_ceiling {
            return Err(retained_bytes_denial(effect.name()));
        }
        self.effects.push(WorthQueryApplicationRealizedEffect::Emit(
            WorthQueryApplicationEmission::new(effect.name(), payload),
        ));
        self.emission_retained_bytes = retained_bytes;
        Ok(())
    }

    /// Emits the exact typed payload selected by an installed external-effect
    /// contract. The provider later matches this emission to that contract;
    /// callers never supply wire bytes at commit or dispatch.
    pub fn emit_external<Effect, Payload>(
        &mut self,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        payload: Payload,
    ) -> Result<(), WorthQueryApplicationAttemptDenial>
    where
        Effect: OperationEmits<Operation>,
        Payload: ApplicationExternalEffectPayload,
    {
        self.admit_program_target(&ApplicationOperationProgramTarget::Emit {
            effect: effect.name().to_string(),
        })?;
        let Some(retained_bytes) = self
            .emission_retained_bytes
            .checked_add(payload.retained_bytes())
        else {
            return Err(retained_bytes_denial(effect.name()));
        };
        if retained_bytes > self.emission_retained_bytes_ceiling {
            return Err(retained_bytes_denial(effect.name()));
        }
        let emission = WorthQueryApplicationEmission::new_external(effect.name(), payload)
            .map_err(|()| external_payload_denial(effect.name()))?;
        self.effects
            .push(WorthQueryApplicationRealizedEffect::Emit(emission));
        self.emission_retained_bytes = retained_bytes;
        Ok(())
    }
}

fn retained_bytes_denial(effect: &str) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::RetainedEffectBytesExceeded,
        effect,
    )
}

fn external_payload_denial(effect: &str) -> WorthQueryApplicationAttemptDenial {
    denial(
        WorthQueryApplicationAttemptDenialKind::ExternalEffectPayloadProjectionRejected,
        effect,
    )
}
