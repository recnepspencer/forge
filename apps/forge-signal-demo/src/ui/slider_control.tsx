import { useEffect, useState } from "react";

export function Slider({
  label,
  value,
  min,
  max,
  step,
  onChange,
  fmt,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  onChange: (v: number) => void;
  fmt?: (v: number) => string;
}) {
  const [draftValue, setDraftValue] = useState(value);

  useEffect(() => {
    setDraftValue(value);
  }, [value]);

  return (
    <label className="slider">
      <span className="slider__head">
        <span>{label}</span>
        <span>{fmt ? fmt(draftValue) : draftValue.toFixed(2)}</span>
      </span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={draftValue}
        onInput={(e) => {
          const next = Number((e.target as HTMLInputElement).value);
          setDraftValue(next);
          onChange(next);
        }}
        onChange={() => {
          // onInput drives live updates; keep onChange attached for browser compatibility.
        }}
      />
    </label>
  );
}
