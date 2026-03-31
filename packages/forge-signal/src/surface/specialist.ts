export class SignalSpecialist {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  graphSummary() {
    return this.inner.graph_summary();
  }

  evaluateDirty() {
    return this.inner.evaluate_dirty();
  }

  readVersions(ids: string[]) {
    return this.inner.read_versions(ids);
  }
}
