function readIdFrom(value) {
  if (typeof value === "string") {
    return value;
  }
  if (value && typeof value.id === "string") {
    return value.id;
  }
  throw new Error("Recipe reads must be signal ids or signal handles.");
}

function normalizePartitionSubscription(scope) {
  if (!scope || typeof scope !== "object" || typeof scope.partition !== "string") {
    throw new Error("Partition subscription requires a partition string.");
  }
  return {
    partition: scope.partition,
    detail: scope.detail ?? null,
    matchMode: scope.matchMode ?? (scope.detail ? "PartitionAndDetail" : "WholePartition"),
  };
}

function normalizeRecipeReadScope(scope) {
  if (!scope || typeof scope !== "object") {
    throw new Error("Recipe read scope must be an object.");
  }
  return {
    partition: typeof scope.partition === "string" ? scope.partition : null,
    partitionFrom: scope.partitionFrom ?? null,
    detail: scope.detail ?? null,
    matchMode: scope.matchMode ?? null,
  };
}

function recipeReadFrom(value) {
  if (typeof value === "string") {
    return value;
  }
  if (value && typeof value.kind === "string" && value.kind === "signal" && typeof value.id === "string") {
    return {
      kind: "signal",
      id: value.id,
      scope: value.scope ? normalizePartitionSubscription(value.scope) : null,
    };
  }
  if (value && typeof value.id === "string") {
    return value.id;
  }
  throw new Error("Recipe reads must be signal ids, handles, or scoped signal read specs.");
}

function familyIdFrom(value) {
  if (typeof value === "string") {
    return value;
  }
  if (value && typeof value.familyId === "string") {
    return value.familyId;
  }
  throw new Error("Family reads must be family ids or family handles.");
}

function recipeFamilyReadFrom(value) {
  if (typeof value === "string") {
    return { kind: "signal", id: value };
  }
  if (value && typeof value.kind === "string") {
    if (value.kind === "signal" && typeof value.id === "string") {
      return {
        kind: "signal",
        id: value.id,
        scope: value.scope ? normalizeRecipeReadScope(value.scope) : null,
      };
    }
    if (value.kind === "keyed" && typeof value.familyId === "string") {
      return {
        kind: "keyed",
        familyId: value.familyId,
        scope: value.scope ? normalizeRecipeReadScope(value.scope) : null,
      };
    }
  }
  if (value && typeof value.id === "string") {
    return { kind: "signal", id: value.id, scope: null };
  }
  if (value && typeof value.familyId === "string") {
    return { kind: "keyed", familyId: value.familyId, scope: null };
  }
  throw new Error("Recipe family reads must be signal ids, signal handles, or keyed family handles.");
}

export class SourceBuilder {
  constructor(id) {
    this.spec = { id, initial: null };
  }

  initial(value) {
    this.spec.initial = value;
    return this;
  }

  build() {
    return { ...this.spec };
  }
}

export class RecipeBuilder {
  constructor(id) {
    this.spec = { id, reads: [] };
  }

  reads(...reads) {
    this.spec.reads = reads.flat().map(recipeReadFrom);
    return this;
  }

  expr(value) {
    this.spec.expr = value;
    return this;
  }

  when(expr) {
    this.spec.when = { expr };
    return this;
  }

  identityExact() {
    this.spec.identity = { kind: "exact" };
    return this;
  }

  identity(expr) {
    this.spec.identity = { kind: "expr", expr };
    return this;
  }

  build() {
    if (!("expr" in this.spec)) {
      throw new Error(`Recipe \`${this.spec.id}\` is missing an expression.`);
    }
    return {
      id: this.spec.id,
      reads: [...this.spec.reads],
      expr: this.spec.expr,
      when: this.spec.when ?? null,
      identity: this.spec.identity ?? null
    };
  }
}

export class SourceFamilyBuilder {
  constructor(familyId) {
    this.spec = { familyId, initial: null };
  }

  initial(value) {
    this.spec.initial = value;
    return this;
  }

  build() {
    return { ...this.spec };
  }
}

export class RecipeFamilyBuilder {
  constructor(familyId) {
    this.spec = { familyId, reads: [] };
  }

  reads(...reads) {
    this.spec.reads = reads.flat().map(recipeFamilyReadFrom);
    return this;
  }

  expr(value) {
    this.spec.expr = value;
    return this;
  }

  when(expr) {
    this.spec.when = { expr };
    return this;
  }

  identityExact() {
    this.spec.identity = { kind: "exact" };
    return this;
  }

  identity(expr) {
    this.spec.identity = { kind: "expr", expr };
    return this;
  }

  build() {
    if (!("expr" in this.spec)) {
      throw new Error(`Recipe family \`${this.spec.familyId}\` is missing an expression.`);
    }
    return {
      familyId: this.spec.familyId,
      reads: [...this.spec.reads],
      expr: this.spec.expr,
      when: this.spec.when ?? null,
      identity: this.spec.identity ?? null
    };
  }
}

export const define = {
  source(id) {
    return new SourceBuilder(id);
  },
  recipe(id) {
    return new RecipeBuilder(id);
  },
  sourceFamily(familyId) {
    return new SourceFamilyBuilder(familyId);
  },
  recipeFamily(familyId) {
    return new RecipeFamilyBuilder(familyId);
  }
};

export const keyed = {
  read(family) {
    return { kind: "keyed", familyId: familyIdFrom(family) };
  },
  signal(read) {
    return { kind: "signal", id: readIdFrom(read), scope: null };
  }
};

export const partition = {
  whole(partition) {
    return { partition, detail: null, matchMode: "WholePartition" };
  },
  detail(partition, detail) {
    return { partition, detail, matchMode: "PartitionAndDetail" };
  },
  currentKey(detail = null, matchMode = null) {
    return { partitionFrom: "key", detail, matchMode };
  },
};
