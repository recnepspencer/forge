function readIdFrom(value) {
  if (typeof value === "string") {
    return value;
  }
  if (value && typeof value.id === "string") {
    return value.id;
  }
  throw new Error("Recipe reads must be signal ids or signal handles.");
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
    this.spec.reads = reads.flat().map(readIdFrom);
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
    this.spec.reads = reads.flat().map((read) => ({ familyId: familyIdFrom(read) }));
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
    return { familyId: familyIdFrom(family) };
  }
};
