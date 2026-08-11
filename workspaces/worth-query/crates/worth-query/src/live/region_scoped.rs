#[cfg(test)]
mod execution;
#[cfg(test)]
mod planning;
#[cfg(test)]
mod stream_delivery;

#[cfg(test)]
pub(crate) use execution::execute_region_scoped_live_change;
#[cfg(test)]
pub(crate) use planning::admit_region_scoped_live_plan;
#[cfg(test)]
pub(crate) use stream_delivery::lower_region_scoped_execution_to_stream_contract;
