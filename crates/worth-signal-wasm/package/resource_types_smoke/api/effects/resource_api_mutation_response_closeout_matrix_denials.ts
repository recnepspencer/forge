import { createSignals } from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

// @ts-expect-error mutation response closeout matrices do not accept arguments
signals.resource.mutationResponses.closeoutMatrix({
  lane: "saveDetailReplace",
});
