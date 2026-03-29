import {
  define,
  expr,
  type MergePlanReport,
  type RecipeHandle,
  type SignalRuntime,
  type SourceFamilyHandle,
  type SourceHandle
} from "../src/index.js";

declare const runtime: SignalRuntime;

const price: SourceHandle<number> = runtime.defineSource(
  define.source<number>("price").initial(100)
);

const tax: SourceHandle<number> = runtime.defineSource(
  define.source<number>("tax").initial(8)
);

const total: RecipeHandle<number> = runtime.defineRecipe(
  define
    .recipe<number>("total")
    .reads(price, tax)
    .expr(expr.sum(expr.read<number>("price"), expr.read<number>("tax")))
);

const totalValue: number = total.read();
price.set(totalValue);

const prices: SourceFamilyHandle<number> = runtime.defineSourceFamily(
  define.sourceFamily<number>("priceFamily").initial(0)
);

const taxes = runtime.defineSourceFamily(
  define.sourceFamily<number>("taxFamily").initial(0)
);

const totals = runtime.defineRecipeFamily(
  define
    .recipeFamily<number>("totalFamily")
    .reads(prices, taxes)
    .expr(expr.sum(expr.read<number>("priceFamily"), expr.read<number>("taxFamily")))
);

prices.key("cart-1").set(100);
const keyedTotal: number = totals.key("cart-1").read();

const profileRecipe = runtime.defineRecipe(
  define
    .recipe<{ name: string; tier: string }>("profile")
    .reads("price")
    .expr(
      expr.pick(
        expr.object({
          name: "forge",
          tier: "runtime",
          ignored: "x"
        }),
        "name",
        "tier"
      )
    )
);

const profile = profileRecipe.read();
const tier: string = profile.tier;

const mergePlan: MergePlanReport = runtime.history().planMergeBranches(1, 2);
const candidateCount: number = mergePlan.candidateCount;
const keyedSum: number = keyedTotal + candidateCount + Number(tier.length);

void keyedSum;
