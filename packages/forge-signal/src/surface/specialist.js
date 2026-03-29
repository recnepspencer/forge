export class SignalSpecialist {
  constructor(inner) {
    this.inner = inner;
  }

  graphSummary() {
    return this.inner.graph_summary();
  }

  evaluateDirty() {
    return this.inner.evaluate_dirty();
  }

  readVersions(ids) {
    return this.inner.read_versions(ids);
  }
}
