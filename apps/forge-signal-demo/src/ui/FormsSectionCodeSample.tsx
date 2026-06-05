function CodeLine({ children }: { children?: any }) {
  return <div className="forms-code-line">{children}</div>;
}

function Tok({
  kind,
  children,
}: {
  kind: "kw" | "fn" | "var" | "str" | "prop" | "op" | "plain";
  children: any;
}) {
  return <span className={`forms-tok forms-tok-${kind}`}>{children}</span>;
}

export function FormsSectionCodeSample() {
  return (
    <div className="forms-code-block" role="presentation">
      <CodeLine>
        <Tok kind="kw">export function</Tok> <Tok kind="fn">RolloutSettingsForm</Tok>() <Tok kind="plain">{"{"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">rolloutSource</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceLine</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">rollout</Tok>());
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">countryApproval</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceLine</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">countryApproval</Tok>());
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">saveRollout</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useResourceOperation</Tok>(<Tok kind="plain">resources</Tok>.<Tok kind="fn">saveRollout</Tok>());
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">validators</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useProductFormValidators</Tok>({"{"} <Tok kind="var">countryApproval</Tok> {"}"});
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="plain">{`// Saved source data and backend truth feed one form controller.`}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="kw">const</Tok> <Tok kind="var">form</Tok> <Tok kind="op">=</Tok>{" "}
        <Tok kind="fn">useSignalsForm</Tok>({"{"}
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">source</Tok>: <Tok kind="var">rolloutSource</Tok>.<Tok kind="prop">value</Tok> <Tok kind="op">??</Tok>{" "}
        <Tok kind="var">EMPTY_ROLLOUT</Tok>,
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">resources</Tok>: {"{"} <Tok kind="prop">countryApproval</Tok>: <Tok kind="var">countryApproval</Tok> {"}"},
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">fields</Tok>: ({ "{" } <Tok kind="var">field</Tok> { "}" }) <Tok kind="op">=&gt;</Tok> ({"{"}
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">price</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"price"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: <Tok kind="var">validators</Tok>.<Tok kind="prop">price</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">targetMargin</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"targetMargin"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">readOnly</Tok>: <Tok kind="plain">true</Tok>,
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="plain">{`// backend policy field: price validation reads this margin floor`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">shippingRegions</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"shippingRegions"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: <Tok kind="var">validators</Tok>.<Tok kind="prop">shippingRegions</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">carrierEmails</Tok>: <Tok kind="var">field</Tok>(<Tok kind="str">"carrierEmails"</Tok>, {"{"}
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="prop">validate</Tok>: <Tok kind="var">validators</Tok>.<Tok kind="prop">carrierEmails</Tok>,
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="prop">actions</Tok>: ({ "{" } <Tok kind="var">resourceAction</Tok> { "}" }) <Tok kind="op">=&gt;</Tok> ({"{"}
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="prop">save</Tok>: <Tok kind="var">resourceAction</Tok>(<Tok kind="str">"save"</Tok>, <Tok kind="var">saveRollout</Tok>),
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">{"}),"}</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{"});"}</Tok>
      </CodeLine>
      <CodeLine />
      <CodeLine>
        {"  "}
        <Tok kind="kw">return</Tok> (
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">form</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Retail price"</Tok> <Tok kind="plain">{`{...form.field("price")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Target margin"</Tok> <Tok kind="plain">{`{...form.field("targetMargin")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">MultiSelectDropdown</Tok> <Tok kind="prop">label</Tok>=<Tok kind="str">"Shipping regions"</Tok> <Tok kind="plain">{`{...form.field("shippingRegions")}`}</Tok> <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{form.draft.shippingRegions.map((code) => (`}</Tok>
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">TextField</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="prop">key</Tok>=<Tok kind="plain">{`{code}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="prop">label</Tok>=<Tok kind="plain">{`{\`${"${code}"} carrier email\`}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"          "}
        <Tok kind="plain">{`{...form.field("carrierEmails").item(code)}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"        "}
        <Tok kind="plain">/&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`))}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{/* reset disables when the form is already back at source */}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">button</Tok> <Tok kind="prop">type</Tok>=<Tok kind="str">"button"</Tok> <Tok kind="prop">disabled</Tok>=<Tok kind="plain">{`{!form.dirty().isDirty}`}</Tok> <Tok kind="prop">onClick</Tok>=<Tok kind="plain">{`{form.reset}`}</Tok><Tok kind="plain">&gt;</Tok>Reset<Tok kind="plain">&lt;/</Tok><Tok kind="fn">button</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">{`{/* save auto-disables until price, approvals, and carrier emails all pass */}`}</Tok>
      </CodeLine>
      <CodeLine>
        {"      "}
        <Tok kind="plain">&lt;</Tok><Tok kind="fn">button</Tok> <Tok kind="prop">disabled</Tok>=<Tok kind="plain">{`{form.action("save").disabled}`}</Tok><Tok kind="plain">&gt;</Tok>Save changes<Tok kind="plain">&lt;/</Tok><Tok kind="fn">button</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"    "}
        <Tok kind="plain">&lt;/</Tok><Tok kind="fn">form</Tok><Tok kind="plain">&gt;</Tok>
      </CodeLine>
      <CodeLine>
        {"  "}
        <Tok kind="plain">{" );"}</Tok>
      </CodeLine>
      <CodeLine>
        <Tok kind="plain">{"}"}</Tok>
      </CodeLine>
    </div>
  );
}
