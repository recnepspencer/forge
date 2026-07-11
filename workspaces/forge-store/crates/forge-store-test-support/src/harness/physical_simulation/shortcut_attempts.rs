use forge_store_physical_certification::{
    DriverAdmissionDenial, IoPressureDriver, MemoryPressureDriver, PhysicalSimulationDriver,
    private_mutation_driver_attempt, test_support_verdict_driver_attempt,
};

pub fn private_mutation_driver_attempt_fixture()
-> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    private_mutation_driver_attempt()
}

pub fn fake_in_memory_only_driver_attempt()
-> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    MemoryPressureDriver::fake_in_memory_only()
}

pub fn sleep_based_scheduling_driver_attempt()
-> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    IoPressureDriver::sleep_based_scheduling()
}

pub fn test_support_verdict_driver_attempt_fixture()
-> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    test_support_verdict_driver_attempt()
}
