# Text platform

WORTH UI qualifies text once and carries the same typed layout through measurement,
accessibility geometry, headless presentation, native rendering, and reconstruction. Text
consumers do not choose fonts, reshape source strings, or ask the operating system for a
substitute font.

## The model

A text layout is produced from three owned inputs:

1. a qualified global font collection;
2. an admitted UTF-8 paragraph with explicit constraints and style spans; and
3. explicit profile, font-collection, and text-scale generations.

`qualify_text_layout` performs admission, Unicode 17 analysis, whole-cluster fallback,
OpenType shaping, bidi visual ordering, line fitting, and interaction recording. Its result,
`UiQualifiedTextLayout`, owns the selected font-byte identity and immutable layout artifact.
Hosts receive borrowed views of that artifact. This is why measurement, caret geometry,
accessibility, headless output, native rendering, and reconstruction cannot silently disagree.

## Bring your own fonts

Start from `UiGlobalFontCollection::admit_qualified_profile()`, then register one or more
application packs with `register_application_pack`. A pack may contain multiple families and
multiple faces per family. The application owns every supplied byte.

Accepted containers are TTF, OTF, TTC, and OTC. Set `face_index` for a collection face.
WOFF, WOFF2, malformed fonts, unsupported color tables, fonts beyond the admitted byte and
face capacities, and fonts without qualified license metadata are rejected before publication.
WORTH UI never searches system fonts as a fallback.

Each face declares its family, static weight, width, slant, and license record. Admission
derives and freezes its name records, variable axes, OpenType feature inventory, Unicode
coverage, color-glyph posture, glyph-expansion bound, and exact font-byte digest. A text span
then supplies:

- an ordered `UiFontFamilyStack`;
- a `UiTextFaceRequest` for weight, width, and upright/italic/oblique slant;
- optional `UiFontVariationCoordinate` values such as `wght` or `wdth`; and
- optional `UiOpenTypeFeature` values such as `liga`.

The resolver considers one best face per authored family in stack order. Registration order is
not a selection rule. Unsupported explicit features, out-of-range axes, foreign family
receipts, and stale collection authority are typed denials.

## Fallback and emoji

Fallback is deterministic and atomic at the shaped cluster boundary. WORTH UI never splits a
cluster merely to mix fonts. The route is:

- the span's ordered application family stack;
- qualified profile defaults for ordinary text;
- the qualified color-emoji face for Unicode 17 RGI emoji sequences; and
- Last Resort when no qualified face covers the whole cluster.

The frozen Unicode 17 corpus includes grapheme, word, line-break, bidi, variation-selector,
and all 3,953 RGI emoji sequences. ZWJ families, flags, keycaps, skin-tone modifiers, tag
sequences, and VS15/VS16 stay atomic. Emoji selection carries the same exact face and font-byte
identity into shaping, measurement, rendering, and reconstruction.

## Generations and replacement

Register, replace, or remove a pack by consuming the current collection and naming the exact
successor generation. A predecessor collection remains usable only by layouts and
reconstruction sources that already pin it. It cannot qualify new text after its successor is
published.

This separation is intentional:

- fresh qualification requires the current admission lease;
- existing layouts keep their original `Arc`-owned font bytes and identities;
- replacing or removing a pack does not mutate an existing layout; and
- reconstruction of an existing layout uses its original collection, not the newest pack.

Do not reuse a numeric generation as collection identity. Layout requests carry the exact
collection lineage, so two independent collections with the same generation number cannot be
substituted.

## Paragraphs, spans, and language

`UiTextParagraphConstraints` defines the BCP-47 language (or `und`), base direction, wrapping,
alignment, overflow policy, font size, width, line height, letter and word spacing, tab
interval, and line limit. Every byte of a nonempty paragraph must be covered exactly once by
ordered, nonoverlapping `UiTextStyleSpan` ranges on UTF-8 boundaries.

Style-span ranges, caret positions, hit results, and selection ranges refer to original UTF-8
byte boundaries. Selection cannot bisect a grapheme or shaped cluster. Bidi carets retain their
visual edge and upstream/downstream affinity, and a logical selection may produce multiple
visual rectangles. Empty paragraphs and trailing hard-break lines still have canonical line
and caret anchors even though the break itself paints no glyph.

Each qualified layout records paragraph, line, run, cluster, and glyph logical geometry plus
font-derived ink bounds. Italic overhangs and displaced combining marks therefore remain
measurable even when their ink extends beyond advances or line logical boxes.

## Capacity and cost

Admission reserves conservative bounds before analysis or shaping. The qualified profile caps
source bytes, style spans, families, faces, application font bytes, graphemes, shaping runs,
glyph expansion, lines, and retained layouts. A pack's derived shaping expansion participates
in that preflight; an application font cannot defer an unbounded expansion until shaping.

Ordinary content-only and width-only changes are paragraph-local. Unchanged sibling layouts
are reused by identity, and their text work is zero. Changing language, family stack, selected
face, feature values, variation coordinates, scale generation, or other layout-affecting
constraints invalidates only the affected paragraph unless the authored constraint itself is
global. Reconstruction is a named, separately costed lane.

## Measurement and accessibility

Use the layout's lines, glyphs, logical bounds, ink bounds, caret stops, hit testing, and
selection rectangles. Headless measurement and accessibility geometry consume the same
borrowed layout identity as presentation. They must not infer geometry from the source string
or invoke a font library independently.

Color is a paint-only property. A color-only update may reuse layout only when the exact font
collection lineage, layout-affecting style, and layer order are unchanged.

## DPI, text scale, and raster generations

DPI scale and framework text scale are different authorities. A pure monitor-DPI change keeps
the logical layout identity, line breaks, original UTF-8 ranges, and caret geometry. It replaces
only the device-pixel raster and atlas generations derived from that layout. A framework text-scale
change is layout-affecting: qualify a successor layout before presentation because advances, line
breaks, ink bounds, hit geometry, and selection rectangles may all change.

The same rule applies to width. Changing one paragraph's authored width creates a successor layout
for that paragraph; it does not authorize reshaping unchanged siblings. Do not multiply a DPI scale
into the authored font size and then treat the result as a new logical layout—that would make a
window move between monitors change measurement and accessibility identity.

## Raster and atlas lifecycle

Phase 4 freezes the handoff to native rasterization; it does not let layout or a host widget own a
GPU atlas. The native renderer owns separate bounded alpha and color atlases. The qualified profile
admits at most four 1024×1024 alpha pages, two 2048×2048 color pages, 8,192 entries, 36 MiB of atlas
texels, and an 8 MiB staged-upload budget. An atlas key includes the font-collection and profile
generations, exact face and glyph, variation coordinates, palette, size, raster source, DPI scale,
and fractional origin.

Live layout and presentation work pin their entries. Eviction may choose only unpinned entries;
saturation is a typed denial and never overwrites a live glyph. Reconstruction consumes the same
qualified layout and retained font bytes, then rebuilds missing raster/atlas state under a new raster
generation. This boundary is where Phase 5 will add deterministic outline, COLR, bitmap, and emoji
raster output without changing Phase 4 font selection or layout identity.

## Compiling example

The repository example registers an owned variable application font, selects it through an
ordered family stack, enables `liga`, requests a `wght` coordinate, and qualifies mixed Latin,
Arabic, and emoji text. Emoji falls back as a whole cluster to the qualified color-emoji face.

```powershell
cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui --example text_platform
```

See [text_platform.rs](../crates/worth-ui/examples/text_platform.rs) for the complete compiling
program. In an application, use your own licensed `Arc<[u8]>` values rather than embedding the
profile fixture used by the example.

## Failure guidance

- `UiFontCollectionAdmissionDenial` means the profile, pack, face metadata, container, byte
  budget, color tables, feature/axis metadata, generation, or license record was not admitted.
- `UiTextQualificationDenial::Admission` means the paragraph, spans, generations, or derived
  capacity were rejected before text work.
- `Fallback`, `Shaping`, and `Layout` denials identify the exact later effect-free staging
  boundary; no partially qualified layout is published.
- A stale collection denial should be resolved by qualifying new text against the current
  collection. Keep the predecessor collection only for already-qualified layout or
  reconstruction authority.

Do not recover from a denial by asking a host widget or OS API to render the raw text. That
creates a second font, shaping, measurement, and accessibility authority and is outside the
WORTH UI text contract.
