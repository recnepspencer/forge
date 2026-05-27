import { createSignals } from "./index.js";

const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});

const store = signals.featureStore({
  id: "workplace-user-groups-admin",
  state: {
    selectedGroupId: null as string | null,
    selectedCandidateId: "",
    view: "users" as "users" | "groups",
  },
  actions: ({ set, read }) => ({
    setSelectedGroupId(next: string | null) {
      return set("selectedGroupId", next);
    },
    setSelectedCandidateId(next: string) {
      return set("selectedCandidateId", next);
    },
    showGroups() {
      return set("view", "groups");
    },
    snapshot() {
      return read();
    },
  }),
});

const selectedGroupId: string | null = store.read().selectedGroupId;
const selectedCandidateId: string = store.read().selectedCandidateId;
const selectedView: "users" | "groups" = store.read().view;
store.actions.setSelectedGroupId("group-12");
store.actions.setSelectedCandidateId("candidate-7");
store.actions.showGroups();
const scopedStore = signals.scope("admin").featureStore({
  id: "catalog",
  state: {
    selectedProductId: null as string | null,
  },
  actions: ({ set }) => ({
    setSelectedProductId(next: string | null) {
      return set("selectedProductId", next);
    },
  }),
});
const scopedStoreScopeId: string = scopedStore.scopeId;

void selectedGroupId;
void selectedCandidateId;
void selectedView;
void scopedStoreScopeId;

await signals.terminate();
