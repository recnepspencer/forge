# @forge/signal

TypeScript package for Forge Signal's browser-facing wasm runtime.

The package is built around typed builders and runtime handles instead of raw object soup.

```ts
import { createSignalRuntime, define, expr } from "@forge/signal";

const runtime = await createSignalRuntime();

const price = runtime.defineSource(
  define.source<number>("price").initial(100)
);

const tax = runtime.defineSource(
  define.source<number>("tax").initial(8)
);

const total = runtime.defineRecipe(
  define
    .recipe<number>("total")
    .reads(price, tax)
    .expr(expr.sum(expr.read("price"), expr.read("tax")))
);

price.set(110);
console.log(total.read());
```

Keyed families get their own handles too:

```ts
const prices = runtime.defineSourceFamily(
  define.sourceFamily<number>("price").initial(0)
);

const taxes = runtime.defineSourceFamily(
  define.sourceFamily<number>("tax").initial(0)
);

const totals = runtime.defineRecipeFamily(
  define
    .recipeFamily<number>("total")
    .reads(prices, taxes)
    .expr(expr.sum(expr.read("price"), expr.read("tax")))
);

prices.key("cart-1").set(100);
taxes.key("cart-1").set(8);

console.log(totals.key("cart-1").read());
```

Build the wasm bundle from this package directory with:

```bash
npm run build:wasm
```

This package is currently aimed at browser and bundler apps. Raw Node ESM can load the module, but full runtime execution still hits the underlying wasm clock limitation on this target.
