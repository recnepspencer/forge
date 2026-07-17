use super::super::{
    UiAdmittedAllocationSourceOrder, UiAdmittedAllocationStreamIngress,
    UiAllocationFrameDispatcher, UiAllocationFrameIngressIdentity,
    UiAllocationFrameSourceGeneration, UiAllocationFrameSourceLease,
};
use super::{UiAllocationFrameSourceFact, UiAllocationFrameSourceSubmission};

#[derive(Debug, Default)]
pub(crate) struct UiAllocationFrameGatewayState {
    sources: Vec<UiAllocationFrameSourceLease>,
}

impl UiAllocationFrameGatewayState {
    pub(crate) fn launch() -> Self {
        Self::default()
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn admit(
        &mut self,
        dispatcher: &mut UiAllocationFrameDispatcher,
        submission: UiAllocationFrameSourceSubmission,
    ) -> Result<
        UiAdmittedAllocationStreamIngress,
        (
            super::super::UiAllocationFrameSourceAdmissionDenial,
            Box<UiAllocationFrameSourceFact>,
        ),
    > {
        let UiAllocationFrameSourceSubmission {
            lane,
            source_identity: identity,
            source_generation,
            ingress_identity,
            source_order,
            fact,
        } = submission;
        let generation = UiAllocationFrameSourceGeneration::from_gateway(source_generation);
        let source_index = self.sources.iter().position(|source| {
            source.source_lane() == lane && source.source_identity() == identity
        });
        let index = match source_index {
            Some(index) if self.sources[index].source_generation() == generation => index,
            Some(index) => {
                let successor =
                    match dispatcher.advance_source_generation(&self.sources[index], generation) {
                        Ok(successor) => successor,
                        Err(denial) => return Err((denial, Box::new(fact))),
                    };
                self.sources[index] = successor;
                index
            }
            None => {
                let lease = match dispatcher.admit_source_generation(lane, identity, generation) {
                    Ok(lease) => lease,
                    Err(denial) => return Err((denial, Box::new(fact))),
                };
                self.sources.push(lease);
                self.sources.len() - 1
            }
        };
        Ok(self.sources[index].admit_gateway_ingress(
            generation,
            UiAllocationFrameIngressIdentity::from_gateway(ingress_identity),
            UiAdmittedAllocationSourceOrder::from_gateway(source_order),
            fact,
        ))
    }
}
