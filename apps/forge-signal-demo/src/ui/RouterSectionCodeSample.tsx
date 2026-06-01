import React from "react";

function CodeLine({ children }: { children?: React.ReactNode }): React.ReactElement {
  return <div className="router-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: React.ReactNode;
}): React.ReactElement {
  return <span className={`router-tok router-tok-${kind}`}>{children}</span>;
}

export function RouterSectionCodeSample(): React.ReactElement {
  return (
    <div className="router-code-block" role="presentation">
      <CodeLine>
        <Tok kind="plain">{`// Reuse one stable() policy across these route resources: stay fresh until something explicitly invalidates the line.`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">defaultPolicy</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">resourcePolicyProfiles</Tok>.<Tok kind="fn">stable</Tok>();
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="plain">{`// Reconcile teaches the catalog family how to update item rows without replacing the whole payload.`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">catalogFamily</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/catalog"</Tok>)
      </CodeLine>
      <CodeLine>
        {"  "}.<Tok kind="fn">items</Tok>((<Tok kind="var">item</Tok>) <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="var">item</Tok>.<Tok kind="prop">name</Tok>)
      </CodeLine>
      <CodeLine>
        {"  "}.<Tok kind="fn">reconcile</Tok>(
        (<Tok kind="var">value</Tok>) <Tok kind="op">=&gt;</Tok> <Tok kind="var">value</Tok>.<Tok kind="prop">items</Tok>,
      </CodeLine>
      <CodeLine>
        {"    "}
        (<Tok kind="var">value</Tok>, <Tok kind="var">nextItems</Tok>) <Tok kind="op">=&gt;</Tok> ({"{"}
        ...<Tok kind="var">value</Tok>, <Tok kind="prop">items</Tok>: [...<Tok kind="var">nextItems</Tok>] {"}"}),
      </CodeLine>
      <CodeLine>
        {"  "})
      </CodeLine>
      <CodeLine>
        {"  "}.<Tok kind="fn">list</Tok>({"{"} <Tok kind="prop">policy</Tok>: <Tok kind="var">defaultPolicy</Tok>,{" "}
        <Tok kind="prop">load</Tok>: <Tok kind="kw">async</Tok> () <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">loadCatalogJson</Tok>() {"}"});
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">orderFamily</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/orders/:orderId"</Tok>).<Tok kind="fn">detail</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">policy</Tok>: <Tok kind="var">defaultPolicy</Tok>,
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">load</Tok>: <Tok kind="kw">async</Tok> ({ "{" } <Tok kind="var">orderId</Tok> { "}" }){" "}
        <Tok kind="op">=&gt;</Tok> <Tok kind="fn">loadOrderJson</Tok>(<Tok kind="var">orderId</Tok>),
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">adminProductsFamily</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/admin/products"</Tok>).<Tok kind="fn">detail</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">policy</Tok>: <Tok kind="var">defaultPolicy</Tok>,
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">load</Tok>: <Tok kind="kw">async</Tok> () <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">loadAdminProductsJson</Tok>(),
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">revenueReportFamily</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/reports/revenue"</Tok>).<Tok kind="fn">detail</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">policy</Tok>: <Tok kind="var">defaultPolicy</Tok>,
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">load</Tok>: <Tok kind="kw">async</Tok> () <Tok kind="op">=&gt;</Tok>{" "}
        <Tok kind="fn">loadRevenueJson</Tok>(),
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">requiresSession</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">prerequisite</Tok>(
        <Tok kind="str">"requiresSession"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">evaluate</Tok>: ({ "{" } <Tok kind="var">facts</Tok>, <Tok kind="var">redirect</Tok> { "}" }){" "}
        <Tok kind="op">=&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="var">facts</Tok>.<Tok kind="prop">role</Tok> <Tok kind="op">===</Tok>{" "}
        <Tok kind="str">"loggedOut"</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="op">?</Tok> <Tok kind="var">redirect</Tok>({"{"} <Tok kind="prop">href</Tok>:{" "}
        <Tok kind="str">"/sign-in"</Tok>, <Tok kind="prop">reason</Tok>: <Tok kind="str">"signInRequired"</Tok> {"}"})
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="op">:</Tok> <Tok kind="plain">true</Tok>,
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">adminOnly</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">prerequisite</Tok>(
        <Tok kind="str">"adminOnly"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">evaluate</Tok>: ({ "{" } <Tok kind="var">facts</Tok>, <Tok kind="var">forbidden</Tok> { "}" }){" "}
        <Tok kind="op">=&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="var">facts</Tok>.<Tok kind="prop">role</Tok> <Tok kind="op">===</Tok>{" "}
        <Tok kind="str">"admin"</Tok> <Tok kind="op">?</Tok> <Tok kind="plain">true</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="op">:</Tok> <Tok kind="var">forbidden</Tok>({"{"} <Tok kind="prop">reason</Tok>:{" "}
        <Tok kind="str">"adminRoleRequired"</Tok> {"}"}),
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="plain">{`// "intent" means start warming the route resource when navigation looks likely,`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{`// like a click or transition request, not only after the page is already visible.`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">routes</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">define</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">orderDetails</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(
        <Tok kind="str">"/orders/:orderId"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">admission</Tok>: [<Tok kind="var">requiresSession</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">resources</Tok>: {"{"}
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="prop">page</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">resourceLine</Tok>(
        <Tok kind="var">orderFamily</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">params</Tok>: ({ "{" } <Tok kind="var">params</Tok> { "}" }){" "}
        <Tok kind="op">=&gt;</Tok> ({"{"} <Tok kind="prop">orderId</Tok>: <Tok kind="var">params</Tok>.<Tok kind="prop">orderId</Tok> {"}"}),
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">prefetch</Tok>: <Tok kind="str">"intent"</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="plain">{"},"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="plain">{`}),`}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">adminProducts</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(
        <Tok kind="str">"/admin/products"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">admission</Tok>: [<Tok kind="var">requiresSession</Tok>, <Tok kind="var">adminOnly</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">resources</Tok>: {"{"}
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="prop">page</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">resourceLine</Tok>(
        <Tok kind="var">adminProductsFamily</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">params</Tok>: () <Tok kind="op">=&gt;</Tok> ({"{"}{"}"}),
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">prefetch</Tok>: <Tok kind="str">"intent"</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="plain">{"},"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="plain">{`}),`}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">revenueReport</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(
        <Tok kind="str">"/reports/revenue"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">admission</Tok>: [<Tok kind="var">requiresSession</Tok>, <Tok kind="var">adminOnly</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="prop">resources</Tok>: {"{"}
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="prop">page</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">resourceLine</Tok>(
        <Tok kind="var">revenueReportFamily</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">params</Tok>: () <Tok kind="op">=&gt;</Tok> ({"{"}{"}"}),
      </CodeLine>
      <CodeLine>
        {"        "}<Tok kind="prop">prefetch</Tok>: <Tok kind="str">"intent"</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}<Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}<Tok kind="plain">{"},"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="plain">{`}),`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="plain">{`// This wraps a browser-style navigation event so the router can admit it,`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{`// record it in history, and tell us whether the route was admitted, redirected, or denied.`}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">ingress</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="prop">browserHistory</Tok>.<Tok kind="fn">push</Tok>(
        <Tok kind="str">"/admin/products"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">routeIdentity</Tok>: <Tok kind="str">"router-section"</Tok>,
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">report</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="kw">await</Tok> <Tok kind="var">routes</Tok>.<Tok kind="fn">admitBrowserHistoryIngress</Tok>(
        <Tok kind="var">ingress</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"  "}<Tok kind="prop">role</Tok>: <Tok kind="str">"admin"</Tok>,
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
    </div>
  );
}
