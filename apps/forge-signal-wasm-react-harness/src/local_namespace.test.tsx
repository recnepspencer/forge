import { describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";

describe("local namespace authoring", () => {
  it("creates dialog, list, and form-source state without raw scope plumbing", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "invite-user-dialog",
      });
      const list = signals.local.listState({
        identity: "candidate-users",
        initial: ["a", "b"],
      });
      const formSource = signals.local.formSource({
        identity: "invite-user-form",
        initial: { email: "" },
      });

      expect(dialog.scopeId).toBe("invite-user-dialog");
      expect(dialog.signal.signalIdentity?.().localId).toBe("open");
      expect(dialog.signal()).toBe(false);
      dialog.open();
      expect(dialog.signal()).toBe(true);
      dialog.toggle();
      expect(dialog.signal()).toBe(false);
      expect(typeof dialog.free).toBe("function");

      expect(list.scopeId).toBe("candidate-users");
      expect(list.items.signalIdentity?.().localId).toBe("items");
      expect(list.items()).toEqual(["a", "b"]);
      list.items.set(["c"]);
      expect(list.items()).toEqual(["c"]);
      list.reset();
      expect(list.items()).toEqual(["a", "b"]);
      expect(typeof list.free).toBe("function");

      expect(formSource.scopeId).toBe("invite-user-form");
      expect(formSource.signal.signalIdentity?.().localId).toBe("source");
      expect(formSource.signal()).toEqual({ email: "" });
      expect(formSource.source.kind).toBe("signal");
      formSource.signal.assign({ email: "alex@example.com" });
      expect(formSource.signal().email).toBe("alex@example.com");
      formSource.reset();
      expect(formSource.signal()).toEqual({ email: "" });
      expect(typeof formSource.free).toBe("function");

      dialog.free();
      list.free();
      formSource.free();
    } finally {
      signals.free();
    }
  });

  it("keeps local helpers available on scoped namespaces", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const scoped = signals.scope("admin");
      const dialog = scoped.local.dialogState({
        identity: "delete-product-dialog",
        initialOpen: true,
      });

      expect(dialog.scopeId).toBe("admin.delete-product-dialog");
      expect(dialog.signal.signalIdentity?.().canonicalId).toBe(
        "admin.delete-product-dialog.open",
      );
      expect(dialog.signal()).toBe(true);
    } finally {
      signals.free();
    }
  });
});
