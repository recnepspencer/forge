function CodeLine({ children }: { children?: any }) {
  return <div className="resource-lines-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: any;
}) {
  return <span className={`resource-lines-tok resource-lines-tok-${kind}`}>{children}</span>;
}

export function ResourceLinesSectionCodeSample() {
  return (
    <div className="resource-lines-code-card">
      <div className="forms-card-topline"><span>Resource authoring</span></div>
      <h3>Declare response shape, then materialize lines.</h3>
      <div className="resource-lines-code-block" role="presentation">
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">api</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="plain">signals</Tok>.<Tok kind="fn">api</Tok>({"{"}
        </CodeLine>
        <CodeLine>
          {"  "}
          <Tok kind="prop">baseUrl</Tok>: <Tok kind="str">"/api"</Tok>,
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
          <Tok kind="kw">const</Tok> <Tok kind="var">productDetail</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="plain">signals</Tok>.<Tok kind="prop">resource</Tok>.<Tok kind="prop">response</Tok>.<Tok kind="fn">detail</Tok>&lt;<Tok kind="plain">Product</Tok>&gt;()({"{"}
        </CodeLine>
        <CodeLine>
          {"  "}
          <Tok kind="prop">status</Tok>: <Tok kind="str">"status"</Tok>,
        </CodeLine>
        <CodeLine>
          {"  "}
          <Tok kind="prop">price</Tok>: <Tok kind="str">"price"</Tok>,
        </CodeLine>
        <CodeLine>
          {"  "}
          <Tok kind="prop">inventory</Tok>: <Tok kind="str">"inventory"</Tok>,
        </CodeLine>
        <CodeLine>
          <Tok kind="plain">{"});"}</Tok>
        </CodeLine>
        <CodeLine />
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">products</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="var">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/products"</Tok>)
        </CodeLine>
        <CodeLine>
          {"  "}.<Tok kind="fn">response</Tok>(<Tok kind="plain">signals</Tok>.<Tok kind="prop">resource</Tok>.<Tok kind="prop">response</Tok>.<Tok kind="fn">array</Tok>&lt;<Tok kind="plain">Product</Tok>&gt;()({"{"}
        </CodeLine>
        <CodeLine>
          {"    "}
          <Tok kind="prop">id</Tok>: <Tok kind="str">"id"</Tok>,
        </CodeLine>
        <CodeLine>
          {"    "}
          <Tok kind="prop">summary</Tok>: {"{"} <Tok kind="prop">total</Tok>: <Tok kind="str">"total"</Tok>, <Tok kind="prop">lowStock</Tok>: <Tok kind="str">"lowStock"</Tok> {"}"},
        </CodeLine>
        <CodeLine>
          {"  "}
          <Tok kind="plain">{"}))"}</Tok>
        </CodeLine>
        <CodeLine>
          {"  "}.<Tok kind="fn">list</Tok>({"{"} <Tok kind="prop">load</Tok>: <Tok kind="var">loadProducts</Tok> {"}"});
        </CodeLine>
        <CodeLine />
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">product</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="var">api</Tok>.<Tok kind="fn">url</Tok>(<Tok kind="str">"/products/:productId"</Tok>)
        </CodeLine>
        <CodeLine>
          {"  "}.<Tok kind="fn">response</Tok>(<Tok kind="var">productDetail</Tok>)
        </CodeLine>
        <CodeLine>
          {"  "}.<Tok kind="fn">detail</Tok>({"{"} <Tok kind="prop">load</Tok>: <Tok kind="var">loadProduct</Tok> {"}"});
        </CodeLine>
        <CodeLine />
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">catalogLine</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="var">products</Tok>.<Tok kind="fn">line</Tok>({"{}"});
        </CodeLine>
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">detailLine</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="var">product</Tok>.<Tok kind="fn">line</Tok>({"{"} <Tok kind="prop">productId</Tok> {"}"});
        </CodeLine>
        <CodeLine />
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">catalog</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="fn">useResourceLine</Tok>(<Tok kind="var">catalogLine</Tok>, <Tok kind="var">store</Tok>);
        </CodeLine>
        <CodeLine>
          <Tok kind="kw">const</Tok> <Tok kind="var">detail</Tok> <Tok kind="op">=</Tok>{" "}
          <Tok kind="fn">useResourceLine</Tok>(<Tok kind="var">detailLine</Tok>, <Tok kind="var">store</Tok>);
        </CodeLine>
        <CodeLine />
        <CodeLine>
          <Tok kind="kw">await</Tok> <Tok kind="var">catalogLine</Tok>.<Tok kind="fn">refresh</Tok>();
        </CodeLine>
        <CodeLine>
          <Tok kind="var">detailLine</Tok>.<Tok kind="fn">invalidate</Tok>();
        </CodeLine>
        <CodeLine>
          <Tok kind="fn">console</Tok>.<Tok kind="fn">log</Tok>(<Tok kind="var">catalog</Tok>.<Tok kind="fn">status</Tok>().<Tok kind="prop">kind</Tok>);
        </CodeLine>
        <CodeLine>
          <Tok kind="fn">console</Tok>.<Tok kind="fn">log</Tok>(<Tok kind="var">catalog</Tok>.<Tok kind="fn">freshness</Tok>().<Tok kind="prop">kind</Tok>);
        </CodeLine>
        <CodeLine>
          <Tok kind="fn">console</Tok>.<Tok kind="fn">log</Tok>(<Tok kind="var">catalogLine</Tok>.<Tok kind="fn">diagnostics</Tok>().<Tok kind="prop">lastEffect</Tok>);
        </CodeLine>
      </div>
    </div>
  );
}
