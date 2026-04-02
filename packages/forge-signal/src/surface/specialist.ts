import type { DiagnosticsGraphSummary, RunSummary, VersionSummary } from "../types.d.ts";

export class SignalSpecialist {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  graphSummary(): DiagnosticsGraphSummary {
    return this.inner.graph_summary() as DiagnosticsGraphSummary;
  }

  evaluateDirty(): RunSummary {
    return this.inner.evaluate_dirty() as RunSummary;
  }

  readVersions(ids: string[]): VersionSummary[] {
    return this.inner.read_versions(ids) as VersionSummary[];
  }
}
