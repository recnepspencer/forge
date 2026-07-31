#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentPayloadProjectionCost {
    declared_fields: usize,
    query_inputs_read: usize,
    application_inputs_read: usize,
    admitted_utf8_bytes: usize,
}

pub struct UiIntentInputBasisReceipt {
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    publication_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    cost: UiIntentPayloadProjectionCost,
}

pub(crate) struct UiIntentInputBasis {
    receipt: UiIntentInputBasisReceipt,
    interaction: crate::runtime::interaction::UiSemanticInteraction,
    query_inputs: Box<[worth_ui_query_binding::UiProjectionInputFactReference]>,
    application_inputs: Box<[super::UiIntentApplicationInputReference]>,
}

pub(crate) struct UiIntentInputBasisInput {
    pub(crate) generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    pub(crate) publication_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(crate) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    pub(crate) interaction: crate::runtime::interaction::UiSemanticInteraction,
    pub(crate) query_inputs: Vec<worth_ui_query_binding::UiProjectionInputFactReference>,
    pub(crate) application_inputs: Vec<super::UiIntentApplicationInputReference>,
    pub(crate) cost: UiIntentPayloadProjectionCost,
}

impl UiIntentInputBasis {
    pub(crate) fn seal(input: UiIntentInputBasisInput) -> Self {
        Self {
            receipt: UiIntentInputBasisReceipt {
                generation: input.generation,
                publication_frame: input.publication_frame,
                target: input.target,
                cost: input.cost,
            },
            interaction: input.interaction,
            query_inputs: input.query_inputs.into_boxed_slice(),
            application_inputs: input.application_inputs.into_boxed_slice(),
        }
    }

    pub(crate) const fn receipt(&self) -> &UiIntentInputBasisReceipt {
        &self.receipt
    }

    pub(crate) fn retained_owner_reference_count(&self) -> usize {
        let _ = &self.interaction;
        1 + self.query_inputs.len() + self.application_inputs.len()
    }
}

impl UiIntentInputBasisReceipt {
    pub const fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation
    }

    pub const fn publication_frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.publication_frame
    }

    pub const fn target(&self) -> crate::runtime::interaction::UiPresentedInteractionTargetView {
        self.target
    }

    pub const fn cost(&self) -> UiIntentPayloadProjectionCost {
        self.cost
    }
}

impl UiIntentPayloadProjectionCost {
    pub(crate) fn record_field(&mut self) {
        self.declared_fields = next(self.declared_fields);
    }

    pub(crate) fn record_query_input(&mut self) {
        self.query_inputs_read = next(self.query_inputs_read);
    }

    pub(crate) fn record_application_input(&mut self) {
        self.application_inputs_read = next(self.application_inputs_read);
    }

    pub(crate) fn record_utf8_bytes(&mut self, bytes: usize) {
        self.admitted_utf8_bytes = self
            .admitted_utf8_bytes
            .checked_add(bytes)
            .expect("bounded payload byte accounting exhausted");
    }

    pub const fn declared_fields(self) -> usize {
        self.declared_fields
    }

    pub const fn query_inputs_read(self) -> usize {
        self.query_inputs_read
    }

    pub const fn application_inputs_read(self) -> usize {
        self.application_inputs_read
    }

    pub const fn admitted_utf8_bytes(self) -> usize {
        self.admitted_utf8_bytes
    }
}

fn next(value: usize) -> usize {
    value
        .checked_add(1)
        .expect("bounded payload field accounting exhausted")
}
