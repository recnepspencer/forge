import { createResourceViewHandle } from "../views/view_handle.js";
import { requireCurrentMaterialization } from "./state/line_handle_helpers.js";

function createLineView(lineBacking, project) {
  if (typeof project !== "function") {
    throw new TypeError(
      "resource line view(...) requires a projection function",
    );
  }
  const materialization = requireCurrentMaterialization(lineBacking);
  const viewHandle = createResourceViewHandle(
    materialization.lineScope.computed(
      () => project(requireCurrentMaterialization(lineBacking).binding.valueSignal()),
      {
        debugName: "resourceLineView",
      },
    ),
  );
  materialization.lifecycle.addOwnedView(viewHandle);
  return viewHandle;
}

export { createLineView };
