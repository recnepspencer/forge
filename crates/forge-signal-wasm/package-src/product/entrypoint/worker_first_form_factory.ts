import { createFormController } from "../forms/form_controller.js";
import { createFormSourceFactory } from "../forms/sources/form_sources.js";
import { freezeObject } from "../graph_support.js";

const formSourceFactory = createFormSourceFactory();

export function createWorkerFirstFormFactory(signalNamespace) {
  function createForm(declaration) {
    return createFormController(signalNamespace, declaration);
  }

  Object.defineProperty(createForm, "source", {
    enumerable: true,
    value: formSourceFactory,
  });

  return freezeObject(createForm);
}
