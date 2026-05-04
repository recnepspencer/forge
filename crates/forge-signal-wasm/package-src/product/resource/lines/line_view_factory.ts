import { createResourceViewHandle } from "../views/view_handle.js";

function createLineView(materialization, project) {
  if (typeof project !== "function") {
    throw new TypeError(
      "resource line view(...) requires a projection function",
    );
  }
  const viewHandle = createResourceViewHandle(
    materialization.lineScope.computed(
      () => project(materialization.binding.valueSignal()),
      {
        debugName: "resourceLineView",
      },
    ),
  );
  materialization.lifecycle.addOwnedView(viewHandle);
  return viewHandle;
}

export { createLineView };
