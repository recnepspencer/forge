export interface DemoMetadata {
  id: number;
  title: string;
  purpose: string;
  preface: string;
  difficulty: "Beginner" | "Intermediate" | "Advanced";
  primaryMessage: string;
  forgeCode: string;
  alternativeCode: string;
  alternativeName: string;
  explanationAlternative: string;
  explanationForge: string;
  whatYouGet: string[];
  relatedDocsPath: string;
}

export const demoRegistry: DemoMetadata[] = [
  {
    id: 1,
    title: "Local Reactive Signals",
    purpose: "Show the smallest useful Forge primitive and what it gives you for local reactive state.",
    preface: "This is the spreadsheet version of local state: one source cell changes, and every dependent cell updates from the same graph. Change the title and watch slug, readiness, and output update without separate useMemo wiring or duplicate component state.",
    difficulty: "Beginner",
    primaryMessage: "Forge gives you more structured local reactivity than plain ad-hoc component state.",
    forgeCode: `import { createSignals } from "forge-signal-wasm";
const signals = await createSignals();

const count = signals.input(0);
const doubled = signals.computed(() => count() * 2);
const status = signals.computed(() =>
  count() >= 10 ? "OPTIMAL SYSTEM DENSITY" : "BOOTSTRAP INITIALIZATION"
);
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
  status: status(),
}));
`,
    alternativeName: "React useState + useMemo",
    alternativeCode: `import { useState, useMemo } from "react";

const [count, setCount] = useState(0);

// Double-computation danger & dependencies array wiring
const doubled = useMemo(() => count * 2, [count]);
const status = useMemo(() => 
  count >= 10 ? "Optimal Load" : "Bootstrap"
, [count]);`,
    explanationAlternative: "React local state can model the same UI, but the reactive contract is split between state setters, memo dependency arrays, and component rerender semantics. Observation and diagnostics are separate concerns you still have to author yourself.",
    explanationForge: "The same handle-based runtime gives you writable input, derived computed truth, and a published output projection without switching mental models. This first demo stays on the stable local graph lane instead of pretending extra observer surfaces are already frictionless in React.",
    whatYouGet: [
      "Runtime-owned input, computed, and output handles",
      "Published output projection from the same handle graph",
      "One handle model instead of separate local state plus observer glue"
    ],
    relatedDocsPath: "learn/feature-index"
  },
  {
    id: 2,
    title: "Resource Lines",
    purpose: "Show that every resource read materializes as inspectable line truth with value, status, freshness, diagnostics, and history.",
    preface: "Think of a line like a query result with a black box recorder attached. The catalog line and the selected product detail line are separate reads, so refreshing the list, receiving a backend patch, or invalidating a detail can affect different truth surfaces. Click the line tabs and products, then watch the inspector name the line, latest effect, freshness, and history.",
    difficulty: "Intermediate",
    primaryMessage: "Forge resource lines make server truth inspectable before writes enter the story.",
    forgeCode: `const api = signals.api({
  baseUrl: "/api",
  effects: signals.resource.effects.branchNative(),
});

const productDetail = signals.resource.response.detail<Product>()({
  status: "status",
  price: "price",
  inventory: "inventory",
});

const products = api
  .url("/products")
  .response(signals.resource.response.array<Product>()({
    id: "id",
    summary: { total: "total", lowStock: "lowStock" },
  }))
  .list({ load: loadProducts });

const product = api
  .url("/products/:productId")
  .response(productDetail)
  .detail({ load: loadProduct });

const catalogLine = products.line({});
const detailLine = product.line({ productId });

const catalog = useResourceLine(catalogLine, store);
const detail = useResourceLine(detailLine, store);

console.log(catalog.status().kind);
console.log(catalog.freshness().kind);
console.log(catalogLine.diagnostics().lastEffect);
console.log(catalogLine.history().entries());`,
    alternativeName: "Generic server-state query",
    alternativeCode: `const catalog = useQuery({
  queryKey: ["products"],
  queryFn: loadProducts,
});

const detail = useQuery({
  queryKey: ["product", productId],
  queryFn: () => loadProduct(productId),
});

const lineState = {
  value: catalog.data,
  status: catalog.status,
  stale: catalog.isStale,
  history: appAuthoredHistory,
  diagnostics: appAuthoredDiagnostics,
};`,
    explanationAlternative: "Server-state hooks can fetch the data, but the app usually authors separate conventions for freshness, diagnostics, history, and line identity.",
    explanationForge: "Forge materializes resource reads as line truth. The line carries value, status, freshness, diagnostics, history, and effect evidence as one runtime-owned object.",
    whatYouGet: [
      "Catalog and detail line identities",
      "Line status and freshness",
      "Declared response summaries",
      "Diagnostics and history attached to the line",
      "Delivery, invalidation, refresh, and rollback surfaces"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 3,
    title: "Structured Form Model",
    purpose: "Show what signals.form(...) buys you in a simple but real form.",
    preface: "This is like React Hook Form plus your query-backed policy checks plus your submit-disable logic, but one controller owns the result. Edit the product fields and watch dirty state, backend-dependent validation, reset posture, and submit readiness move together instead of being recomputed in component glue.",
    difficulty: "Beginner",
    primaryMessage: "Forge forms are not just fields and validation wiring; they expose a full form model.",
    forgeCode: `const articleFields = signals.resource.detailFields({
  title: {
    read: (value) => value.title,
    write: (value, title) => ({ ...value, title }),
  },
  status: {
    read: (value) => value.status,
    write: (value, status) => ({ ...value, status }),
  },
});

const articleDetail = api.url("/articles/:articleId").detail({
  reconcile: articleFields,
  load: ({ articleId }) => fetchArticle(articleId),
});

const form = signals.form({
  source: signals.form.source.resourceLine(articleDetail.line({ articleId: "article-12" }), {
    id: "article-form",
  }),
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      resourceEffectProfile: signals.resource.effects.branchNative(),
    }),
  }),
});`,
    alternativeName: "React Hook Form + React Query + Zod",
    alternativeCode: `const query = useQuery({
  queryKey: ["article", articleId],
  queryFn: () => fetchArticle(articleId),
});

const form = useForm({
  values: query.data,
  resolver: zodResolver(schema),
});

const onSubmit = form.handleSubmit(async (values) => {
  await patchArticle(articleId, values);
  queryClient.setQueryData(["article", articleId], values);
});`,
    explanationAlternative: "You assemble separate layers for source loading, validation, form state, submit wiring, and post-submit cache reconciliation. Each layer works, but the shape is spread across multiple libraries and app glue.",
    explanationForge: "Forge keeps source truth, draft truth, validation, readiness, and resource-backed submit planning on one form surface. The submit path can be inspected through the same controller instead of being split across fetch, form, and cache code.",
    whatYouGet: [
      "Unified source, draft, and effective value layers",
      "Built-in semantic dirty diffing",
      "Reactive validation and readiness blockers",
      "Resource-backed submit planning and execution"
    ],
    relatedDocsPath: "forms/index"
  },
  {
    id: 4,
    title: "Navigation Authority",
    purpose: "Show that Forge routing can drive a real commerce admin with role-based page access, breadcrumb policy, retained history, and replayable session outcomes.",
    preface: "This is routing as an admission pipeline, not just path matching. Switch roles and choose routes: the router admits, redirects, or denies the browser ingress, warms route-local resources, and records a story you can replay under another role instead of inventing a separate guard/history adapter.",
    difficulty: "Intermediate",
    primaryMessage: "Forge routing owns richer navigation truth than standard route libraries.",
    forgeCode: `const routes = signals.router.define({
  home: signals.router.route("/"),
  sales: signals.router.route("/sales", {
    breadcrumb: signals.router.breadcrumb({
      id: "sales",
      label: "Sales Stats",
    }),
  }),
  catalog: signals.router.route("/products", {
    breadcrumb: signals.router.breadcrumb({
      id: "catalog",
      label: "Product Catalog",
    }),
  }),
  editProduct: signals.router.route("/products/:productId/edit", {
    breadcrumb: signals.router.breadcrumb({
      id: "edit-product",
      label: "Edit Product",
      parent: signals.router.breadcrumbParent({
        carry: true,
        fallback: signals.router.breadcrumbTrail([
          signals.router.breadcrumbEntry({
            id: "catalog",
            label: "Product Catalog",
            target: "/products",
          }),
          signals.router.breadcrumbEntry({
            id: "product-parent",
            label: "Trailblazer Jacket",
            target: ({ href }) => href.replace("/edit", ""),
          }),
        ]),
      }),
    }),
  }),
  restricted: signals.router.route("/permission-denied"),
});

const story = signals.router.browserHistory.story();
const editRef = routes.editProduct.to({ params: { productId: "p-118" } });
const allowedHref = canEdit ? editRef.href : routes.restricted.to({}).href;
const ingress = signals.router.browserHistory.push(allowedHref);
const report = await routes.admitBrowserHistoryIngress(ingress);

story.record(report);

console.log(editRef.href);
console.log(report.diagnostics().routeId);
console.log(story.breadcrumbTrail().entries.map((entry) => entry.label));
console.log(story.admittedEntries().map((entry) => entry.routeId));`,
    alternativeName: "React Router v6",
    alternativeCode: `import { BrowserRouter, Routes, Route, Link } from "react-router-dom";

// Standard string wiring
<Link to={\`/items/\${itemId}\`}>Item Details</Link>

// Separate libraries needed to construct breadcrumbs
const breadcrumbs = [
  { path: "/", label: "Home" },
  { path: \`/items/\${itemId}\`, label: \`Item \${itemId}\` }
];`,
    explanationAlternative: "Ordinary router demos stop at matching and component switching. Once you need route-owned breadcrumbs, retained session history, and role-based outcomes that can be replayed across a real admin flow, the app usually grows its own navigation framework beside the router.",
    explanationForge: "Forge lets the router own more than path matching. Route references generate hrefs, browser-history ingress admits boundary truth, breadcrumb policy is declared on routes, and the same admitted session can be replayed after role-based access decisions change which pages the user can actually reach.",
    whatYouGet: [
      "Typed page references for sales, catalog, add, and edit routes",
      "Role-sensitive route outcomes and permission redirects",
      "Router-owned breadcrumb policy and entry provenance",
      "Retained browser-history story entries",
      "Navigation verification and projection tools",
      "Replayable routed session outcomes by user type"
    ],
    relatedDocsPath: "router/index"
  },
  {
    id: 5,
    title: "Optimistic Resource Writes",
    purpose: "Show what Forge resources provide when a write changes visible server truth.",
    preface: "Both sides do the same optimistic add. The TanStack side is the normal callback model: snapshot, patch cache, confirm, rollback, toast. The Forge side treats the optimistic insert as a resource-line effect, so the interesting output is not just whether the mutation succeeded; it is lifecycle, rollback posture, diagnostics, and feedback you can inspect afterward.",
    difficulty: "Intermediate",
    primaryMessage: "Forge optimistic writes are branch-native, inspectable, and rollback-aware.",
    forgeCode: `const api = signals.api({ baseUrl: "/api" });

const taskDetail = api.url("/tasks/:taskId").detail({
  load: ({ taskId }) => fetchTask(taskId),
});

// Compile a single resource line instance
const line = taskDetail.line({ taskId: "t-4" });

console.log(line.summary()); // loading -> settled
console.log(line.value());   // raw data`,
    alternativeName: "React Query (TanStack Query)",
    alternativeCode: `import { useQuery } from "@tanstack/react-query";

const { data, isLoading, refetch } = useQuery({
  queryKey: ["task", taskId],
  queryFn: () => fetchTask(taskId),
});

// Manual synchronization blocks needed to patches
// or reconcile updates into the local query caches`,
    explanationAlternative: "Operates as a generic data-fetching store. Requires separate manual mutations, cache-patching scripts, and event listeners to keep loaded elements in sync.",
    explanationForge: "Materializes a specific Resource Line. Supports built-in loading/settled wrappers, automatic mutation response reconciliation, and local optimistic patching profiles.",
    whatYouGet: [
      "Dynamic line-level data cache handles",
      "Reactive loading/settled state objects",
      "Optimistic write patches",
      "Reconciliation and fallback support"
    ],
    relatedDocsPath: "resources/index"
  },
  {
    id: 6,
    title: "Route-Coupled Resource Form",
    purpose: "Show the first stacked composed example linking routes, resources, and forms.",
    preface: "This is the adapter-tax demo. TanStack, Formik, and a router are not the problem; the problem is the layer you write to translate query status into form status, mutation status into route-leave rules, server results into cache patches, and lifecycle events into toasts. The Forge block shows the same workflow when those contracts already line up.",
    difficulty: "Advanced",
    primaryMessage: "Forge primitives compose into workflows without requiring a separate orchestration layer.",
    forgeCode: `// Combine Routing + Resources + Forms in a single flow
const routes = signals.router.define({
  detail: signals.router.route("/tasks/:taskId"),
  edit: signals.router.route("/tasks/:taskId/edit")
});

const taskDetail = api.url("/tasks/:taskId").detail({
  load: ({ taskId }) => fetchTask(taskId)
});

// Backing form directly with the resource line
const form = signals.form({
  source: taskDetail.line({ taskId }).toSource(),
  fields: ({ field }) => ({
    title: field("title"),
    status: field("status")
  })
});`,
    alternativeName: "React Router + Formik + React Query",
    alternativeCode: `// Requires complex useEffect chains to map query fields
// into Formik initial values, along with search param triggers
useEffect(() => {
  if (query.data) {
    formik.setValues(query.data);
  }
}, [query.data]);

// Dynamic route checking needed to prevent unsaved changes`,
    explanationAlternative: "High coordination debt. Demands fragile useEffect syncer loops, manual dirty caches inside routers to blocks exits, and context bridging.",
    explanationForge: "Primitives align naturally. Form binds natively to Resource Lines, and the route admission process checks form.readiness to intercept unsaved departures.",
    whatYouGet: [
      "Zero-glue form + resource + router bindings",
      "Auto-warming resource lines on route transition",
      "Preserved draft continuity between routes",
      "Dynamic route exit validation guards"
    ],
    relatedDocsPath: "forms/route-coupling/route-authority-handoff"
  }
];
