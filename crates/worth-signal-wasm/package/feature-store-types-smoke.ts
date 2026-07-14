import { createSignals } from "./index.js";

const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});

type DataTableUserLayoutConfig = {
  readonly density: "compact" | "comfortable";
  readonly visibleColumns: readonly string[];
  readonly pinned: {
    readonly left: readonly string[];
    readonly right: readonly string[];
  };
};

type WorkplaceAuditLogsAdminQueryValues = {
  readonly search: string;
  readonly severities: readonly ("info" | "warning" | "error")[];
  readonly includeResolved: boolean;
};

const initialLayoutConfig: DataTableUserLayoutConfig = {
  density: "comfortable",
  visibleColumns: ["event", "actor", "severity"],
  pinned: {
    left: ["event"],
    right: [],
  },
};

const initialQueryValues: WorkplaceAuditLogsAdminQueryValues = {
  search: "",
  severities: ["warning"],
  includeResolved: false,
};

type WorkplaceAuditLogsAdminStoreState = {
  search: string;
  page: number;
  layoutConfig: DataTableUserLayoutConfig;
  queryValues: WorkplaceAuditLogsAdminQueryValues;
  quickReportId: string | null;
};

const initialAuditStoreState: WorkplaceAuditLogsAdminStoreState = {
  search: "",
  page: 1,
  layoutConfig: initialLayoutConfig,
  queryValues: initialQueryValues,
  quickReportId: null,
};

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

const auditStore = signals.featureStore({
  id: "workplace-audit-logs-admin",
  state: initialAuditStoreState,
  actions: ({ set, read }) => ({
    setQueryValues(next: WorkplaceAuditLogsAdminQueryValues) {
      return set("queryValues", next);
    },
    setLayoutConfig(next: DataTableUserLayoutConfig) {
      return set("layoutConfig", next);
    },
    resetFilters() {
      return set("queryValues", initialQueryValues);
    },
    snapshot() {
      return read();
    },
  }),
});

const layoutConfig: DataTableUserLayoutConfig = auditStore.read().layoutConfig;
const queryValues: WorkplaceAuditLogsAdminQueryValues = auditStore.read().queryValues;
auditStore.actions.setQueryValues({
  search: "billing",
  severities: ["error"],
  includeResolved: true,
});
auditStore.actions.setLayoutConfig({
  density: "compact",
  visibleColumns: ["event", "severity"],
  pinned: {
    left: ["severity"],
    right: [],
  },
});

void selectedGroupId;
void selectedCandidateId;
void selectedView;
void scopedStoreScopeId;
void layoutConfig;
void queryValues;

await signals.terminate();
