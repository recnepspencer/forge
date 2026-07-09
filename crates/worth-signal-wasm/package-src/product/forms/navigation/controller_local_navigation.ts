export function applyControllerLocalNavigation(navigation, form, execution) {
  const plan = execution.planSnapshot;
  if (
    execution.resultKind !== "fulfilled"
    || plan?.kind !== "step"
    || plan.step?.routeCoupled === true
  ) {
    return;
  }
  navigation.applyStepAction(plan, form.steps().artifacts);
}
