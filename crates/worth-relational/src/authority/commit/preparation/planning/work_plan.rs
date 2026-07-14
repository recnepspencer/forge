use crate::authority::commit::preparation::facade::PreparationWorkPlan;

pub(crate) fn empty_preparation_work_plan<'runtime>() -> PreparationWorkPlan<'runtime> {
    PreparationWorkPlan::default()
}
