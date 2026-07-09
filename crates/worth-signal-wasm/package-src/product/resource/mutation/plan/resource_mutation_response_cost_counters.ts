import {
  isExactMutationResponseExecutionKind,
} from "../resource_mutation_response_target_execution.js";

function readPayloadFieldExtractionBreadth(responseMappedFieldNames) {
  return responseMappedFieldNames?.length ?? 0;
}

function readTopologyTraversalBreadth(plannedTargets) {
  return readExactTargetCostBreadth(plannedTargets, "topologyTraversalBreadth");
}

function readReconstructionBreadth(plannedTargets) {
  return readExactTargetCostBreadth(plannedTargets, "reconstructionBreadth");
}

function readExactTargetCostBreadth(plannedTargets, field) {
  return plannedTargets.reduce(
    (total, target) =>
      isExactMutationResponseExecutionKind(target.execution.artifact.kind)
        ? total + target.target.cost[field]
        : total,
    0,
  );
}

export {
  readPayloadFieldExtractionBreadth,
  readReconstructionBreadth,
  readTopologyTraversalBreadth,
};
