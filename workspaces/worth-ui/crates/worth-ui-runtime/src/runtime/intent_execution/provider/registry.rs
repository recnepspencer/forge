use core::marker::PhantomData;
use std::sync::Arc;

use crate::capability::{
    FrozenIntentDefinitionCapabilities, IntentDefinitionDescriptor, UiIntent, UiIntentDefinition,
    UiIntentDefinitionDestination, UiIntentExecutionDestination, UiIntentId, UiIntentPayload,
    UiIntentPayloadProjection, UiIntentPayloadProjectionViolation, UiIntentProjectedValue,
    UiIntentRuntimeServiceDestination, UiIntentSchema, UiIntentTransitionDestination,
    UiIntentTransitionOutcome,
};

use super::{
    UiIntentExecutionBindingSupport, UiIntentExecutionProvider, UiIntentProviderVersion,
    UiPreparedIntentExecution,
};

#[path = "registry/digest.rs"]
mod digest;
#[path = "registry/runtime_service_support.rs"]
mod runtime_service_support;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiIntentExecutionBindingPreparationDenial {
    DuplicateIntent { intent: UiIntentId },
    MissingBinding { intent: UiIntentId },
    DefinitionMismatch { intent: UiIntentId },
    ExtraBinding { intent: UiIntentId },
}

pub(crate) struct UiIntentExecutionBindingPlan {
    entries: Vec<UiRegisteredIntentExecutionBinding>,
}

pub(crate) struct FrozenIntentExecutionBindings {
    descriptors: Vec<UiIntentExecutionBindingDescriptor>,
    bindings: Vec<Arc<dyn UiIntentExecutionBinding>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UiIntentExecutionBindingDescriptor {
    intent: UiIntentId,
    payload: UiIntentSchema,
    outcome: UiIntentSchema,
    destination: UiIntentExecutionDestination,
    provider_version: UiIntentProviderVersion,
    support: UiIntentExecutionBindingSupport,
}

struct UiRegisteredIntentExecutionBinding {
    descriptor: UiIntentExecutionBindingDescriptor,
    binding: Arc<dyn UiIntentExecutionBinding>,
}

trait UiIntentExecutionBinding: Send + Sync {
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation>;
}

struct UiApplicationIntentExecutionBinding<I, Provider>
where
    I: UiIntent,
    Provider: UiIntentExecutionProvider<I>,
{
    provider: Arc<Provider>,
    intent: PhantomData<fn() -> I>,
}

struct UiTransitionIntentExecutionBinding<I: UiIntent> {
    destination: UiIntentTransitionDestination,
    intent: PhantomData<fn() -> I>,
}

struct UiUnsupportedCommandIntentExecutionBinding<I: UiIntent> {
    intent: PhantomData<fn() -> I>,
}

struct UiRuntimeServiceIntentExecutionBinding<I: UiIntent> {
    destination: UiIntentRuntimeServiceDestination,
    intent: PhantomData<fn() -> I>,
}

impl UiIntentExecutionBindingPlan {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn register_application<I, Provider>(
        &mut self,
        definition: UiIntentDefinition<I, crate::capability::UiApplicationEffectDestination>,
        provider: Provider,
    ) -> Result<(), UiIntentExecutionBindingPreparationDenial>
    where
        I: UiIntent,
        Provider: UiIntentExecutionProvider<I>,
    {
        let descriptor = descriptor(
            &definition,
            Provider::VERSION,
            UiIntentExecutionBindingSupport::Supported,
        );
        self.push(UiRegisteredIntentExecutionBinding {
            descriptor,
            binding: Arc::new(UiApplicationIntentExecutionBinding::<I, Provider> {
                provider: Arc::new(provider),
                intent: PhantomData,
            }),
        })
    }

    pub(crate) fn register_transition<I>(
        &mut self,
        definition: UiIntentDefinition<I, crate::capability::UiTransitionDefinitionDestination>,
    ) -> Result<(), UiIntentExecutionBindingPreparationDenial>
    where
        I: UiIntent,
        I::ProductOutcome: UiIntentTransitionOutcome,
    {
        let UiIntentExecutionDestination::UiTransition(destination) =
            definition.execution_destination()
        else {
            unreachable!("transition definition retains its destination marker")
        };
        let descriptor = descriptor(
            &definition,
            UiIntentProviderVersion::stable(1),
            UiIntentExecutionBindingSupport::Supported,
        );
        self.push(UiRegisteredIntentExecutionBinding {
            descriptor,
            binding: Arc::new(UiTransitionIntentExecutionBinding::<I> {
                destination,
                intent: PhantomData,
            }),
        })
    }

    pub(crate) fn register_unsupported_command<I>(
        &mut self,
        definition: UiIntentDefinition<I, crate::capability::UiRuntimeServiceDefinitionDestination>,
    ) -> Result<(), UiIntentExecutionBindingPreparationDenial>
    where
        I: UiIntent,
    {
        let UiIntentExecutionDestination::RuntimeService(destination) =
            definition.execution_destination()
        else {
            unreachable!("runtime-service definition retains its destination marker")
        };
        debug_assert_eq!(
            destination,
            UiIntentRuntimeServiceDestination::InvokeCommand,
            "command registration is prevalidated by the application builder"
        );
        let descriptor = descriptor(
            &definition,
            UiIntentProviderVersion::stable(1),
            UiIntentExecutionBindingSupport::Unsupported,
        );
        self.push(UiRegisteredIntentExecutionBinding {
            descriptor,
            binding: Arc::new(UiUnsupportedCommandIntentExecutionBinding::<I> {
                intent: PhantomData,
            }),
        })
    }

    pub(crate) fn register_portal_service<I>(
        &mut self,
        definition: UiIntentDefinition<I, crate::capability::UiRuntimeServiceDefinitionDestination>,
    ) -> Result<(), UiIntentExecutionBindingPreparationDenial>
    where
        I: UiIntent,
    {
        let UiIntentExecutionDestination::RuntimeService(destination) =
            definition.execution_destination()
        else {
            unreachable!("runtime-service definition retains its destination marker")
        };
        assert!(
            matches!(
                destination,
                UiIntentRuntimeServiceDestination::OpenPortal
                    | UiIntentRuntimeServiceDestination::ClosePortal
            ),
            "only portal service destinations are installed by this milestone surface"
        );
        let descriptor = descriptor(
            &definition,
            UiIntentProviderVersion::stable(1),
            UiIntentExecutionBindingSupport::Supported,
        );
        self.push(UiRegisteredIntentExecutionBinding {
            descriptor,
            binding: Arc::new(UiRuntimeServiceIntentExecutionBinding::<I> {
                destination,
                intent: PhantomData,
            }),
        })
    }

    pub(crate) fn freeze(
        mut self,
        definitions: &FrozenIntentDefinitionCapabilities,
    ) -> Result<FrozenIntentExecutionBindings, UiIntentExecutionBindingPreparationDenial> {
        self.entries.sort_by_key(|entry| entry.descriptor.intent);
        if self.entries.len() != definitions.len() {
            if let Some(definition) = definitions
                .definitions()
                .iter()
                .find(|definition| !self.has(definition.id()))
            {
                return Err(UiIntentExecutionBindingPreparationDenial::MissingBinding {
                    intent: definition.id(),
                });
            }
            let extra = self
                .entries
                .iter()
                .find(|entry| definitions.get(&entry.descriptor.intent).is_none())
                .expect("unequal aligned sets have an extra binding");
            return Err(UiIntentExecutionBindingPreparationDenial::ExtraBinding {
                intent: extra.descriptor.intent,
            });
        }
        let mut descriptors = Vec::with_capacity(self.entries.len());
        let mut bindings = Vec::with_capacity(self.entries.len());
        for (entry, definition) in self.entries.into_iter().zip(definitions.definitions()) {
            if !entry.descriptor.matches(definition) {
                return Err(
                    UiIntentExecutionBindingPreparationDenial::DefinitionMismatch {
                        intent: definition.id(),
                    },
                );
            }
            descriptors.push(entry.descriptor);
            bindings.push(entry.binding);
        }
        Ok(FrozenIntentExecutionBindings {
            descriptors,
            bindings,
        })
    }

    fn push(
        &mut self,
        entry: UiRegisteredIntentExecutionBinding,
    ) -> Result<(), UiIntentExecutionBindingPreparationDenial> {
        if self.has(entry.descriptor.intent) {
            return Err(UiIntentExecutionBindingPreparationDenial::DuplicateIntent {
                intent: entry.descriptor.intent,
            });
        }
        self.entries.push(entry);
        Ok(())
    }

    fn has(&self, intent: UiIntentId) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.descriptor.intent == intent)
    }
}

impl FrozenIntentExecutionBindings {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn len(&self) -> usize {
        self.bindings.len()
    }

    pub(crate) fn project_at(
        &self,
        slot: crate::capability::UiIntentDefinitionSlot,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation> {
        self.bindings[slot.index()].project(values)
    }

    pub(crate) fn support_at(
        &self,
        slot: crate::capability::UiIntentDefinitionSlot,
    ) -> UiIntentExecutionBindingSupport {
        self.descriptors[slot.index()].support
    }
}

impl Clone for FrozenIntentExecutionBindings {
    fn clone(&self) -> Self {
        Self {
            descriptors: self.descriptors.clone(),
            bindings: self.bindings.clone(),
        }
    }
}

impl UiIntentExecutionBindingDescriptor {
    fn matches(&self, definition: &IntentDefinitionDescriptor) -> bool {
        self.intent == definition.id()
            && self.payload == definition.payload_schema()
            && self.outcome == definition.product_outcome_schema()
            && self.destination == definition.execution_destination()
    }
}

impl<I, Provider> UiIntentExecutionBinding for UiApplicationIntentExecutionBinding<I, Provider>
where
    I: UiIntent,
    Provider: UiIntentExecutionProvider<I>,
{
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation> {
        project_payload::<I>(values).map(|payload| {
            UiPreparedIntentExecution::application::<I, Provider>(
                payload,
                Arc::clone(&self.provider),
            )
        })
    }
}

impl<I> UiIntentExecutionBinding for UiTransitionIntentExecutionBinding<I>
where
    I: UiIntent,
    I::ProductOutcome: UiIntentTransitionOutcome,
{
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation> {
        project_payload::<I>(values)
            .map(|payload| UiPreparedIntentExecution::transition::<I>(payload, self.destination))
    }
}

impl<I: UiIntent> UiIntentExecutionBinding for UiUnsupportedCommandIntentExecutionBinding<I> {
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation> {
        project_payload::<I>(values).map(UiPreparedIntentExecution::unsupported_command::<I>)
    }
}

impl<I: UiIntent> UiIntentExecutionBinding for UiRuntimeServiceIntentExecutionBinding<I> {
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiPreparedIntentExecution, UiIntentPayloadProjectionViolation> {
        project_payload::<I>(values).map(|payload| {
            UiPreparedIntentExecution::runtime_service::<I>(payload, self.destination)
        })
    }
}

fn descriptor<I: UiIntent, D: UiIntentDefinitionDestination>(
    definition: &UiIntentDefinition<I, D>,
    provider_version: UiIntentProviderVersion,
    support: UiIntentExecutionBindingSupport,
) -> UiIntentExecutionBindingDescriptor {
    UiIntentExecutionBindingDescriptor {
        intent: definition.id(),
        payload: definition.payload_schema(),
        outcome: definition.product_outcome_schema(),
        destination: definition.execution_destination(),
        provider_version,
        support,
    }
}

fn project_payload<I: UiIntent>(
    values: Vec<UiIntentProjectedValue>,
) -> Result<I::Payload, UiIntentPayloadProjectionViolation> {
    let mut projection = UiIntentPayloadProjection::<I::Payload>::new(values);
    let payload = I::Payload::project(&mut projection)?;
    projection.finish()?;
    Ok(payload)
}
