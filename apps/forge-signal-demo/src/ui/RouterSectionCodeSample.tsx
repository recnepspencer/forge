function CodeLine({ children }: { children?: any }) {
  return <div className="router-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: any;
}) {
  return <span className={`router-tok router-tok-${kind}`}>{children}</span>;
}

export function RouterSectionCodeSample() {
  return (
    <div className="router-code-block" role="presentation">
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">api</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="fn">api</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="prop">baseUrl</Tok>: <Tok kind="str">"/router-demo-api"</Tok>,
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="prop">effects</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">resource</Tok>.<Tok kind="prop">effects</Tok>.<Tok kind="fn">branchNative</Tok>(),
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">resources</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useStorefrontRouteResources</Tok>(<Tok kind="var">api</Tok>);
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">admission</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useRouteAdmissionPolicies</Tok>(<Tok kind="plain">signals</Tok>);
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">routePage</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useIntentRouteResource</Tok>(<Tok kind="plain">signals</Tok>);
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">routes</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">define</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="prop">catalog</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(<Tok kind="str">"/catalog"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"} <Tok kind="prop">page</Tok>: <Tok kind="fn">routePage</Tok>(<Tok kind="var">resources</Tok>.<Tok kind="prop">catalog</Tok>) {"}"},
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="prop">orderDetails</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(<Tok kind="str">"/orders/:orderId"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">admission</Tok>: [<Tok kind="var">admission</Tok>.<Tok kind="prop">requiresSession</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"}
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">page</Tok>: <Tok kind="fn">routePage</Tok>(<Tok kind="var">resources</Tok>.<Tok kind="prop">orderDetails</Tok>, ({ "{" } <Tok kind="var">params</Tok> { "}" }) <Tok kind="op">=&gt;</Tok> ({"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">orderId</Tok>: <Tok kind="var">params</Tok>.<Tok kind="prop">orderId</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"})),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">{"},"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="prop">adminProducts</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(<Tok kind="str">"/admin/products"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">admission</Tok>: [<Tok kind="var">admission</Tok>.<Tok kind="prop">requiresSession</Tok>, <Tok kind="var">admission</Tok>.<Tok kind="prop">adminOnly</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"} <Tok kind="prop">page</Tok>: <Tok kind="fn">routePage</Tok>(<Tok kind="var">resources</Tok>.<Tok kind="prop">adminProducts</Tok>) {"}"},
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="prop">revenueReport</Tok>: <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="fn">route</Tok>(<Tok kind="str">"/reports/revenue"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">admission</Tok>: [<Tok kind="var">admission</Tok>.<Tok kind="prop">requiresSession</Tok>, <Tok kind="var">admission</Tok>.<Tok kind="prop">adminOnly</Tok>],
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"} <Tok kind="prop">page</Tok>: <Tok kind="fn">routePage</Tok>(<Tok kind="var">resources</Tok>.<Tok kind="prop">revenueReport</Tok>) {"}"},
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">ingress</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="prop">browserHistory</Tok>.<Tok kind="fn">push</Tok>(<Tok kind="str">"/admin/products"</Tok>);
      </CodeLine>
      <CodeLine>
        <Tok kind="kw">const</Tok> <Tok kind="var">report</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="kw">await</Tok> <Tok kind="var">routes</Tok>.<Tok kind="fn">admitBrowserHistoryIngress</Tok>(<Tok kind="var">ingress</Tok>, {"{"} <Tok kind="prop">role</Tok>: <Tok kind="str">"admin"</Tok> {"}"});
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">signals</Tok>.<Tok kind="prop">router</Tok>.<Tok kind="prop">browserHistory</Tok>.<Tok kind="fn">story</Tok>().<Tok kind="fn">record</Tok>(<Tok kind="var">report</Tok>);
      </CodeLine>
    </div>
  );
}
