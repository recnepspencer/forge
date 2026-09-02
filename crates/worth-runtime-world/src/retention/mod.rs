mod dependency_counts;
mod obligation_transfer;
mod unique_component_pin;

pub(crate) use dependency_counts::{ComponentBasisDependencyClass, ComponentBasisDependencyCounts};
pub(crate) use obligation_transfer::{
    ComponentBasisObligationTransfer, ComponentBasisObligationTransferDestination,
};
pub(crate) use unique_component_pin::ExactComponentPinRequest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_accounting_and_transfer_destinations_are_closed() {
        let mut counts = ComponentBasisDependencyCounts::zero();
        for class in ComponentBasisDependencyClass::ALL {
            assert_eq!(counts.get(class), 0);
            assert_eq!(counts.increment(class), Some(1));
        }
        assert_eq!(counts.total(), ComponentBasisDependencyClass::ALL.len());

        let transfer = ComponentBasisObligationTransfer::new(
            ComponentBasisDependencyClass::AdmittedObservation,
            ComponentBasisObligationTransferDestination::Release,
        );
        assert_eq!(
            transfer.from(),
            ComponentBasisDependencyClass::AdmittedObservation
        );
        assert_eq!(
            transfer.to(),
            ComponentBasisObligationTransferDestination::Release
        );

        for class in ComponentBasisDependencyClass::ALL {
            assert_eq!(counts.decrement(class), Some(0));
        }
        assert_eq!(counts.total(), 0);
    }
}
