export function wrapSpecialist(rawSpecialist) {
  return {
    evaluateDirty() {
      return rawSpecialist.evaluate_dirty();
    },
    evaluate_dirty() {
      return rawSpecialist.evaluate_dirty();
    },
    graphSummary() {
      return rawSpecialist.graph_summary();
    },
    graph_summary() {
      return rawSpecialist.graph_summary();
    },
    readVersions(ids) {
      return rawSpecialist.read_versions(ids);
    },
    read_versions(ids) {
      return rawSpecialist.read_versions(ids);
    },
    free: rawSpecialist.free.bind(rawSpecialist),
    [Symbol.dispose]() {
      if (typeof rawSpecialist[Symbol.dispose] === "function") {
        rawSpecialist[Symbol.dispose]();
        return;
      }
      rawSpecialist.free();
    },
  };
}
