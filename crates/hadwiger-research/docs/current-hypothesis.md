# Current Hypothesis

Status: active. Adopted 2026-06-12 after the Heule-510 criticality
reconnaissance (see `frontier-findings-heule-510-criticality.md`). This file
names the one research program the exploration loop is currently funding,
why it was chosen over the alternatives, and what would kill it.

## Primary Hypothesis (H-FRAC)

The fractional chromatic number of the plane satisfies `chi_f(R^2) > 4`,
and a finite exact-coordinate unit-distance witness family or geometric
fractional certificate for a bound strictly above 4 is reachable by
small-graph search driven from this pipeline.

Grounding: the lower bound moved from 3.6190 to 4 in 2023
(arXiv:2311.10069), against a standing upper bound of 4.3599. The
record-moving witness was a 27-vertex graph with geometric fractional
chromatic number 4, not an ordinary finite graph with `chi_f(G) >= 4`.
That distinction is now part of the route: ordinary finite-graph LP is a
screening and reproduction lane, while the frontier lane is geometric
fractional coloring with retained isometry/equality certificates. Small
certified computations are still winning in this lane; the gap is finite
and known. By contrast, the integral lanes are saturated: the 5-chromatic
vertex record (Parts, 509 vertices / 2442 edges, arXiv:2010.12665)
absorbed years of specialist effort for single-digit gains, and the k=5
forcing-gadget ladder toward `chi >= 6` has a near-zero expected yield at
reachable graph sizes (see Suppressed Directions).

New constraint from 2026-06-12 literature check: the 27-vertex witness lives
in the Moser lattice, and a June 2026 note proves geometric 4-colorings of
the entire Moser lattice and Moser ring. Therefore the Moser-lattice
neighborhood is a reproduction/suppression target, not an improvement
region. Any improvement search must either leave the Moser lattice/ring or
use a construction whose retained geometry is not covered by that coloring
result.

Falsifier / kill criterion: if the pipeline cannot first reproduce the
known `chi_f >= 4`-regime certificates on small witnesses, or if witness
perturbation and column-generation search stall at 4.0 with no improving
direction after a bounded search of the published witness family's
neighborhood, this hypothesis is retired with the search transcript as the
retained dead-end evidence.

## Why impact clears the bar

- Any certified `chi_f(R^2) >= 4 + epsilon` is a new true bound on a
  Hadwiger-Nelson quantity - published-result-level movement, not motif
  archaeology.
- The search space is tens of vertices with exact algebraic coordinates,
  not 500+; the pipeline's exact-geometry, LP screening (`good_lp` /
  `clarabel`), and SAT lanes are already built for exactly this shape.
- LP lower-bound certificates (feasible duals over independent sets) are
  small, replayable artifacts - they fit the crate's proof-carrying
  discipline natively, unlike multi-hundred-MB DRAT proofs.

## Program Phases

1. **P1 - in-pipeline reproduction.** Compute exact ordinary fractional
   chromatic bounds with retained certificates for the classical ladder
   (Moser spindle, Golomb graph, known small witnesses), then add the
   distinct geometric fractional lane needed to reproduce the 2311.10069
   27-vertex `chi_gf = 4` certificate. Exit: pipeline-certified
   ordinary/geometric fractional artifacts, with the Moser-lattice cap
   retained as suppression evidence.
2. **P2 - column-generation search engine.** Max-weight independent set
   oracle (SAT/ILP lane) + LP master problem over exact-embedding graphs,
   so `chi_f` of candidates with hundreds of vertices is computable with
   dual certificates.
3. **P3 - witness improvement search.** Perturb and extend the known
   witness family only through Moser-lattice escape moves: vertex additions
   in exact fields outside the retained Moser basis, gluing with
   high-`chi_f` cores outside the Moser ring, and spindle-style
   amplifications ranked by geometric dual slack. Every candidate enters
   through Query declarations and retains suppression records on failure.

## G27 pressure escape loop - 2026-06-12

The retained G27 geometric-fractional witness now replays the full rational
dual certificate in-crate: 182,304 atom columns, 39,072,252 sparse matrix
contributions, and a 16,855-coordinate rational witness. The replay derives a
compact pressure skeleton: only 168 atom columns are tight.

Tight atom size distribution:

| size | tight atoms |
| --- | ---: |
| 2 | 1 |
| 3 | 7 |
| 4 | 13 |
| 5 | 22 |
| 6 | 31 |
| 7 | 34 |
| 8 | 31 |
| 9 | 14 |
| 10 | 12 |
| 11 | 3 |

Top tight-pair co-participation:

| pair | tight co-participation |
| --- | ---: |
| 8-18 | 34 |
| 21-26 | 28 |
| 5-20 | 26 |
| 8-16 | 26 |
| 8-23 | 26 |
| 18-23 | 26 |
| 20-27 | 26 |
| 15-20 | 24 |

Five pressure-derived escape iterations were generated as retained Hadwiger
artifacts by `run_g27_pressure_escape_hypothesis_iterations_checked`:

| iteration | class | target | score | interpretation |
| --- | --- | --- | ---: | --- |
| 1 | outside-field clamp | pair 8-18 | 5,712 | Add an exact outside-Moser point that cuts the most reused tight pair. |
| 2 | tight-pair bridge | pair 21-26 | 4,704 | Check whether pressure is localized or distributed across a second pair. |
| 3 | isometry breaker | row 685 mapping 8 -> 13, 18 -> 6 | 4,692,800 | Break the highest tight-touch non-singleton congruence relation and measure slack response. |
| 4 | tight-atom transversal | vertices 23,5,12,14,18 | 1,240 | Anchor across the five highest tight-atom participation vertices. |
| 5 | non-Moser core graft | pair 8-18 plus vertex 21 | 230 | Graft a retained non-Moser high-fractional core onto the tight skeleton. |

Refinement: the raw highest-pressure isometry row was row 32, mapping
`12 -> 14`, but that is a singleton congruence. Singletons are universally
congruent in the plane, so the loop now suppresses singleton-only isometry
breakers instead of pretending they are a valid escape move.

Current best lead: **isometry breaker on row 685**. It is the highest-pressure
non-singleton row, touching 50 tight atoms and 46,928 sparse atom columns. The
retained lead report materializes row 685 as the exact mapping
`8 -> 13, 18 -> 6` and creates a mutation obligation requiring outside-Moser
exact algebraic geometry: preserve the G27 unit-distance replay while changing
that row's slack response. The next concrete work is to search for the
smallest outside-Moser anchor or graft that changes this two-point congruent
subset relation without destroying the G27 embedding.

The first bounded anchor scan checked the retained Moser-coordinate box with
per-coordinate expansion 1: 2,058 coefficient points, excluding the existing
27 seed points. It found 56 row-685 asymmetric anchor breakers and retained the
first 12 compactly. These are **not** escape evidence, because every one still
lives in the Moser-coordinate basis; the scan is retained as suppression
evidence. Its conclusion is operational: row-685 can be locally disturbed
inside the capped lattice, so the next funded search must add exact algebraic
coordinates outside the Moser lattice/ring rather than more integer
coefficient points.

Five real hypothesis rounds are now complete in `g27-next-five-rounds.json`.
They tested the exact dual-unit anchor motif directly instead of counting
infrastructure as research progress:

1. Pair `8-18`: the two exact unit-circle intersections are Moser-basis
   anchors, so the easiest row-685 outside-Moser dual anchor does not exist.
2. Pair `13-6`: the mapped comparison side is also Moser-capped, so row 685
   does not escape through the image pair.
3. Pair `21-26`: a second tight pair also has only Moser-basis dual anchors.
4. Pair `5-20`: another independent tight pair also caps out in the retained
   Moser-coordinate geometry.
5. Pair `8-16`: even a self-anchored variant has only Moser-basis dual anchors.

Decision: retire the exact dual-unit anchor motif for this G27 pressure pass.
The next funded route is **non-dual outside-Moser graft search**: single-anchor
or multi-constraint algebraic candidates must change geometric-fractional slack
without relying on the exhausted two-circle intersection motif.

Rounds 6-8 tested the next three motifs from that decision:

6. **Tight-atom hitting set.** Exact enumeration over the 168 tight atoms found
   zero transversals of size <= 4. The minimum size is 5, with retained
   witnesses `4-5-6-7-14`, `5-6-7-9-16`, and `5-7-9-13-16`.
7. **Parameterized one-anchor blocker.** The retained best size-5 transversal
   has zero Moser-basis common anchors, and the size-5 requirement blocks the
   cheap one-anchor route for this pressure pass.
8. **Pressure-skeleton spindle preflight.** The next funded route is now a
   manufactured-rotation spindle around hinge vertex `8`, using a retained
   pressure fragment containing `8`, `18`, and `23`.

Decision: one-anchor blockers are retired until new evidence changes the tight
face. The next real implementation target is an exact rotated-fragment
certificate language: represent a rotation parameter, replay fragment isometry
by exact pairwise distances, enforce one outside-Moser pin, and then replay the
geometric-fractional LP for surviving candidates.

The three proposed motif families have now all been searched at their current
certificate depth:

- **Pressure-skeleton spindle:** retained and then pre-screened two
  manufactured-rotation candidates (`pi/6` pinned at `21`, `pi/4` pinned at
  `23`). The exact basis audit shows both have zero nontrivial static G27
  unit-closure pins, so these two concrete rotations are suppression evidence.
  The spindle family remains funded only as a broader rotation/pin search.
- **Tight-atom blocker:** exact hitting-set enumeration falsified size `<= 4`;
  one-anchor blockers are retired for this pressure pass.
- **Cross-ring fusion:** retained three foreign-field fusion candidates
  (`76/21` family over `sqrt2`, Golomb/spindle composite over `sqrt5`, foreign
  spindle composite over `sqrt7`). These require the P2 column-generation
  replay before they can be evaluated as LP-improvement evidence.

Next move: broaden the exact rotated-fragment search over rotation and pin
choices, because the first `pi/6` and `pi/4` probes do not close nontrivially
against static G27 vertices.

Basis audit update: the public `G_27.txt` coordinate file pins the retained
Moser coefficient model to the exact symbolic basis
`(1,0)`, `(1/2,sqrt3/2)`, `(5/6,sqrt11/6)`,
`((5-sqrt33)/12,(5sqrt3+sqrt11)/12)`. The audit replays the G27 adjacency
exactly as `49` unit edges and `302` non-edges. This removes the previous
coordinate-basis ambiguity and makes exact rotated-fragment replay possible
without trusting floating-point coordinates.

Broadened rotation-pin search update: with the exact basis pinned, a bounded
float-screened search over the retained pressure fragment around hinge `8`
found `206` witness/pin closure derivations. The best unsuppressed closure
family rotates by about `103.221` degrees and creates `9` static G27 closure
pairs; canonical retained provenance starts from witness vertex `10` pinned to
static vertex `27`. This is not authority yet, but it is the first nontrivial
post-dual-anchor lead worth exact algebraic angle replay.

Exact equation update: the `10 -> 27` lead has now been retained as a checked
angle-equation artifact. Around hinge `8`, the moving radius is `3`, the pin
distance is `(9-sqrt33)/2`, the rotated point must satisfy dot product
`(13-sqrt33)/4`, and the circle-intersection height numerator is
`(7+sqrt33)/8`. Therefore exact closure-pair replay requires adjoining the
new radical `sqrt((7+sqrt33)/8)` to the retained Moser field. This is the
first concrete manufactured field-extension signal from the pressure-skeleton
spindle route.

Exact closure replay update: adjoining that manufactured radical is enough to
retire the specific `10 -> 27` broadened-spindle lead. Both algebraic rotation
branches replay exactly, but each branch makes only the intended pin
`10-27` a unit edge; the other eight float-screened static closure pairs are
not exact unit distances. The conclusion is useful suppression evidence:
future spindle searches must require exact closure replay before funding LP
replay, and this `103.221` degree branch should not be re-planned.

Batch exact replay update: the retained twelve float-screened rotation-pin
candidates have now been replayed with candidate-specific manufactured
radicals. No candidate has a broad exact closure of three or more static unit
pairs, so the broad closure version of the pressure-skeleton spindle route is
retired. Three candidates do retain exact two-pin closures. Those are not LP
evidence yet; they are smaller manufactured-closure leads that need pressure
scoring against the tight atom skeleton before the loop decides whether to
fund a mutated-graph LP replay or pivot to cross-ring fusion.

Pressure-score update: the three exact two-pin survivors do not touch the
retained top-pressure tight atom structure strongly enough to fund LP
preflight. The pressure-skeleton spindle route is therefore retired at the
current certificate depth. The next funded route is **cross-ring fusion**:
the retained foreign-field candidates must be evaluated with a
column-generation replay instead of more rotation-pin search.

Cross-ring fusion preflight update: retained fusion candidates are now scored
after spindle retirement. The selected candidate is the `76_21_fractional_core`
over `sqrt2`, sharing G27 vertex `8`. This does not prove improvement; it
funds the next exact task: build a master/pricing column-generation replay for
that foreign-field core and test whether the fused graph can produce a
geometric-fractional lower-bound certificate above the retained G27 value.

Cross-ring column-generation state update: the selected `76_21` lead has
standalone fractional lower bound `76/21`, while the retained G27 witness is
already at `4 = 84/21`. More importantly, the Cranston/Rabern source shows
`76/21` is an asymptotic discharging lower bound for a graph sequence, not a
retained finite 76-vertex column core. This retires the `76_21_fractional_core`
as a finite cross-ring fusion target. The next finite-core route must use a
machine-retainable graph/certificate.

Finite-core replacement update: the Bellitto/Pêcher/Sédillot `W_circles_607`
public data is now retained as a finite weighted fractional-core audit. The
replay verifies `607` vertices, `3390` edges, `607` integer weights summing to
`1999983`, and the retained weighted-independence denominator `512933`, giving
lower bound `1999983/512933`. This is still below G27, but the exact lift gap is
only `51749/512933`, far smaller than the retired `76/21` lead. The next
funded task is weighted-independent-set certificate replay plus fused
G27/`W_circles_607` pricing, not more spindle or asymptotic-core search.

Dual-slack inversion update: the pessimistic gate for the hidden-angle route
"attach to low-current-pressure but tight-adjacent G27 vertices" has been run
against retained G27 pressure/slack evidence. The nearest candidates are
vertices `9`, `25`, `7`, `13`, and `17`; the top score is only `148`, far below
the funding threshold `2000`, and all have zero retained Moser-basis
triple-unit attachments. This retires the broad dual-slack inversion route at
the current interface model. The failure is not "already capped by Moser
anchors"; it is weaker: the slack vertices simply do not couple strongly enough
to tight-neighborhood structure to justify algebraic attachment replay.

Algebraic-field friction update: the field-mismatch route has also been gated.
The current retained candidates name foreign field labels such as `sqrt2`,
`sqrt5`, and `sqrt7`, and some touch high-pressure G27 interfaces, but the
route does **not** yet have retained exact foreign coordinate geometry. Field
labels are not evidence. The broad route is therefore retired for now with a
specific reactivation condition: retain an exact foreign coordinate model and
replay real cross-field unit contacts against high-pressure G27 vertices. Until
then, no column-generation or algebraic friction work is funded from labels
alone.

Same-field finite pressure-donor update: inspecting the `W_circles_607`
source changed the route. Its vertices are not approximate floats; they are
exact coordinates in the same `Q(sqrt3, sqrt11, sqrt33)` basis as retained G27.
The exact geometry audit now parses all `607` symbolic vertices and independently
replays the `3390` unit-distance edges from the retained edge data. This means
`W_circles_607` is not a foreign-field friction object; it is a same-field
finite pressure donor. The next funded route is to search for a small
same-field interface between G27 tight/equality structure and high-weight
`W_circles_607` vertices, then test whether the fused weighted/geometric
fractional certificate can clear the exact `51749/512933` lift gap.

Same-field interface search update: after second-agent critique, the route was
tightened from "many contacts" to an exact capacity test against the lift
numerator `51749`. The top-decile search aligned the three highest-pressure
G27 anchors against the sixty-one highest-weight `W_circles_607` anchors and
replayed every translated cross-unit contact exactly. The best contact pocket
is real but not fundable: anchoring G27 vertex `23` to W vertex `254` creates
`358` exact cross-unit contacts with optimistic W-weight capacity `2289724`,
but every contact lands on a G27 vertex already priced by the retained tight
face (`358` priced, `0` unpriced, `0` unpriced capacity). Therefore the
top-pressure/top-weight same-field interface route is retired at this
certificate depth. The next novel theory should invert the pressure premise:
look for high-weight W donors that attach to low-pressure or zero-pressure G27
vertices without collapsing into the already-priced G27 tight face.

Slack-halo inversion update: the second-agent critique correctly warned that
raw low pressure is the wrong proxy and asked for an all-anchor slack-halo
scan ranked by G27-unpriced contact capacity. That scan has now run across all
`27` G27 anchors against the top `61` W anchors (`1647` exact alignments). It
does not produce a new lead: the best slack-halo candidate is again G27 vertex
`23` anchored to W vertex `254`, with `358` contacts, total W contact weight
`2289724`, and `0` G27-unpriced contact capacity. The conclusion is sharper
than simply "wrong anchors": under the retained tight-atom witness, the binary
priced/unpriced G27 filter is vacuous for this same-field donor search. The
next theory must use a marginal pressure model (for example pressure-normalized
or dual-slack-weighted contact value), not a zero-pressure/unpriced predicate.

Marginal pressure diagnostic update: after second-agent critique, the
pressure-normalized score was treated explicitly as a heuristic, not as a
quantity in the same units as the `51749` lift numerator. The fixed best
alignment `G27 23 -> W 254` was replayed as a per-contact channel report:
`358` exact contacts, total W contact weight `2289724`, and normalized score
`6955146274232/130008417`. That normalized score is numerically above `51749`,
but this is not lift evidence because the denominator is an invented
participation penalty, not certificate reduced cost. The top channel is
`G27 13 <- W 304`, W weight `36195`, G27 tight participation `36`, normalized
contribution `36195/37`; the top-five normalized share is only
`269928621527/3477573137116`. Conclusion: pressure normalization is a useful
diagnostic table, but it still does not name an exact lift witness. The next
funded theory must compute a real reduced-cost or sensitivity value from the
retained G27 certificate, then use the W donor contacts as candidate columns
only if that reduced-cost value survives exact replay.

Tight-atom contact triage update: the next second-agent critique rejected
"low tight fraction" as backwards and asked whether the same-field W donor
contacts actually hit retained tight atoms or near-zero slack atoms. The exact
triage now replays the fixed `G27 23 -> W 254` alignment against all `168`
retained tight atom masks. All `168` tight atoms are contacted. The top tight
atom has size `10`, all ten of its vertices are contacted
(`1,3,6,10,11,12,13,21,26,27`), and its W contact incidence weight is
`1044873`, far above the `51749` lift numerator. This does not mint theorem
authority because incidence weight can double-count donor pressure and is not
yet a fixed-dual reduced-cost proof. It does fund the next exact task:
compute fixed-dual pricing per retained G27 atom, compare donor-compatible
gain against that atom's slack in certificate units, and require a negative
reduced-cost witness before claiming any fused lift.

Fixed-dual pricing update: the second-agent critique sharpened the next test:
compatible W mass is not pricing evidence unless it is an independent W set.
The new fixed-dual diagnostic therefore excludes the glued W anchor, filters
W vertices by exact cross-unit compatibility with the retained G27 atom, and
prices the induced W subgraph by a certified bracket: a deterministic W
independent witness gives a lower bound, while a clique-cover bound in the
original W graph gives an upper bound. On the strongest tight atom channel,
the compatible W subgraph has `502` usable W vertices and excludes `104`;
the first retained independent witness had weight `487378`, while the current
upper bound was `758402`. This falsifies the raw contact-incidence optimism
(`1044873` was not usable W donor value), but it does not retire the route:
the upper bound still sits above the retained global W alpha weight `512933`.
The route now needs a stronger MWIS certificate for the compatible W subgraph,
not more pressure/contact proxies.

Threshold-MWIS update: the next second-agent critique approved a narrow
threshold kill-test but warned against building a general solver. The
implemented refinement decomposes the compatible W induced subgraph into
components, solves the eight small components exactly, and applies
deterministic one- and two-swap local improvement inside the one large
component. The component structure is lopsided: `9` components total, largest
component size `494`, exact small components `8`. The retained independent
witness improves to weight `498748` on `154` W vertices, leaving a target gap
of `14185` below the retained W alpha weight `512933`; the current clique-cover
upper bound remains `758402`. Therefore this is progress on the certificate
gap, not a decision. The same-field alignment remains unresolved until a real
threshold MWIS certificate either finds a compatible independent set of weight
`>=512933` or proves an upper bound below `512933`.

Fractional stable-set LP update: the next critique allowed a plain LP only as
a cheap surprise-kill diagnostic, and warned that no float primal objective can
retire the route without a replayable rational dual. The edge-relaxation LP
has now run on the same exact compatible W subgraph. It tightens the diagnostic
upper bound from the greedy clique-cover value `758402` to `722367`, but this
is still far above the target `512933`. Therefore the plain fractional
stable-set relaxation is too weak to decide the alignment. Do not re-fund this
edge-only LP path; the next useful certificate must be integer MWIS,
branch-and-bound/cutting-plane, or a stronger replayable rational dual with
clique/odd-cycle cuts.

Clique-cut LP update: the next critique allowed exact maximal-clique cuts as a
single bounded surprise test, but warned not to let clique enumeration become a
side quest. The compatible W subgraph enumerates cleanly: `1181` maximal
cliques, no cap hit, largest clique size `3`. Adding those clique constraints
to the stable-set LP drops the diagnostic ceiling from `722367` to `526858`.
That is still above the `512933` target by `13925`, so it cannot retire the
route, and because it is a floating LP primal ceiling it cannot carry authority
without a replayable rational dual. But unlike the edge LP, this is a real
near miss: the remaining certificate problem is now small enough to justify one
more certificate-friendly cut pass, most naturally odd-cycle/odd-hole cuts or
a threshold branch-and-bound on the `494`-vertex component.

Odd-cycle cut update: the next critique recommended separating genuinely
violated odd-cycle inequalities from the clique-LP primal solution rather than
enumerating short holes. A bounded parity-Dijkstra separator now adds at most
`64` cuts per round for `8` rounds. On the strongest channel it adds `228`
odd-cycle cuts over `6` rounds, with best violation `774445` ppm, and drops
the diagnostic ceiling from `526858` to `521799`. This is real movement but
still above the `512933` target by `8866`, so the bounded odd-cycle hypothesis
does not decide the alignment. Continue with threshold MWIS / branch-and-bound
or rational dual replay infrastructure rather than more unconstrained LP cut
experiments.

Threshold-MWIS B&B update: the next critique approved a narrow integer
branch-and-bound only if it stayed certificate-oriented and did not confuse a
threshold witness with exact MWIS. The fixed-dual posture now funds any
compatible independent witness with weight `>=512933`, while a threshold hit
no longer pretends to be exact MWIS. A native bounded B&B probe was added for
the retained `502`-vertex compatible channel. Exact small components contribute
`61894`, reducing the dominant `494`-vertex component target to `451039`.
The first integer-only probe is intentionally shallow (`20` nodes, ignored in
normal tests because it takes about two minutes): it keeps the incumbent at
`498748`, prunes only `2` nodes by threshold upper bound, and leaves best open
integer upper bound `743476`. This does not decide the alignment and shows the
naive native clique-cover B&B is too weak/slow as currently shaped. The next
funded integer step needs a stronger imported MWIS/CP-SAT certificate with
crate replay, or a materially better native bound/branching scheme; do not
keep increasing the shallow node cap as if that were new evidence.

LP-guided witness-repair update: the next critique approved one narrow
positive-witness attempt because a single compatible independent set of weight
`>=512933` would be exact replayable progress. A deterministic repair pass now
uses the odd-cycle LP primal values only as ranking guidance, then verifies
every candidate witness by exact graph independence and integer weight. The
predeclared menu runs `120` attempts across five destroy rankings, six destroy
sizes, and four refill scorers. It finds no improvement: the best witness
remains `498748` on `154` vertices. This retires the bounded LP-guided local
repair heuristic for this alignment. It does not count against existence of a
threshold witness. The next funded step should be imported exact MWIS/CP-SAT
certificate infrastructure or a genuinely stronger native integer bound, not
more local-repair tuning.

PB/SAT threshold preflight update: the next critique approved a bounded exact
SAT/PB preflight only as an encoding-size and quick-solve experiment, with SAT
model replay as the main prize and UNSAT trusted only with retained proof and
independent PB encoding validation. The dominant-component instance is frozen
at `494` vertices, `2225` edge clauses, exact small-component contribution
`61894`, and dominant target `451039`. A straightforward truncated weighted
totalizer is not viable: even with per-node sum caps at `50000`, the estimate
hits `37` capped merge nodes, `2102958` auxiliary variables, and
`40784945678` clauses. This retires the direct weighted-totalizer SAT/PB
encoding for this channel. It does not retire exact solver work generally; the
next exact route needs a materially different certificate strategy, such as an
external MWIS/CP-SAT solver that emits a compact witness or independently
checkable proof artifact.

Structural preflight update: the next critique approved one native-structure
kill-test before pivoting out of in-crate search. The dominant compatible-W
component is structurally hostile to separator DP: `494` vertices, `2225`
edges, no articulation points, a single `494`-vertex biconnected block, only
one simplicial vertex, no open-neighborhood twin classes, degeneracy `6`, and
heuristic elimination widths `161` (min-degree) and `131` (min-fill). This
meets the predeclared retirement criteria for the native structural route. It
does not prove the alignment is impossible; it says the next exact step should
be an external MWIS/CP-SAT solver/certificate artifact with in-crate replay,
not a hand-rolled separator/treewidth solver for this component.

External MWIS artifact update: the next critique approved an external integer
solver pass only as a positive-witness search. The retained dominant MWIS
instance now has a canonical line export plus an in-crate replay checker that
recomputes the same compatible channel, dominant membership, independence, and
integer weight from retained data. Export digest:
`15076449e9e0bd160545a4e5ed7812aaea1770d110918ff4466dd74fd33a1406`.
SciPy/HiGHS solved the `494` binary / `2225` edge-constraint MILP to reported
optimality in about `33` seconds and returned a best dominant independent set
of weight `440702`; crate replay verifies it as independent with total weight
`502596`. This is below the dominant target `451039`, so no positive witness
was found. The reported optimum is retained only as external diagnostic
evidence, not theorem authority or a negative certificate. The next exact
advance would need an independently checkable upper-bound certificate/proof, or
a different alignment/channel with a replayed positive witness.

Top-10 atom MWIS sweep update: the next critique approved one final exact
same-alignment sweep because contact-incidence rank and heuristic MWIS lower
bounds are not exact-MWIS orderings. The sweep freezes the retained top-10
tight atoms under `G27 23 -> W 254`, exports each compatible-W channel by
explicit atom mask and digest, solves each binary MWIS with the same
SciPy/HiGHS template, and batch-replays the selected W vertices inside the
crate. All ten HiGHS runs reported optimal within the `300s` per-channel
budget and all ten crate replays are below threshold. Best replayed totals are
`504396` for atom masks `101719589`, `34610725`, and `101719076`; the
threshold is `512933`. This retires the frozen top-10 tight-atom sweep for this
exact same-field alignment. It is still not a formal negative certificate for
all possible channels or alignments, because the external optimality claims do
not carry independently checkable proof authority.

Retained alternate-alignment MWIS screen update: the next critique allowed one
last bounded same-field alignment pass, using the retained candidate union from
the pressure-interface and slack-halo scans, excluding the retired
`G27 23 -> W 254` alignment. The artifact freezes `13` alternate alignments
and the top `5` tight atoms per alignment (`65` channels total), each replayed
by alignment key and atom mask. SciPy/HiGHS reported success on every channel
within the `300s` per-channel budget, and crate replay verifies every selected
set as independent and below threshold. The best replayed total is `511201`
for `G27 8 -> W 301` on atom masks `101719589` and `34610725`, leaving a
`1732` gap to `512933`. This retires the retained alternate same-field
alignment MWIS screen narrowly as a positive-witness route. It is not a formal
negative certificate for all alignments, and the solver optimality claims remain
external diagnostic evidence only.

MWIS certificate feasibility update: a proposed near-miss repair/attachment
follow-up around `G27 8 -> W 301` was rejected by the second agent as seductive
overfitting unless it introduced a new mechanism. The replacement hypothesis was
certificate authority: try to turn the external HiGHS optima for the frozen
near-miss channels into independently replayable upper-bound evidence. A checked
root-certificate screen now reconstructs the two best channels
(`101719589`, `34610725`) and evaluates exact clique-cover plus edge/clique/
odd-cycle LP ceilings. The root certificates fail decisively: for mask
`101719589`, clique LP total ceiling is `546227` and odd-cycle total ceiling is
`543428` after `158` cuts; for mask `34610725`, clique LP total ceiling is
`548970` and odd-cycle total ceiling is `546352` after `145` cuts. Both remain
well above the `512933` target. This retires the root-relaxation certificate
route for the near-miss channels. Any further certificate work must be a real
branch proof with replayable node bounds, not more root cuts or heuristic solver
trust.

Branch-certificate preflight update: the next critique allowed only a strict
bounded preflight before any full proof-engineering project, because a branch
certificate for one same-field channel is narrow suppression evidence unless
the proof format is reusable. The checked preflight now freezes
`G27 8 -> W 301`, atom mask `101719589`, and runs deterministic best-first
include/exclude branching with exact isolated-vertex inclusion and clique-cover
node bounds. It stops at the predeclared `10000` node cap. Result: no nodes
prune below the `451278` dominant threshold; best open total upper bound is
still `644951`, above the global `512933` target by `132018`. This retires the
clique-cover branch-certificate strategy for the near-miss channel. A full
branch proof would need materially stronger replayable node bounds, not just
more nodes.

Quadratic anchor attachment audit update: the next critique funded the pass
only as a cleanup/retirement audit, not as a likely discovery route. The audit
fixes the field premise first: `sqrt3` is already in the retained
`Q(sqrt3,sqrt11,sqrt33)` Moser basis, so radicand-`3` anchor extensions are
not outside-Moser evidence. The checked exact replay audits the `16` retained
quadratic survivors from the row-685 lane. It suppresses `8` radicand-`3`
candidates as inside the retained field. The remaining `8` genuine `sqrt2`
candidates have no exact unit attachments to any of the required row-685
vertices `{8,13,18,6}`. No candidate is mutation-eligible under the
predeclared all-four attachment criterion. This closes the bounded quadratic
anchor lane; future mutation search must generate candidates by solving the
required unit equations directly, not by arbitrary quadratic offsets.

Row-685 direct unit-equation follow-up: after checking the current Codex model
docs, the second-agent pass rejected a new all-four solve as redundant with the
existing dual-unit audit. The suppressing result is the `13,6` image-side
anchor test: any all-four point must be unit to both vertices `13` and `6`;
the audit found exactly the two Euclidean two-circle intersections and both
fail the comparison targets `8` and `18`. That closes the current row-685
all-four attachment mechanism without a new implementation pass. Reopen only if
the obligation changes to a different target set or a weaker predeclared
mutation operator is introduced.

W-circles weighted-certificate preflight update: the second-agent pass approved
one bounded infrastructure triage on the base `W_circles_607` weighted
independence certificate, with no novelty claim. The checked preflight replays
the retained `607` vertices, `3390` edges, `607` weights summing to `1999983`,
and target weighted alpha `512933`. Cheap root certificates fail by a wide
margin: greedy witness weight is `497885`, clique-cover upper bound is
`959119`, edge LP ceiling is `999992`, and maximal-clique LP ceiling is
`666662` with `1768` maximal cliques, no clique cap, and largest clique size
`3`. Bounded odd-cycle separation adds no cuts from that clique-LP solution and
also stops at `666662`, more than `1000` above the target. This retires cheap
root replay as a way to certify the published W bound in-crate. Future W donor
or fused G27/W progress needs an imported replayable proof artifact, a rational
dual from a stronger solver, or a branch certificate; not more root LP tuning.

W-circles symmetry preflight update: a second-agent critique allowed one exact
automorphism audit only as branch-certificate compression triage. The checked
preflight tests the identity, horizontal reflection, vertical reflection about
`x = 3`, and the half-turn about `(3,0)` using exact
`Q(sqrt3,sqrt11,sqrt33)` point lookup, integer weight preservation, and
retained edge preservation. Only the identity and half-turn are valid weighted
automorphisms; both individual reflections fail because transformed vertices
are missing from the retained vertex set. The resulting group has size `2`,
`304` vertex orbits, one fixed vertex, and `1695` edge orbits. This is weak
2x descriptive compression, not enough to fund an orbit-aware branch proof on
its own. Reopen only if a real branch-certificate format demonstrates concrete
proof shrinkage from the half-turn symmetry; otherwise keep moving toward
imported/replayable W607 proof artifacts or stronger external certificate
imports.

W-circles public-artifact inventory update: the next second-agent critique
approved only a bounded artifact triage, with a strict success criterion: a
hit must be an independently replayable upper-bound artifact for the exact
retained W607 digest, not a CPLEX script or floating solver transcript. The
LaBRI package named by the public page was downloaded as `avoidingDistance1b.zip`
and hash-inventoried. Its retained W607 `.dat` and vertices files match the
crate-pinned hashes exactly (`be181...b4dad` and `5ccc...69e95`), so this is
the right public instance. The inventory contains `15` files: `7` data files,
`1` OPL model, `4` scripts, and `3` generated-code helpers. It contains `0`
proof-like files. Therefore public-package import is retired as a certificate
route: it provides reproducibility data/model/scripts only, not a replayable
proof of `alpha_w(W607) <= 512933`. Future W607 certificate work must bring a
new artifact with schema `instance_digest + objective_upper_bound +
branch_tree_or_rational_dual_or_weighted_cover + exact_replay_checker`.

Public donor certificate triage update: the next proposed "find another
finite donor" pass was narrowed by second-agent critique to certificate-bearing
triage only. The bounded web pass checked the current G27 paper, the W607
LaBRI package, the dense-UDG beam-search line, and 2026 unit-distance exponent
certificate packages. No new Hadwiger-Nelson finite donor was found with both
machine-readable exact data and a replayable weighted-independence or
fractional-coloring certificate. The one genuinely certificate-bearing new
package found is Tseng's Sawin-style Erdos unit-distance pointwise proof
package, but that concerns the number of unit distances among `n` points, not
coloring, weighted independence, or G27 pressure interfaces. Public donor
hunting is therefore retired until a source provides an exact finite graph plus
branch tree, rational dual, weighted cover, SAT/PB proof, or equivalent
replayable certificate. Next funded direction: design a minimal reusable MWIS
upper-bound certificate format and test whether an external solver can emit
enough branch/dual/cover information for base W607 or one frozen near-miss
channel.

MWIS upper-bound certificate replay update: the second-agent critique rejected
a new measurement preflight because it would only rerun known clique-cover and
branch failures. The approved smaller step was an importer-facing replay
contract. A checked `WeightedCliqueCoverLeafV1` verifier now binds a certificate
to a graph digest, candidate set, clique rows, integer capacities, per-vertex
weight coverage, and objective equality. A toy path fixture replays at its
target (`objective = 4`). The W607 greedy root-cover fixture also replays,
bound to the retained W607 digest, but it is deliberately weak: `270` cliques,
objective `959119`, target `512933`, excess `446186`.

External weighted-cover producer update: a second-agent critique approved one
bounded integer clique-cover ILP over the `1768` retained maximal cliques of
W607. This certificate class cannot prove the target because the retained
clique LP is already `666662`, but it can test whether external cover artifacts
are worth replaying. SciPy/HiGHS produced an integer cover with `1192` positive
clique rows and objective `666661`; the crate replay verifies every row as a
clique, every capacity as integer, the W607 digest binding, and all vertex
weight coverage. This improves the greedy cover by `292458` and clears the
predeclared `<= 700000` funding bar, but remains `153728` above `512933`. The
root weighted-cover artifact route is therefore useful as certificate plumbing
but not sufficient mathematically. Next funded step needs branch composition or
rational dual/cut certificates, not more root clique-cover optimization.

Branch plus ILP-cover preflight update: the next second-agent critique allowed
only a strict falsification pass, with a useful depth-1 bar of max child
`<= 625000` and retirement if residual cover solves were already too slow. A
first broad branch script timed out at the 20-minute cap before producing a
diagnostic file, so the pass was reduced to the single most favorable obvious
split: include/exclude W vertex `304`, the highest-weight vertex (`36195`).
Even that one split is not promising. The include residual has `582` vertices,
`3036` edges, and `1546` cliques; HiGHS hit the `60s` limit with incumbent
residual cover `567933`, total `604128`. The exclude residual has `606`
vertices, `3366` edges, and `1756` cliques; it hit the time limit with incumbent
`654599`. The max child diagnostic is therefore `654599`, well above the
`625000` useful threshold, and neither child solved cheaply. Retire
base-W607 branch certificates whose leaves are only integer clique-cover ILPs.
Future branch work needs stronger leaf bounds, most likely rational dual/cut
certificates or solver-native proof artifacts, before proof-size engineering is
funded.

Rational root clique-cover diagnostic update: the next second-agent critique
approved rational dual replay only as leaf-certificate infrastructure, with a
strict requirement that any root rational certificate be no worse than the
existing replayed integer cover `666661`. The external fractional clique-cover
LP over the same `1768` maximal cliques solved immediately with objective
`666661.0000000003`, i.e. no meaningful improvement over the denominator-1
integer cover already retained. Direct rationalization by individual bounded
denominators explodes common denominators, while simple scaled-ceiling repairs
give objective ceiling `666662` even at denominator `1_000_000`. Therefore a
root-only rational clique-cover checker would duplicate existing authority
rather than improve it. Retire root rational clique-cover replay for W607; any
future rational certificate must include non-clique cuts, branch-local duals
that beat integer covers, or solver-native proof artifacts.

Dense rank-cut preflight update: the next second-agent critique rejected broad
non-clique cut hunting, but allowed one bounded kill-test over dense induced
subgraphs with exact alpha. The external diagnostic solved the W607 clique LP
at objective `666661`, then checked `48` pockets up to `80` vertices: top LP
mass, top weighted LP contribution, closed neighborhoods, and clipped two-hop
neighborhoods around high-contribution vertices. Every pocket had exact
unweighted alpha far above the LP mass; no violated rank cut was found. Typical
two-hop clipped pockets have LP mass `26.6667` and alpha between `36` and `48`.
This retires bounded root dense-rank cuts as a way to lower the W607 root bound.
Future cut work needs a named structure with demonstrable violated mass, not
generic local pocket search.

Weighted local-rank cut update: the failed unweighted rank test had missed the
actual weighted structure. A second-agent critique approved one bounded
weighted test: for a pocket `S`, the valid inequality is
`sum_{v in S} w_v x_v <= alpha_w(G[S])`, so the root one-third point violates
exactly when `weight(S) - 3*alpha_w(G[S]) > 0`. The bounded pass checked `83`
deterministic pockets up to `120` vertices (top weights, closed neighborhoods,
two-hop clips, and dense weighted expansions). It found `16` pockets with
violation numerator at least `3000`. Adding those diagnostic cuts to the root
LP dropped the W607 bound from `666661` to `641090.9615`. The best single
violation is the top-120-weight pocket: weight `1047755`, exact-MILP diagnostic
`alpha_w = 316539`, violation numerator `98138`.

This is the first genuinely different W607 root mechanism after clique,
odd-cycle, unweighted-rank, and branch-cover failures. It is not theorem
authority yet: the pocket `alpha_w` values came from external MILP diagnostics,
not retained in-crate proof artifacts. The next funded step is to convert the
violated weighted pockets into replayable crate evidence: recompute or certify
their weighted MWIS upper bounds, replay the weighted-rank rows, and test
whether the cut family has a geometric/orbit pattern worth expanding.

Weighted rank-cut replay update: the accepted `16` weighted local-rank cuts now
replay inside the crate. The replay reconstructs W607 from retained data,
deterministically regenerates each accepted pocket, recomputes exact weighted
MWIS with the crate solver, verifies the witness weight and independence, and
checks the exact violation numerator against the retained values. The release
example verifies all `16` rows: the top-weight-120 pocket has weight `1047755`,
`alpha_w = 316539`, and violation numerator `98138`; the dense `223/224`
size-120 pockets each have violation numerator `48638`. Runtime is substantial
for this first exact replay pass (about eight minutes in release), so the next
step should improve proof ergonomics or identify the geometric pattern before
expanding the cut family. The legitimate retained claim is now narrow but real:
W607 has replayed weighted local-rank cuts that lower the diagnostic root LP
from `666661` to about `641091`; this still does not prove
`alpha_w(W607) <= 512933`.

Weighted rank pattern/orbit preflight update: the half-turn completion
hypothesis is now retired. The bounded diagnostic reconstructs the retained
`16` cuts, computes support hashes and half-turn canonical support hashes, then
generates only missing half-turn completions for the accepted centered generator
types (`twohop80`, `twohop120`, `dense80`, `dense120`). It finds `6` candidates.
The dense completions are exact duplicate supports already represented by the
retained `223/224` dense cuts. The only non-duplicate completions,
`twohop80_384` and `twohop80_385`, have exact diagnostic
`alpha_w = 220860`, giving violation numerator `-7787`; they are not cuts.
Re-solving the root LP with the existing `16` cuts plus all accepted new cuts
therefore gives identical objective `641090.9615275878`, with marginal drop
`0`. Static half-turn/orbit expansion is not a research mechanism here. The
next funded cut route must be adaptive separation from the post-16-cut LP
solution, generating weighted-rank pockets from high remaining fractional mass
instead of mirroring root-discovered pockets.

Adaptive weighted-rank separation update: the post-16 LP separation heuristic
has also been retired at the bounded diagnostic depth. The diagnostic first
reproduces the retained post-16 objective `641090.9615275878`, then ranks W607
vertices by remaining weighted fractional contribution `w_v x_v` and generates
`92` unique supports of size at most `120` from top-score, one-hop, two-hop, and
dense expansions around the top `20` centers. Only one generated support is
actually violated at the post-16 solution with current violation at least
`3000`: `twohop100_304`. It is a high-overlap variant of retained structure
(`0.833` Jaccard against retained supports), and even adding all currently
violated generated rows drops the LP only from `641090.9615275878` to
`640825.3919449048`, a marginal improvement of about `265.57`. This misses the
predeclared `500` total-drop threshold and leaves no accepted low-overlap new
cuts. Local weighted-rank enumeration around the current LP has therefore
exhausted its cheap value. The next funded route should turn the retained
16-cut phenomenon into stronger certificate/proof structure, or test a more
principled lifted/disjunctive cut mechanism, not more neighborhood pocket
heuristics.

Branch-local weighted-rank diagnostic update: a depth-1 vertex-split version of
the lifted/disjunctive idea was also tested under strict second-agent gates.
The diagnostic reproduces the post-16 root objective, selects `8` branch
vertices from `w_v x_v`, fractional-weight, and retained-cut pressure ranks,
solves both children for each, then runs child-local weighted-rank separation
only on the best `3` raw splits. The best raw split is vertex `304`, with
include child `604127.0` and exclude child `638603.488`. Child-local separation
adds two high-overlap cuts in the exclude child and lowers that child to
`632232.3997`; the include child does not move. This is a real depth-1
improvement but misses the predeclared funding bar: it is above both the
`625000` proof-engineering threshold and the `631000` "interesting follow-up"
threshold. Splits on `456` and `225` improve only their include children while
their exclude children remain around `639467.5`. Retire branch-local
neighborhood rank cuts at this depth. The next useful theory should attack
proof authority and stronger leaf bounds directly, for example by trying to
rationalize compact child LP duals over edge, triangle, and retained rank rows,
or by designing a principled lifted/disjunctive certificate rather than
enumerating more local supports.

Root dual certificate update: the compact LP-dual proof-authority path is now
retained, with a narrow but real certificate. The post-16 W607 root relaxation
over edge, triangle, and the `16` replayed weighted-rank rows has a compact
positive dual support: `595` triangle rows, `2` weighted-rank rows
(`top_weight_120` and `dense80_304`), no edge rows, and no singleton upper-bound
repairs. The floating dual objective is `641090.9615275887`. Rounding all
positive dual multipliers upward to denominator `1024` gives an exact
triangle/rank weighted-cover certificate with `597` rows and objective
`656787579/1024 = 641394.1201171875`, with zero repair rows and exact minimum
coverage slack `0`. The new in-crate replay checker verifies the retained W607
graph/weight digest, every triangle row as pairwise adjacent, both rank supports
as deterministic retained pockets, both rank `alpha_w` values by recomputing
exact MWIS, exact integer coverage against `weight[v] * 1024`, and exact
objective equality. This certifies the first useful W607 root mechanism with
proof authority, but it is still far above the published target `512933`.
Future use: this row language is now a substrate for branch-local dual
certificates or lifted/disjunctive certificates; it is not itself a target
proof.

V304 branch-local dual certificate update: the first branch-local dual cover
now also replays exactly. After second-agent critique, the bounded diagnostic
tested only the meaningful child from the branch-rank pass: exclude W vertex
`304`. The deleted-vertex model and explicit `x_304 = 0` model agree exactly at
`632232.3996589432` after adding the two child-local rows from the diagnostic
(`top_wx_120` and `dense120_303`). The positive child dual support is compact:
`599` parent-triangle rows and one child weighted-rank row, `dense120_303`; no
singleton/fixed-bound rows and no repairs. The child-rank row carries about
`38.85%` of the dual objective. Rounding upward to denominator `1024` gives an
exact branch certificate with objective
`647496725/1024 = 632321.0205078125`, row count `600`, and min active coverage
slack `0`. The in-crate replay checker verifies parent-triangle validity,
branch-active coverage for every vertex except `304`, sorted listed child-rank
support, exact MWIS replay for `alpha_w(dense120_303) = 287232`, and exact
objective equality. This proves branch-local dual certificates are viable
proof-plumbing artifacts, but the bound is still far above the W607 target and
the paired include child remains `604127.0`; the next mathematical question is
whether the include/exclude logic around `304` can be lifted into a stronger
root-level disjunction or whether deeper branch duals compound enough to matter.

V304 one-node split update: the paired include/exclude certificates now make a
compact one-node branch proof replayable. The include side fixes W vertex `304`,
deletes its closed neighborhood, and replays a residual triangle/edge dual cover
with denominator `1024`, objective
`618626223/1024 = 604127.1708984375`, `573` residual clique rows, and min active
coverage slack `0`. The residual model agrees with the explicit `x_304 = 1`
LP to numerical tolerance. Combining this include bound with the replayed
exclude certificate gives the valid objective split cut
`w*x <= U_exclude - (U_exclude - U_include) x_304`. Added to the post-16 root LP,
that single cut is violated by about `18746.13` at the old root solution and
lowers the LP from `641090.9615275878` to `632321.0205078127`, driving
`x_304` to `0`. This is a useful compressed branch-bound certificate, not a
new native root-cut theory: it restates the one-node branch proof and lands on
the exclude certificate. Future progress must either find a non-objective
disjunctive cut from the child row systems or show that deeper compact branch
duals compound materially toward the `512933` target.

V304 exclude depth-2 update: the bounded compounding test is retired at the
second agent's raw gate. Inside the `x_304 = 0` child, with the retained root
rows plus the two v304-exclude child rows, the best second split among eight
fractional high-leverage vertices is vertex `386` (tied with its half-turn
mates). Its raw grandchildren are `613194.8024` and `631039.9884`, so the max
child misses the strict `631000` pre-separation gate by about `39.99`. Per the
predeclared rule, no bespoke grandchild weighted-rank separation was run. This
is useful negative evidence: depth-2 branch-dual compounding is not obviously
strong at the next node, and continuing to polish branch numbers would be
mispriced.

V304 lifted child-rank update: the non-objective single-row lift was also
tested and retired. The positive exclude-child row `dense120_303` has a
120-vertex full support but only `119` active vertices when `x_304 = 0`.
Recomputing exact MWIS on that active support gives `alpha0 = 287232`, and the
include-side residual support has `beta = 240974`, so the minimum stable-set
lift coefficient is `M = 0`. Thus the row is actually an ordinary parent-valid
weighted-rank cut on a child-discovered support. It violates the post-16 root
solution by `2554.4249`, but lowers the LP only from `641090.9615` to
`640492.3210`, below the `1000`-drop funding bar. Combined with the v304
objective split, it improves the split bound only from `632321.0205` to
`632282.2162`, below the `100` improvement bar. Retire isolated child-rank
lifting for v304. The stronger remaining theory is aggregate child-dual
lifting: lift the whole positive exclude certificate as a conditional cover,
not just one rank pocket.

V304 aggregate child-dual lift update: aggregate lifting is now the first
non-objective branch-certificate lift with real root movement. The full
v304-exclude certificate was aggregated into its integer coverage vector
`c_v` over active vertices, with objective numerator `647496725` and minimum
active coverage slack `0`. The exact include-branch lift coefficient would be
`L = U0 - gamma`, where `gamma` is a coverage-weight MWIS on the
`x_304 = 1` residual. The capped exact MILP did not close `gamma` within
`180s`, but its dual upper bound certifies `gamma <= 546085806`, hence the
weaker valid lift `L >= 101410919`. This conservative lift is already stronger
than the residual-objective split coefficient `65934182`. Adding the resulting
parent-valid aggregate row to the post-16 root LP lowers the objective from
`641090.9615` to `632232.3997`, forcing `x_304 = 0`. Combined with the v304
objective split, it improves the split bound by `88.6208`, narrowly missing
the predeclared `100` beyond-split funding bar. This is not yet publishable
target progress, but it is a genuine new certificate mechanism: nonuniform
child-dual coverage can be lifted into a valid parent root cut using only a
certified residual MWIS upper bound. Next tests should either close the
coverage-weight `gamma` exactly, try the symmetric include-aggregate lift, or
search for a child certificate whose aggregate lift beats the split by a
larger margin.

V304 include-aggregate lift update: the symmetric include-side aggregate is
retired by the second agent's cheap pre-MWIS gate. The include certificate
covers vertex `304` by `37063680 = 1024 * 36195`, covers all `582` residual
active vertices with min slack `0`, and covers no neighbors of `304`, as
expected from the `x_304 = 1` branch. At the post-16 root solution, however,
the aggregate left side is only `570375351`, and the root violation gate would
require a lift coefficient at most `-76258321.9`. Since the coefficient cannot
be useful when the zero-lift row already misses the gate, no coverage-weight
MWIS was run. Standalone include-aggregate lifting is not a mechanism here.
The sharper remaining version is not another one-sided lift, but an optimized
nonnegative combination of include and exclude aggregate coverage vectors
before lifting.

V304 projected aggregate-mix update: the combined-aggregate hypothesis was
tightened by the second agent into a projected `x_304 = 0` face test, because
the current best split plus exclude aggregate LP already has `x_304 = 0` and
branch lift coefficients are irrelevant there. The grid screen found that the
best row is actually the pure exclude aggregate coverage vector (`lambda=1`),
but with a newly computed projected exclude-face RHS. A `180s` capped
coverage-weight MWIS gives an incumbent `52393800800` and certified upper bound
`61337239200` for the scaled projected RHS; the current face solution has LHS
`64749672500`, so the certified projected violation is
`3412433300 / (100*1024) = 33324.5439`. Adding this projected exclude-face row
to the current `x_304 = 0` LP lowers that face from `632232.3997` to
`598919.8441`. Since the include child remains certified at
`604127.1709`, the diagnostic one-node branch max drops to `604127.1709`.
This is the largest W607 movement in the current certificate line, but it is
not yet a root-valid theorem artifact: the projected row is face-valid only
for `x_304 = 0` and must be turned into a parent-valid lifted certificate or a
checked branch proof before claiming authority.

V304 projected parent-lift update: the face cut now lifts back to a valid
parent row. Using the projected exclude-face upper bound `gamma0 <= 613372392`
and the include-branch coverage upper bound `gamma1 <= 546085806`, the
conservative lift is `L = 67286586`, giving
`c0*x + 67286586*x_304 <= 613372392`. The second-agent critique confirmed that
upper bounds are safe here as long as both are for the same coverage vector
`c0` on the correct branch domains. This row strictly dominates the older
aggregate lift for every `x_304 < 1`, and the LP confirms it: adding the single
parent-valid lifted aggregate row to the post-16 root relaxation lowers the
objective from `641090.9615` to `598919.8441`, with `x_304 = 0`. It improves
the previous best split/aggregate bound by `33312.5556`. This is a real
certificate-mechanism advance, but still not W607 target authority: the bound
is above `512933`, and the row currently relies on diagnostic MILP upper
bounds that need exact replay/certification before publication-grade claims.

Gamma hardening compatibility update: flat edge/triangle cover replay is not
the right proof language for the two coverage-weight gamma bounds. A short
compatibility probe tried to cover the exact exclude aggregate weights `c0` on
both branch domains using edges and triangles. On the `x_304 = 0` domain the
best flat LP cover is `670405293.0`, which is `9.30%` above the needed
`gamma0 <= 613372392`. On the `x_304 = 1` residual the best flat LP cover is
`581636664.67`, `6.51%` above the needed `gamma1 <= 546085806`. Both miss the
`0.5%` compatibility gate. Therefore the parent-lift row is real diagnostic
evidence, but exact hardening probably needs a richer certificate language:
branch/tree proof, solver-native dual-bound export, or additional non-clique
rows, not a compact flat triangle cover.

Post-parent-lift separation update: local rank/c0 pocket separation does not
compound the new parent lift. A bounded separator at the `598919.8441` LP
solution generated `78` supports and tested `234` rows across original
W-weights, `c0`-weights, and a small hybrid. Two low-overlap `c0` rows were
accepted by the formal gates, but the reoptimized objective moved only
`0.7876`, from `598919.8441` to `598919.0564`. The largest raw `c0` violations
are therefore almost entirely redundant with the parent-lift row. Retire local
post-lift rank separation as a compounding mechanism; future progress should
come from another lifted branch/aggregate certificate or a replayable
branch/tree certificate for the gamma bounds.

Post-parent-lift branch pre-screen update: a second lifted aggregate around
another high-fractional vertex is also retired at the cheap raw gate. Under the
`16` root rank rows plus the v304 projected parent-lift row, the top `8`
fractional vertices all remain essentially at `x = 1/3`, but fixing them one
at a time barely changes the bound. The best split is vertex `152`: exclude
child `598919.4160`, include child `598914.2639`, raw max `598919.4160`, and
the induced objective split moves the parent LP by only `0.4281`. This misses
the strict `590000` max-child funding gate and the `5000` split-movement gate.
Do not fund a second aggregate/gamma calculation from high-weight fractional
single-vertex splits; the remaining plateau is not exposing another v304-style
branch mechanism at this depth.

Gamma0 branch-tree preflight update: the existing projected parent-lift row has
a surprisingly compact replayable-shape proof path for its harder gamma bound.
For the `x_304 = 0` coverage-weight MWIS problem on `c0`, adding the inherited
root weighted-rank rows to the edge/triangle relaxation gives root LP upper
`654024375.5893` (still `6.63%` above target), but a best-bound branch tree
closes the target `gamma0 <= 613372392` in only `17` expanded nodes and `18`
closed leaves. The worst closed leaf is already `608954093.6667`, about `4.4M`
below target. This does not yet mint theorem authority because the leaf bounds
are floating LP solves, but it strongly funds exact denominator-rounded leaf
dual export/replay for the gamma0 certificate language.

Gamma0 leaf-dual export update: exact rounding succeeded immediately at
denominator `1024` for all `18` branch leaves. The candidate artifact binds the
W607 graph digest, the `c0` vector digest from the v304-exclude certificate,
the `x_304 = 0` branch domain, every leaf include/exclude state, and exact
integer coverage/objective checks for the rounded dual rows. Total positive
leaf rows: `10135`. Worst exact rounded leaf objective:
`608954093.8311`, still below the required `613372392`. This funds a Rust
replay checker for gamma0; until that checker independently recomputes active
sets, row validity, coverage slacks, and leaf partition coverage, the artifact
is a proof candidate rather than final authority.

Gamma0 replay update: the crate now independently replays that gamma0 branch
certificate. The checker reconstructs W607, recomputes `c0` from the exact
v304-exclude certificate, regenerates the deterministic root rank supports,
verifies the leaf family semantically partitions the `x_304 = 0` branch domain,
and checks all `10135` edge/triangle/rank rows with integer coverage arithmetic.
Replay result: `18` leaves, denominator `1024`, worst leaf objective numerator
`623568992083`, i.e. `608954093.8311 <= 613372392`, with min coverage slack
`0`. This gives proof authority for the projected gamma0 face bound. The full
parent-lift row is still not fully replayed until the include-side `gamma1`
bound is similarly hardened or otherwise certified.

Gamma1 and parent-lift replay update: the include-side gamma bound now also has
the same certificate authority. A bounded probe first confirmed that flat
edge/triangle/rank cover still misses gamma1 by `6.51%`, so branch proof was
needed. The `x_304 = 1` residual branch tree then closed
`gamma1 <= 546085806` in `15` expanded nodes and `16` leaves, and every leaf
rounded at denominator `1024`. The Rust replay verifies `8555` exact rows with
worst objective numerator `559085319025`, i.e. `545981756.8604 <= 546085806`,
and min slack `0`. A parent-lift replay now composes the gamma0 and gamma1
certificates and checks `613372392 - 546085806 = 67286586`, proving the
parent-valid lifted row
`c0*x + 67286586*x_304 <= 613372392`. This is still not the W607 target
`512933`, but it upgrades the strongest W607 diagnostic mechanism into a
proof-carrying non-objective lifted aggregate certificate.

Branch-slack lift update: the replayed gamma0 proof tree has yielded a second
proof-carrying lifted row. A conservative slack LP over the gamma0 branch
variables `{152,222,225,305,383,384,386,456}` found nonnegative coefficients
on six vertices (`152,222,225,383,386,456`) and an RHS reduction
`4100471.0879`. The `x_304 = 0` face row cuts the current parent-lift solution
by `4626.68` objective units and lowers the face LP from `598919.8441` to
`594914.3515`. Reusing the existing `16` gamma1 leaves for the modified vector
now replays exactly at denominator `1024`: the checker verifies gamma0 slack
charging, all `8555` modified gamma1 rows, semantic branch partitioning, and
the composed canonical parent row
`1024*(c0+p)*x + 64809127989*x_304 <= 623894447014`, with gamma1 side
`559085319025/1024` and parent drop `4005.4925`. This is proof-carrying
mechanism progress, but the resulting diagnostic bound `594914.3515` is still
well above the W607 target `512933`.

Residual branch-slack iteration update: same-substrate p/q iteration is now
retired by the second agent's cheap falsification gate. The only sound next
version was to freeze the replayed `p` coefficients and first RHS reduction
`R1`, compute exact residual gamma0 leaf slack, and look for new nonnegative
`q` coefficients outside the old eight branch variables. That probe finds no
usable residual direction: with the replayed `p` row the root sits at
`594914.3515`, but the residual LP has `q_support_size = 0`, current violation
`0`, face drop `0`, and the top non-old singleton candidates already have
zero residual capacity. The first branch-slack row has spent the cheap slack
on this leaf substrate; future progress needs a different branch/tree proof
object, a new lifted aggregate source, or a stronger row language, not another
iteration over the same gamma0/gamma1 leaves.

Post-branch-slack global rank update: nonlocal W-rank separation on the broad
`1/3` plateau is retired too. The second agent approved exactly one bounded
falsification pass because the branch-slack row changed the LP geometry, but
required original W-weight rows and real LP movement. The diagnostic generated
`127` deduped low-overlap candidates, ran the capped `80` exact MWIS calls on
support sizes `120..240`, and accepted `0` rows. Total LP drop is `0`, and the
heavy `1/3` plateau count remains `15`. The largest raw W violations were
redundant with the current row system: the best single-row drop was only
`147.3249`, with the next at `69.7629`, both below the `500` single-row gate
and far below the `1000` kill threshold for all accepted rows. Do not re-fund
bigger W-rank separation on this same two-parent-row solution without a new
support-generation principle; the remaining plateau is not yielding to rank
rows of this kind.

Plateau branch-tree update: compact disjunctive branching on the current
heavy `1/3` plateau is also retired. The second agent approved one bounded
preflight over the current row system only: the `16` root rank rows plus the
two replayed parent rows, no new row language. Branching on Tier A
`{305,224,385,303,223,384}` fully closes in `16` leaves, so this is not a
node-cap failure, but the worst leaf is still `594597.0511`, only `317.3004`
below the `594914.3515` root. The all-excluded Tier A leaf is the worst. This
misses the `590000` continuation gate, the `2500` Tier-A movement kill gate,
and all export/replay gates. Do not expand to Tier B or fund plateau leaf-dual
export from this evidence; the plateau is not a compact disjunction under the
current proof language.

Mod-3 triangle CG update: sparse rank-1 Chvatal-Gomory cuts from tight
triangle rows are retired at the bounded diagnostic depth. The second agent
confirmed the cut form is sound: triangle multipliers `y_t/3` with
`y_t in {1,2}` give a valid CG cut when every vertex incidence count is
divisible by `3`, and a violation can occur only when `sum_t y_t` is nonzero
modulo `3`. The diagnostic found `1261` strict-tight triangles at tolerance
`1e-8` and searched `26` local pools of `120` triangles around the plateau and
top weighted-fractional vertices. Each pool had a small GF(3) kernel, but
every sampled kernel vector had `sum_t y_t == 0 mod 3`, so the rounded RHS was
integral and no violated sparse mod-3 triangle cut existed. Generated,
tested, and accepted cuts are all `0`; objective remains `594914.3515`, and
the heavy `1/3` plateau count remains `15`. Do not re-fund sparse local mod-3
triangle CG on this same solution without a new pool principle or a reason to
believe larger nonlocal kernels evade the observed integral-RHS obstruction.

Weighted theta preflight update: native W607 weighted Lovasz-theta is retired
as a target-closing lane under the current tooling. The strict conic SDP plan
was mathematically the right object, but Clarabel `0.11.1` attempts a
`272 GB` allocation for the `607x607` PSD cone in both primal and dual forms,
so it cannot provide stable primal/dual residual authority here. As a
non-authoritative fallback, a sparse spectral dual probe optimized edge
multipliers directly against the largest eigenvalue and landed at
`614434.7845`, above even the `590000` hard-kill gate. Sanity checks for the
dual form match the analytic empty-graph value `1999983` and complete-graph
value `36195`. Do not re-fund full-matrix theta unless a genuinely scalable
SDP implementation with certificate-grade residuals is available.

Branch-leaf dual-ray perturbation update: recombining the existing replayed
gamma0/gamma1 leaf row dictionaries is retired after the bounded sound
diagnostic. The second agent allowed exactly one LP over a common
parent-valid vector `d >= 0`, with per-leaf coverability constraints using
only already-exported positive edge/triangle/rank rows and no replay/export
unless it cleared movement gates. The LP found a non-duplicate vector
(`cosine(c0)=0.7855`, `cosine(c0+p)=0.7829`) with large raw current
violation `22466674.8668`, but adding the resulting parent-valid row drops
the current root LP only from `594914.3515` to `594894.6597`, a movement of
`19.6918` versus the `2000` diagnostic gate. The lift is zero
(`gamma0 = gamma1`), `x304` remains `0`, and the row is therefore redundant
with the current two-parent-row relaxation despite raw violation. Do not
re-fund same gamma0/gamma1 leaf-dictionary recombination without a new branch
tree or proof language.

All-excluded residual-face rank update: the specific worst leaf from the
retired plateau branch tree has now been tested directly. Laplace approved
one tiny falsification pass because the all-excluded Tier-A face
`{305,224,385,303,223,384}=0` might expose conditional W-rank pockets hidden
at the root. The fixed-leaf LP reproduced at `594597.0511`. A bounded
leaf-specific support generator produced `64` candidates and ran the capped
`40` exact MWIS calls. It found one genuine conditional row
(`wx_dense220_152`, alpha `362026`) with a `2194.8934` LP drop, moving the
leaf to `592402.1577`. This is real movement but it misses the `3000` funding
gate and remains above the `592000` kill line, so leaf-rank export/follow-up
is retired. The obstruction is not merely a missing rank row on the worst
visible residual face.

Conditional rank-row propagation update: the one real residual-face row was
also replayed globally as bookkeeping closure, not as a reopened rank-search
lane. Heisenberg required deterministic regeneration of `wx_dense220_152`,
support/digest recording, old root and Tier-A baseline reproduction, and no
new search. The row regenerates exactly and reproduces the all-excluded leaf
drop, but it gives numerical-zero root movement and improves the Tier-A max
by only `237.5947`, shifting the worst leaf to a neighboring high leaf
(`223` included; `224,303,385` excluded) at `594359.4564`. This is a hard
retirement: the conditional row is locally real but globally redundant for the
current branch proof object.

Odd-cycle parity update: the current two-parent W607 LP has no useful mod-2
stable-set parity structure. Faraday approved one bounded separator for simple
odd cycles of length `5..101`, warning that the heavy `1/3` plateau should be
invisible to these cuts. The diagnostic reproduced the root at
`594914.3515`, found zero high-weight near-`1/2` vertices, and generated zero
violated odd-cycle cuts in the first round. Objective and the heavy `1/3`
plateau are unchanged. Retire odd-cycle rows on this substrate unless a new
near-half parity structure appears.

Two-variable aggregate-lift update: bounded non-objective affine lifting over
Tier-A pairs is retired. Arendt approved one diagnostic only if all four face
gammas were computed for the same vector `d`, not by reusing objective branch
bounds. The pass tested `7` Tier-A interaction pairs and the vectors `c0` and
`c0+p` with `56` face LP solves. The best row (`c0`, pair `305,224`) has large
raw root violation `287016.2194`, but adding it lowers the root LP only
`20.9058`, from `594914.3515` to `594893.4457`. Top rows are highly aligned
with the weight objective (`cosine ~= 0.956`) and therefore behave like
redundant objective/projection cuts rather than a new disjunctive mechanism.
Do not fund two-variable affine lifting on these Tier-A pairs without a new
non-objective aggregate vector.

Web-template update: the only part of the web/clique-family idea that survived
second-agent critique was a tiny web-only structural falsifier. Clique-family
rows were skipped as too close to the retired CG lanes. The diagnostic used
geometric angular order only to propose candidate cyclic supports, then
required graph-only certification that the support contains the web conflict
edges for `k=3..8` plus exact alpha validation. The first high-priority pool
hit the `500` candidate cap with zero structural webs and zero accepted cuts.
Retire web/antiweb template rows on this substrate unless a new support-ordering
principle appears.

Branch-slack symmetry-image update: Mill approved one low-patience diagnostic
of half-turn images of the replayed parent rows, with the hard guard that
vertex `304` must be fixed. The artifact verifies the half-turn is a full
weighted automorphism (`3390/3390` edges preserved, `607/607` weights
preserved) and that `304` is the only fixed vertex. Both projected and
branch-slack image rows are non-duplicate and non-dominating, and validity is
only by automorphism transport of the already replayed parent inequalities, not
by a new gamma certificate. The current solution is not half-turn symmetric, and
the branch-slack image is violated by about `1341473` in denominator-`1024`
units, but adding it lowers the root LP only `0.4547`
(`594914.3515 -> 594913.8969`); the projected image alone moves nothing.
Retire symmetry-image parent rows on this substrate unless a future proof object
creates a different asymmetric parent inequality.

Tailored gamma0 tree update: rebuilding the `x304=0` branch tree directly for
the modified vector `d=c0+p` is retired as a parent-row improvement path, even
though the branch-bound diagnostic itself is strong. Rawls approved this as one
of the few structurally different remaining tests, but required root movement
before any leaf export. Reoptimizing the old gamma0 leaves for `d` showed the
conservative slack-charged bound was essentially tight on that old partition
(only `168` denominator-`1024` numerator units of spare gain). A fresh
best-bound tree changed the partition and closed in `23` expanded nodes /
`24` leaves with diagnostic `gamma0_d = 609058840.6162`, improving the charged
bound by `213080.2959` objective units. But composing the hypothetical parent
row with the already replayed gamma1 side drops the current root only
`208.0114`, from `594914.3515` to `594706.3402`, below the `250` cheap root
gate and far below export gates. Do not export tailored `gamma0(c0+p)` leaves
from this evidence; the stronger branch-bound number is mostly redundant at
the current parent LP.

Tailored gamma0 slack-face update: the fresh `d=c0+p` branch tree does not
provide a second slack-charging direction. Wegener approved one face-only
funding diagnostic, explicitly not parent-valid until a gamma1 side exists.
Charging nonnegative `q` over the tailored tree's nine branch variables
`{456,383,222,152,386,225,305,384,223}` gives `q_support_size = 0`,
`rhs_reduction = 0`, and current face violation `0`. The face LP movement is
only the already-retired `208.0114` from the stronger gamma0 row itself. This
confirms that the tailored tree has no usable conservative residual slack for
a new modifier. Do not fund gamma1 for `d+q`; there is no `q`.

Plateau affine-disjunction update: fitting the Tier-A plateau branch leaves
into one nonnegative affine objective cut is retired. Ampere approved a cheap
diagnostic as a conservative upper envelope:
`w*x + a_T*x_T <= B`, with undecided Tier-A branch variables charged as `1`
and include-neighbor zero implications graph-verified. Reconstructing the full
Tier-A tree gives `16` leaves and the fitted row has coefficients about
`2084..2086` on all six Tier-A variables, `B = 596443.4888`, and raw current
violation `2640.4843`. But adding the row drops the current root only
`139.1847`, weaker than the scalar max-leaf objective cap's `317.3004` drop and
well below the `250` kill gate. Do not fund Plateau affine leaf replay; the
branch disjunction's useful information is still too close to the retired
max-leaf evidence.

Plateau signed-CGLP update: the stronger signed Tier-A disjunctive-hull row is
also retired. Euler approved one bounded diagnostic only as a disjunctive-hull
cut, not native current-LP authority. For each signed coefficient vector the
script recomputes every leaf support value
`sup_L (w*x + a_T*x_T)` under the same row system, then solves a six-variable
cutting-plane master over boxes `{5000,10000,25000}` plus `50000` only when the
previous best is boundary-driven. The best result is boundary-driven at the
`10000` box with all six coefficients essentially `+10000`; raw violation is
again `2640.4843`, but adding the row lowers the root by only `0.2457`
(`594914.3515 -> 594914.1058`), far below the `250` kill gate and much weaker
than the scalar max-leaf cap. This falsifies hidden bite in the Tier-A signed
disjunctive hull. Do not reopen Tier-A disjunctive aggregation without a larger
or different branch family.

Multileaf conditional-rank bundle update: the plateau is brittle leaf-by-leaf,
but not yet enough to fund replay. Banach approved one bounded branch-leaf
proof-object precheck over the top six Tier-A leaves, measuring max movement
rather than summed drops. The diagnostic keeps rows leaf-local and tests up to
`30` exact MWIS candidates per leaf. All six leaves receive meaningful
conditional-rank drops (`1161..2195`), and five neighboring leaves show a
compact-looking repeated pattern: a `wx_dense220_*` row with alpha about
`358075` plus a `wx_dense180_*` row with alpha `258701`; the all-excluded leaf
retains the old `wx_dense220_152` alpha `362026` row. This is real structure.
However the tested-leaf max only moves from `594597.0511` to `593196.5447`,
a `1400.5064` max improvement, still above the `592000` funding gate and below
the `3000` max-movement gate. Treat this as a strong structural signal for a
parameterized leaf-row family, not as replay/export authority yet.

Multileaf residual-rank update: a second residual pass after the first
leaf-local rank family is currently negative under a harsh cap, but the run is
partial. Zeno approved one bounded falsification pass; the first attempt was
too slow, so the script was tightened to checkpoint after each leaf with only
`3` residual MWIS calls per leaf and a nominal `5s` MWIS cap. The checkpoint
artifact completed five of six leaves and found `0` accepted residual rows,
`0` additional meaningful leaf drops, and only `45.9824` additional max
movement (`593196.5447 -> 593150.5623`). The sixth leaf did not finish before
the outer timeout, so this is not a complete six-leaf theorem about residual
absence. Still, five completed leaves show no evidence of a second compact
conditional rank family near the top residual candidates. Do not fund residual
iteration unless the final leaf is revisited with a better exact-MWIS backend
or a much sharper residual candidate prefilter.

Full-tree rank-family update: the deterministic two-template family has now
been tested over all `16` Tier-A leaves. The diagnostic reused known first-pass
alphas by support digest and spent `20/24` allowed new MWIS calls. The pattern
is real enough to accept rows in `8` leaves with exactly `2.0` templates per
leaf, but it misses the funding gates: full-tree max moves only
`594597.0511 -> 593196.8907` (`1400.1605` movement), above the `592000` max
gate and below the `3000` movement gate. The common `dense180` rows repeatedly
lower neighboring leaves to about `593196`, while the all-excluded leaf lands
at `592402.1577`; the family is compact but not strong enough for replay
design. Retire full-tree first-family export unless a new template changes
this ceiling rather than duplicating the `dense180` plateau.

Conditional-rank disjunctive lifting is retired by dominance. Meitner approved
one cheap falsifier for lifting the accepted full-tree conditional rows through
Tier-A branch literals, but implementation revealed a stronger precheck: every
accepted row's `alpha_w` is an ordinary support MWIS value, so
`q*x <= alpha_w` is already a globally valid rank inequality. Any parent lift
with nonnegative branch-literal RHS charges is weaker at the root. The global
dominance diagnostic added all `8` accepted rows to the current root system and
moved only numerical noise (`594914.351525072 -> 594914.351525073`); the best
single row drop is `0.0`, and the best single row is under-saturated at root by
`35617`. Do not fund Tier-A literal lifting of these first-family rows. A
future branch-lift lane needs genuinely conditional bounds, not globally valid
support MWIS rows that only become violated after branching.

Conditional-alpha lifting is also retired. Gibbs approved one bounded
diagnostic for the only loophole left by dominance: maybe the same accepted
supports have lower support suprema on their source Tier-A faces, yielding a
genuinely conditional row. The conservative script used only the `8` accepted
supports, charged only the six Tier-A branch variables, and first computed
source-face support MWIS values with the leaf fixed semantics. No source face
had `beta_source < alpha_w`, so there were `0` admissible conditional-alpha
rows and no 16-face charge fitting was funded. Root objective stays
`594914.351525072`. The first-family row phenomenon is therefore contextual LP
violation, not a hidden lower branch-specific support independence number.

Pair-seeded second-family rank rows are retired under the strict shape gate.
Erdos approved one bounded falsifier only if pair supports were genuinely
two-center rather than decorative reruns of single-center dense rows. The
diagnostic generated balanced pair expansions on the post-first-family top
leaves with Jaccard caps against root, first-family, prior residual, and
corresponding single-center supports. On the first three worst completed leaves
(`1`, `6`, `12`) every generated pair was rejected before MWIS: each leaf had
`50` `single_like` and `10` `unbalanced` rejections, `0` candidates, and `0`
MWIS calls. The strict H3 hypothesis therefore fails at the required
pair-shaped precheck, not at exact MWIS. Do not fund more dense pair-seeded
second-family rows under this backend; relaxing the shape gate would collapse
back into the retired residual single-center separator.

Post-first-family common-`d` CGLP is retired. Turing approved only a tiny
top-six common-vector falsifier with no Tier-A literal coefficients and all
`16` leaf suprema recomputed before any root LP measurement. The cutting-plane
diagnostic learned the uniform average over the six Tier-A variables
(`d_v = 1/6`) with support size `6`, all-16 bound
`B = 1/3`, and root LHS `1/3`, so raw violation is numerical zero
(`-1.7e-16`) and root drop is numerical zero. The scalar post-family max-leaf
cap still drops the root by `1717.46`, but that is not a new non-objective
common-`d` mechanism. Enriched first-family leaf rows do not expose a useful
common nonnegative direction; do not fund more post-family common-`d` CGLP
without a genuinely different vector source or row language.

Post-first-family micro-branch substrate is funded for follow-up. Bacon
approved one LP-only branch-substrate falsifier over the top six enriched
post-first-family leaves. With no new MWIS/supports, each leaf used its
accepted first-family row and branched on the top six remaining heavy
fractional variables. The same compact pool appears in every leaf:
`{152,222,225,383,386,456}`. Depth-3 trees solve in `22` nodes per leaf
without hitting node caps, and the top-six max falls
`593196.8907 -> 586224.2383`, movement `6972.6524`. Every leaf moves by about
`6973`, and repeated branch variables appear in at least four leaves for all
six pool vertices. This clears the continuation gate and is the first
post-branch-slack evidence of a genuinely compact new branch substrate. It is
not parent authority yet: next work should test whether this six-variable pool
can be lifted/exported as a semantic branch-leaf certificate over all relevant
Tier-A leaves, not treated as a solved W607 bound.

Full-16 fixed-pool micro-branch stress test is not export-ready, but it sharply
localizes the obstruction. Lorentz approved one all-leaf stress test using the
fixed residual pool `{152,222,225,383,386,456}`, depth `3`, node cap `24`, no
new rows/supports/MWIS. All `16` leaves ran with no cap hits and total `352`
nodes. The full max moves `593196.8907 -> 589302.6440` (`3894.2466`
movement), but this misses the export feasibility gate `586500`. The new worst
leaf is leaf `0` (the all-excluded Tier-A leaf): it moves only
`592402.1577 -> 589302.6440`, while the top-six leaves stay near
`586224` and many non-top-six leaves move much more. The fixed pool is
therefore not yet a full semantic branch substrate; next work should inspect
leaf `0` against the same pool and determine whether a compact leaf-0 repair
or altered branch substrate can bring the full-16 max below the export gate.

Leaf-0 depth-4 same-pool repair is retired. Halley approved one bounded
leaf-0-only falsification pass, requiring exact reproduction of the depth-3
leaf-0 number before comparing depth `4`, no new supports/MWIS/variables, and
artifact capture of terminal assignments and pool LP states. New
script/artifact:
`crates/hadwiger-research/docs/run_w607_leaf0_depth4_micro_branch.py` and
`crates/hadwiger-research/docs/w607-leaf0-depth4-micro-branch.json`. The run
reproduces the full-16 leaf-0 depth-3 bottleneck
`592402.1577 -> 589302.6440`, then depth `4` only improves to
`588378.8643`: extra movement `923.7797`, below the `1000` continuation gate,
still above the `588000` kill max and the `586500` export gate. The all-excluded
Tier-A leaf therefore is not fixed by simply going one level deeper on
`{152,222,225,383,386,456}`. Next leaf-0 work should change the repair
language, likely by testing an altered compact pool or a leaf-0-specific
conditional row, not by extending this same-pool tree depth.

Leaf-0 augmented-pool-304 repair is retired. Harvey approved one narrow
comparison after the depth-4 bottleneck showed vertex `304` (one-based;
internal index `303`) as the heaviest outside-pool fractional variable at
`x=0.247156`, score `8945.82`. New script/artifact:
`crates/hadwiger-research/docs/run_w607_leaf0_augmented_pool304.py` and
`crates/hadwiger-research/docs/w607-leaf0-augmented-pool304.json`. The run
compared the old pool `{152,222,225,383,386,456}` to augmented pool
`{152,222,225,304,383,386,456}` on leaf `0` only, depths `3` and `4`, node cap
`80`, no new rows/supports/MWIS. Reproduction gates passed, but the augmented
tree is identical to the old tree at both depths: depth `3` remains
`589302.6440`, depth `4` remains `588378.8643`, extra movement is `0.0`, and
vertex `304` is never selected in the augmented branch traces. Treat `304` as
a terminal-specific decoy, not a branch-substrate variable. Do not fund more
single-variable augmentation from the depth-4 terminal unless the candidate is
shown to enter the actual best-bound path.

Leaf-0 residual-pair closure is a strong positive lead. Schrodinger approved
one bounded diagnostic to test whether the retired same-pool depth-4 leaf0
bottleneck was caused by the residual two-literal ambiguity `{222,456}` rather
than a missing outside variable. New script/artifact:
`crates/hadwiger-research/docs/run_w607_leaf0_residual_pair_closure.py` and
`crates/hadwiger-research/docs/w607-leaf0-residual-pair-closure.json`. The run
uses the same enriched leaf0 row system, old pool `{152,222,225,383,386,456}`,
no new rows/supports/MWIS/variables, and reproduces depth `3`
`589302.6440` and depth `4` `588378.8643`. Exactly one depth-4 terminal remains
above the export gate: assignment `{152=0,225=0,383=0,386=0}` with both `222`
and `456` unfixed, fractional at `1/3`. Splitting the residual pair costs only
`4` extra LP solves and closes the node with children max `581481.0000`; final
closed leaf0 max is `581481.0000`, no cap/timeout, no closed terminal above
`586500`, status `strong_residual_pair_closure`. Next funded test should ask
whether this residual-pair closure can be made into a uniform full-16 substrate
rule: old fixed-pool depth `3` everywhere, exceptional leaf0 depth `4` plus
pair closure, and then check the new full-16 max and export/lift shape.

Full-16 mixed residual closure now funds export/lift design, but only as a
diagnostic branch certificate shape, not as root proof authority. Huygens
approved one narrow export-shape artifact with the explicit failure mode
`authority contamination`. New script/artifact:
`crates/hadwiger-research/docs/run_w607_full16_mixed_residual_closure.py` and
`crates/hadwiger-research/docs/w607-full16-mixed-residual-closure.json`. It
combines the full-16 fixed-pool depth-3 stress with the leaf0 depth-4
residual-pair closure, verifies source reproduction gates, records row-system
digests, and labels root/parent rows as proof-substrate authority while
labeling fixed-pool trees and leaf0 residual closure as diagnostic branch
authority only. Result: old depth-3 full-16 max `589302.6440`; mixed full-16
max `586224.2383`, argmax leaf `1`, margin `275.7617` below the `586500`
export gate, total nodes plus extra solves `356`, no cap/timeout, no new
rows/supports/MWIS/variables. Status `fund_export_lift_design`. Next work
should not claim a proof; it should design an export/lift path that turns this
mixed diagnostic branch certificate into a verifiable parent/root object or
identifies the precise obstruction.

Fresh mixed-branch replay independently validates the export-shaped diagnostic
and clears the stale-artifact concern. Avicenna approved this as a necessary
gate before lift/export work, warning that simply copying prior terminal
bounds would not prove semantic replay. New script/artifact:
`crates/hadwiger-research/docs/run_w607_fresh_mixed_branch_replay.py` and
`crates/hadwiger-research/docs/w607-fresh-mixed-branch-replay.json`. The final
strict run used `--no-checkpoint`: it loaded no prior full-16 stress artifact,
no mixed artifact, no leaf0 closure artifact, and no own replay checkpoint.
It reconstructed all `16` first-family-enriched leaf row systems from
`w607-full-tree-rank-family.json`, reran depth `3` fixed-pool enumeration over
`{152,222,225,383,386,456}` for every leaf, applied the exceptional leaf0
depth `4` residual-pair closure `{222,456}` only on leaf `0`, and emitted
terminal certificate tables. Result: final mixed max `586224.2383`, argmax leaf
`1`, margin `275.7617` below the `586500` export gate, no cap/timeout, no new
rows/supports/MWIS/branch variables, and no failure reasons. This is still
diagnostic branch authority only, but it is now replayed from source rather
than assembled from stale artifacts; next hypothesis should attack the actual
lift/export problem.

Pool-affine terminal-mask lift is retired as scalar-cap equivalent. Hume
approved one bounded bridge test with the key semantic constraint that free
pool variables must be charged as worst-case `1` in every terminal mask. New
script/artifact:
`crates/hadwiger-research/docs/run_w607_pool_affine_lift_probe.py` and
`crates/hadwiger-research/docs/w607-pool-affine-lift-probe.json`. The probe
used the fresh mixed replay terminal table, pool `{152,222,225,383,386,456}`,
nonnegative coefficients, mask rule `fixed_0 -> 0`, `fixed_1 -> 1`, and
`free/unresolved -> 1`, then measured the induced row in the root LP. The LP
learned all-zero coefficients, `B = 586224.2383`, active terminal leaf `1`,
and the post-row objective/drop exactly matches the scalar max-bound row:
`586224.2383`, drop `8690.1133`, `drop_minus_scalar_drop = 0.0`. Failure
reasons: `zero_coefficients`, `raw_violation_below_gate`,
`scalar_cap_equivalent_or_weaker`. This confirms that a conservative
nonnegative six-pool affine mask does not lift the branch certificate; any
actual export must use richer structure than free-variable worst-case masks.

Terminal-dual provenance is inconclusive but rules out the simplest
literal-bound obstruction. Kant approved a bounded dual provenance probe on
all high terminals within `1000` of the mixed max plus the leaf0 residual
children, with the warning that HiGHS duals are nonunique. New script/artifact:
`crates/hadwiger-research/docs/run_w607_terminal_dual_provenance.py` and
`crates/hadwiger-research/docs/w607-terminal-dual-provenance.json`. The probe
re-solved `10` selected terminal LPs, reproduced objectives within
`5.9e-10`, and decomposed positive HiGHS dual mass by row family. Aggregate
fixed-bound/literal mass ratio is `0.0`, so the active certificate is not
powered by terminal-specific fixed-bound duals. Aggregate positive mass is
structural: edges `837329.82`, triangles `3512047.22`, parent lifts
`1830935.63`, first-family leaf rows `309516.43`. However, the common
structural row intersection across selected terminals is empty, mostly because
top-six leaves each use their own leaf-local first-family row and different
edge/triangle rows. Conclusion `dual_degeneracy_inconclusive`; failure reason
`common_structural_core_too_small`. Next export direction should not chase
more fixed-literal diagnostics; it should either align the leaf-local
first-family rows into a replayable family object or construct an explicit
disjunctive/literal lift.

First-family dense180 alignment is a positive family-export lead. Peirce
approved one bounded falsifier, warning that identical template labels and
equal alpha values could be false family signals. New script/artifact:
`crates/hadwiger-research/docs/run_w607_first_family_alignment_probe.py` and
`crates/hadwiger-research/docs/w607-first-family-alignment-probe.json`. The
probe reconstructed the six active top-leaf rows with
`template_id = dense180_top_wx_center_2`, `alpha_w = 258701`, size `180`, and
centers `{152,152,225,383,383,456}` across leaves `{1,2,4,6,9,12}`. Result:
common core size `109`, union size `217`, pairwise Jaccard range
`0.7734..0.8090`, average `0.7912`; all six share identical invariant
signature: size `180`, weight sum `853635`, pool incidence
`{152,222,225,383,386,456}`, empty Tier-A incidence, internal edges `785`,
internal triangles `370`, alpha `258701`. Strict center-shell signatures vary
slightly, but the invariant group contains all six leaves. Dual contribution
from the leaf-local first-family row is also stable around `32084`. Status
`family_export_funded`. Next export hypothesis should try to package this
dense180 family as a reusable disjunctive family object or common-core/variant
certificate, not as unrelated local rows.

Dense180 common-core packaging retires the core row but funds a variant
disjunction route. Dewey approved one bounded diagnostic with at most `8` MWIS
solves to test whether the six active dense180 rows package as a 109-vertex
common core plus 71-vertex variants. New script/artifact:
`crates/hadwiger-research/docs/run_w607_dense180_core_packaging_probe.py` and
`crates/hadwiger-research/docs/w607-dense180-core-packaging-probe.json`.
Results: common core alpha `210431`, but root lhs only `137372` and root drop
numerical zero; union alpha `308031`, root lhs `268792`, root drop numerical
zero. Thus core and union rows are polyhedrally inert at root. However, all six
71-vertex variants solve with identical alpha `115942` (`variant_alpha_spread =
0`), and variant pairwise Jaccard remains structured (`0.5106..0.5778`).
Status `fund_variant_disjunction`; failure reason only
`core_row_root_drop_below_gate`. Next export hypothesis should target an
explicit dense180 variant-disjunction/literal lift, not a global core or union
row.

Dense180 variant literal preflight funds a singleton-Tier-A disjunction route.
Singer approved a bounded preflight while warning that six active leaves are
not automatically a root-valid disjunction. New script/artifact:
`crates/hadwiger-research/docs/run_w607_dense180_variant_literal_preflight.py`
and
`crates/hadwiger-research/docs/w607-dense180-variant-literal-preflight.json`.
Result: the six active dense180 variants align exactly with the six singleton
Tier-A inclusion faces. Active leaves `{1,9,2,12,6,4}` cover included Tier-A
vertices `{223,224,303,305,384,385}` one-to-one; there are no singleton
Tier-A leaves outside the active set. All six variants have size `71`, alpha
`115942`, and the dense180 invariant signature count is `1`. All non-active
leaves are already below the `586500` gate, with worst non-active leaf `0` at
`581481.0000`. Status `fund_singleton_literal_disjunction`. This still is not
a root cut; it identifies the precise disjunctive lift target: singleton
Tier-A faces get dense180 variant rows, while non-singleton/zero-inclusion
faces are covered by the existing mixed branch certificate.

All-16 Tier-A affine literal aggregation is retired as scalar-equivalent.
Aquinas approved one bounded diagnostic requiring semantic masks over all
`16` Tier-A leaves, independently sourced mixed leaf bounds, scalar-cap
comparison, and singleton-control checks. New script/artifact:
`crates/hadwiger-research/docs/run_w607_tier_a_mixed_affine_lift.py` and
`crates/hadwiger-research/docs/w607-tier-a-mixed-affine-lift.json`. The fit
learns nonzero nonuniform coefficients on Tier-A vertices
`{224,303,305,385}`, but the induced root LP objective is exactly the scalar
mixed cap within numerical noise: `586224.2383`, drop `8690.1133`, with
`drop_minus_scalar_drop = 6.98e-10`. Tight leaves are singleton-only
`{1,2,4,9,12}`, and the failure reasons are
`scalar_cap_equivalent_or_weaker` and `singleton_only_active_control`. This
confirms that naive nonnegative Tier-A affine lifting does not export the
branch certificate; the next route needs a stronger proof object than a
single scalar-dominated literal row.

Selected-terminal exact-dual export preflight is retired in its row-only form.
Popper approved a tightly scoped proof-plumbing test over the six near-max
normal terminals plus leaf0's exceptional residual-pair shape. New
script/artifact:
`crates/hadwiger-research/docs/run_w607_selected_terminal_dual_export_preflight.py`
and
`crates/hadwiger-research/docs/w607-selected-terminal-dual-export-preflight.json`.
The script reconstructs terminal row systems from graph/root/parent/
first-family sources and selected assignments, reproduces all floating
terminal objectives, and confirms fixed-branch bound duals are zero. But every
required terminal fails exact row-only export because nonfixed variable-bound
dual mass is nonzero (`~100k..350k` over `8..16` bound rows per terminal). At
coarse denominators the huge parent-row scaling creates enormous objective
overhead; at high denominator structural row-only coverage goes negative.
Status `retire_selected_terminal_export_preflight`. The next plausible
bounded test is an explicit-bound-row formulation, if singleton upper/lower
bound rows are accepted as legitimate proof rows rather than hidden solver
artifacts.

Explicit-bound selected-terminal export is partially positive but still
retired under the canonical denominator gate. Euclid approved one bounded
preflight allowing explicit `0 <= x <= 1` and branch-fixing rows, with
lower-bound rows reported but not upward-rounded in the canonical export. New
script/artifact:
`crates/hadwiger-research/docs/run_w607_explicit_bound_terminal_dual_export_preflight.py`
and
`crates/hadwiger-research/docs/w607-explicit-bound-terminal-dual-export-preflight.json`.
After correcting the formulation so branch fixed assignments and global bounds
are represented as explicit rows with no hidden solver bounds, all four leaf0
residual children export cleanly (`3` at denominator `1024`, one at `4096`;
`592..597` positive rows, exact min slack `0`). The six near-max depth-3
terminals still fail only the objective gate: exact coverage is positive,
hidden solver bound marginals are zero, and row counts are `596..602`, but
rounding the tiny parent-lift multiplier (`~4.89e-7` on RHS `623894447014`)
causes about `29.5k` overhead even at denominator `16777216`, far above the
`275.76` margin to `586500`. Status
`retire_explicit_bound_terminal_export_preflight`. Next bounded question:
whether large-denominator/bigint terminal export is acceptable proof plumbing,
or whether the parent row needs rational reconstruction/rescaling.

Big-denominator selected-terminal export is also partially positive and
sharply localizes the remaining compactness obstruction. Maxwell approved one
bounded probe with denominator ladder `{2^24,2^28,2^32,2^36}` and bit caps.
New script/artifact:
`crates/hadwiger-research/docs/run_w607_bigdenom_terminal_export_probe.py`
and `crates/hadwiger-research/docs/w607-bigdenom-terminal-export-probe.json`.
Result: `8/10` required selected terminals export. All four leaf0 residual
children still pass; four of the six near-max depth-3 terminals pass at
`2^32` with objectives around `586329.878..586340.623`, exact positive slack,
and `596..602` positive rows. Leaves `12` and `9` also clear
objective/coverage at `2^32`, but fail the policy bit cap:
`max_multiplier_numerator_bits = 49` versus gate `48`; at `2^36` they have
tiny objective overhead (`~7`) but fail the same bit cap more strongly. Status
`retire_bigdenom_terminal_export_probe` under Maxwell's strict gates.
Interpretation: selected terminal export has no apparent row-semantics
obstruction; the remaining issue is certificate compactness/presentation,
likely requiring parent-row multiplier rational reconstruction or an
explicitly relaxed bit cap after review.

Per-row rational selected-terminal export fixes the near-max parent-rounding
obstruction but exposes a tiny coverage-repair need. McClintock approved an
exact replay filter where HiGHS duals are only candidate multipliers and
authority comes from exact Fraction coverage/objective replay. New
script/artifact:
`crates/hadwiger-research/docs/run_w607_rational_terminal_export_probe.py`
and `crates/hadwiger-research/docs/w607-rational-terminal-export-probe.json`.
Result: all six near-max depth-3 terminals export with exact rational
objective essentially equal to the floating objective, positive exact slack,
`596..602` rows, and compact parent multipliers. Two leaf0 residual children
also export at cap `1e6`; two fail only from below-rational coverage deficits.
Status `retire_rational_terminal_export_probe`.

Uniform upward rationalization and targeted exact deficit repair are both
retired as presentation mechanisms. The uniform upward policy repairs the
leaf0 children but fails near-max terminals by adding too much objective; the
targeted exact-increment repair explodes denominators/numerator bits. New
script/artifacts:
`crates/hadwiger-research/docs/run_w607_upward_rational_terminal_export_probe.py`,
`crates/hadwiger-research/docs/w607-upward-rational-terminal-export-probe.json`,
`crates/hadwiger-research/docs/run_w607_targeted_rational_terminal_repair.py`,
and
`crates/hadwiger-research/docs/w607-targeted-rational-terminal-repair.json`.
These failures support a mechanism-class presentation rather than a universal
rounding style.

Hybrid selected-terminal certificate policy succeeds as proof-plumbing
preflight. Leibniz approved a mechanism-class policy rather than one universal
multiplier style. New script/artifact:
`crates/hadwiger-research/docs/run_w607_hybrid_terminal_export_preflight.py`
and
`crates/hadwiger-research/docs/w607-hybrid-terminal-export-preflight.json`.
The predeclared class policy is
`top_six_depth3_parent_lift -> per_row_rational_reconstruction` and
`leaf0_residual_child -> common_denominator_upward_integer`. The artifact
regenerates row systems from source, uses explicit bound/fix rows, and
verifies exact coverage/objective for all selected terminals. Result:
`10/10` required selected terminals pass, total positive rows `5971`, worst
selected objective `586224.2382592709`, no failure reasons, status
`fund_full_mixed_tree_terminal_export_design`. Authority remains selected
terminal preflight only, not root theorem or full mixed-tree export authority.
This is the strongest validation milestone so far: the high branch-bound
terminals are exact-certificate friendly under a non-ad-hoc mechanism-class
policy.

Full mixed-tree terminal export preflight succeeds and funds the replay
checker. Tesla approved H25 with strict source binding to the fresh mixed
branch replay, an expected all-terminal manifest count of `135`, no duplicate
terminal keys, exact objective/coverage gates, a `100000` positive-row budget,
and authority limited to terminal export preflight. New script/artifact:
`crates/hadwiger-research/docs/run_w607_full_terminal_export_preflight.py`
and
`crates/hadwiger-research/docs/w607-full-terminal-export-preflight.json`.
After correcting an H25-only helper routing bug that classified every manifest
terminal as audit-only, the strict run passes: `135/135` terminals exported,
one triggered leaf0 depth-4 terminal correctly replaced by four residual
children, total positive rows `80143`, worst exact terminal objective
`586224.2382592709`, and no failure reasons. The predeclared class policy is
validated across the full mixed tree: six high parent-lift depth-3 terminals
use per-row rational reconstruction, while the four leaf0 residual children,
eleven ordinary leaf0 closed terminals, and `114` ordinary non-leaf0 depth-3
terminals use compact common-denominator upward integer certificates. Status
`fund_full_mixed_tree_terminal_replay_checker`. This is not yet root theorem
authority; the next required validation is a Rust replay checker that verifies
the semantic branch partition and all terminal certificate rows from source.

Full-terminal Rust replay checker succeeds as the next authority layer.
Franklin approved H26 while warning that a summary-only JSON checker would be
too weak, so the H25 exporter was upgraded to emit row-level payloads:
every positive proof row now carries explicit integer coefficients and exact
`{num, den}` multipliers. New Rust checker/support/example:
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_full_terminal_export_replay.rs`,
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_full_terminal_export_support.rs`,
and
`crates/hadwiger-research/examples/replay_g27_w_circles_full_terminal_export.rs`.
The checker digest-binds the H25 artifact, verifies manifest shape and
mechanism classes, rejects duplicate semantic terminal keys, replays exact
rational coverage/objective from the exported coefficients over all `607`
vertices for all `135` terminals, enforces objective gates and the `100000`
row budget, and confirms the argmax terminal. The replay command:
`cargo run -p hadwiger-research --example replay_g27_w_circles_full_terminal_export`
prints:
`terminals 135 rows 80143 worst_objective_floor 586224 min_slack_floor 0 status ReplayedFullTerminalExportPreflight`.
This is meaningful proof-plumbing progress: the full terminal export no longer
depends only on Python solver state. It is still not a root theorem; the next
authority gap is semantic branch partition/root composition from this
terminal-replay checker.

Semantic branch partition composition succeeds as H27 preflight authority.
Bernoulli approved the hypothesis as the real gap after H26: terminal replay
proved each listed terminal certificate, but not that the listed terminals
exactly partition the declared branch search. New checker/example:
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_semantic_partition_replay.rs`
and
`crates/hadwiger-research/examples/replay_g27_w_circles_semantic_partition.rs`.
The checker raw-digest-binds the fresh mixed replay and row-carrying H26
terminal artifact, then composes with the H26 exact terminal replay. It verifies
the `16` first-stage Tier-A clauses are complete and disjoint over the `64`
Boolean assignments of `{223,224,303,305,384,385}`; every non-leaf0 leaf's
eight depth-3 fixed-pool terminal clauses partition the `64` assignments of
`{152,222,225,383,386,456}`; leaf0's depth-4/residual-closure clauses partition
the same pool cube after excluding the triggered pre-split terminal and
replacing it with four `{222,456}` residual children; and the exported terminal
pool assignments match the fresh replay. Command:
`cargo run -p hadwiger-research --example replay_g27_w_circles_semantic_partition`
prints:
`tier_assignments 64 terminals 135 rows 80143 status SemanticPartitionTerminalCompositionPreflight`.
This closes the semantic partition/root-composition preflight layer for the
declared mixed diagnostic branch partition. It is still not final theorem
authority: remaining work is to decide whether full source-row reconstruction
and theorem packaging are sufficient for publication, or whether another
native root-level proof object is required.

Row-family semantics replay succeeds as H28 preflight authority. Boole
approved the hypothesis as the remaining direct attack on H26/H27: exact
terminal coverage and semantic partitioning still needed proof that the rows
were legitimate W607 proof rows rather than arbitrary exported coefficients.
New checker/example:
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_row_family_semantics_replay.rs`
and
`crates/hadwiger-research/examples/replay_g27_w_circles_row_family_semantics.rs`.
The checker composes with H27/H26 and validates all `80143` positive rows:
edge rows against the retained exact W607 edge set and row ids; triangle rows
against the retained edge graph; fixed literal rows against combined Tier-A
plus pool assignments; first-family leaf rows against source support digests,
support sizes, alpha/RHS, and W607 integer weights as coefficients; and parent
lift rows against regenerated coefficient vectors from the existing
digest-bound projected-parent and branch-slack lift artifacts. Command:
`cargo run -p hadwiger-research --example replay_g27_w_circles_row_family_semantics`
prints:
`checked_rows 80143 parent_lift_rows 6 status RowFamilySemanticsPreflight`.
Authority is now row-family semantics plus semantic branch partition plus exact
terminal replay for the declared mixed diagnostic partition. Remaining theorem
gap is narrow and explicit: final packaging/source reconstruction audit,
especially whether matching parent vectors to existing digest-bound lift
artifacts is enough for theorem authority or needs a from-first-principles
parent-lift derivation inside the final checker.

Parent-lift provenance/readiness replay succeeds as H29 preflight authority.
Hegel approved the hypothesis only if it composed H28 with both standalone
lift replayers and refused to promote the result to theorem authority. New
checker/example:
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_parent_lift_readiness_replay.rs`
and
`crates/hadwiger-research/examples/replay_g27_w_circles_parent_lift_readiness.rs`.
The checker composes the H28 row-family semantics replay, the projected
parent-lift replay, and the branch-slack lift replay; enforces the exact H28
summary `80143` checked rows and `6` parent-lift row occurrences; enforces the
projected lift summary `(304, 613372392, 546085806, 67286586)`; enforces the
branch-slack lift summary `(623894447014, 559085319025, 64809127989, 8555)`;
then parses the terminal row artifact and verifies that the six exported
terminal parent-lift rows use the canonical `parent_lift_1` RHS and coefficient
vector regenerated from the digest-bound branch-slack/projection sources, with
no unknown parent-lift ids. It also payload-records SHA256 digests for the
terminal rows, fresh mixed replay, first-family source, exclude certificate,
gamma0/gamma1 leaf-dual exports, and branch-slack lift artifact. Command:
`cargo run --release -p hadwiger-research --example replay_g27_w_circles_parent_lift_readiness`
prints:
`checked_rows 80143 parent_lift_rows 6 theorem_authority false ids ["parent_lift_1"] status ParentLiftProvenanceReadinessPreflight`.
Conclusion:
`validated parent-lift provenance for 6 terminal row occurrences across ["parent_lift_1"]; proof-plumbing readiness only, not root theorem authority`.
Verification passed:
`cargo check -p hadwiger-research --example replay_g27_w_circles_parent_lift_readiness`
and `cargo fmt -p hadwiger-research --check`. Authority is now a strict
proof-plumbing/readiness chain for the declared mixed diagnostic partition:
terminal replay + semantic partition + row-family semantics + parent-lift
provenance. It is still not a final W607 theorem claim or Hadwiger-Nelson
record claim; the remaining gap is the external theorem-admission step that
decides whether this diagnostic mixed-branch certificate chain is accepted as
the formal root proof object.

W607 certificate admission/theorem-gap replay succeeds as H30 boundary
authority. Cicero approved the hypothesis only as a hard theorem-boundary audit:
convert H25-H29 into a precise machine-checkable scoped statement, then block
the actual W607/HN target rather than dressing the proof plumbing up as a
theorem. New checker/example:
`crates/hadwiger-research/src/frontier_seeds/g27_w_circles_certificate_admission_gap_replay.rs`
and
`crates/hadwiger-research/examples/replay_g27_w_circles_certificate_admission_gap.rs`.
The checker composes H26 exact terminal replay, H27 semantic partition, H28 row
family semantics, and H29 parent-lift readiness. It then recomputes the exact
worst terminal objective over the row-carrying terminal artifact, enforces the
rounding policy `floor = 586224` and `ceil = 586225`, compares that admitted
scoped ceiling to the W607 target weighted alpha `512933`, and requires
`theorem_authority = false`. Command:
`cargo run --release -p hadwiger-research --example replay_g27_w_circles_certificate_admission_gap`
prints:
`status CertificateAdmissionBlockedTarget admitted_scope DeclaredMixedTerminalPartition admitted_bound_floor 586224 admitted_bound_ceil 586225 target_bound 512933 target_pass false theorem_authority false`.
Blockers:
`blocked_target_bound: admitted scoped bound ceil 586225 exceeds target 512933`;
`blocked_hn_claim: weighted-alpha certificate is not a plane chromatic lower-bound claim`;
`blocked_theorem_authority: scope remains declared mixed terminal partition, not final W607 root theorem`.
Verification passed:
`cargo check -p hadwiger-research --example replay_g27_w_circles_certificate_admission_gap`
and `cargo fmt -p hadwiger-research --check`. This is valuable publishable
proof-engineering progress because it makes the current validation boundary
machine-readable and hard to overclaim. It is not new mathematical movement
toward the `512933` target; the next funded hypothesis must either lower the
certificate bound by about `73292` or bridge to a different theorem-bearing
claim.

Same-field odd-cycle branch preflight succeeds as H31 diagnostic movement.
Newton approved the hypothesis only as a tightly capped falsification test:
the retired branch route used weak clique-cover node bounds, while the known
same-field near-miss root odd-cycle certificate was `543428`, so the question
was whether clique+odd-cycle LP node bounds compound under shallow branching.
New checker/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_odd_cycle_branch_preflight.rs`
and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_odd_cycle_branch.rs`.
The checker freezes the same instance `G27 8 -> W 301`, atom mask
`101719589`, exact-side component weight `61655`, dominant threshold `451278`,
and target total `512933`; recomputes the root clique+odd-cycle LP ceiling;
requires root reproduction at or below `543428`; then runs a strict bounded
branch preflight using the same component split and branch semantics as the
old clique-cover branch preflight, but with fresh clique+odd-cycle LP node
bounds. Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_odd_cycle_branch`
prints:
`atom_mask 101719589 dominant_vertices 521 exact_side_weight 61655 dominant_threshold 451278 root_total_odd_cycle_upper 543428 best_open_total_odd_cycle_upper 518612 nodes 29 open_nodes 28 pruned 2 max_depth 6 elapsed_millis 307672 odd_cycle_cuts 16600 max_node_millis 8337 status PromisingOddCycleBranchBound theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_odd_cycle_branch`
and `cargo fmt -p hadwiger-research --check`. This clears the H31 continuation
gate: the best open total moved by `24816` from the reproduced root, and two
threshold prunes appeared. It still does not prove the target: the best open
bound is `5679` above `512933`, node LPs are slow, and the result has no
theorem authority because the odd-cycle rows are solver-derived diagnostics
without rational replay metadata. Next funded step should either turn this
promising branch into a replayable/rational row certificate, or run a second
small capped stage only if it preserves deterministic row metadata and has a
clear below-target/prune gate.

Same-field odd-cycle row replay succeeds as H32a kill-switch preflight.
Lagrange approved H32 only after splitting it into row replay first and
rational dual/objective authority later: row metadata alone can validate
constraint semantics and determinism, but cannot certify the floating LP
ceiling as theorem authority. New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_odd_cycle_row_replay.rs`,
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_odd_cycle_row_replay_support.rs`,
and
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_odd_cycle_rows.rs`.
The replay freezes the H31 instance, regenerates root plus the first two
threshold-pruned nodes, carries clique rows and odd-cycle rows, stores both
canonical support and ordered odd-cycle witnesses, verifies clique validity and
every odd-cycle witness against the residual graph, then runs the whole replay
twice and requires identical row digests. Command:
`cargo run --release -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_rows`
prints:
`root_total_odd_cycle_bound 543428 pruned_nodes 2 checked_nodes 3 clique_rows 2706 odd_cycle_rows 859 max_odd_cycle_length 21 metadata_bytes 99988 row_digest c366ebe9e7b5da4ebcceba2da2777a7b8b4fa1a0b067e99819a6ab30bb9cbde4 status RowReplayStablePreflight theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_rows`
and `cargo fmt -p hadwiger-research --check`; touched Rust files remain below
the `400` line cap. This passes the H32a artifact-size and stability gates:
root plus two prunes need only about `100 KB` of row metadata, and the digest
is byte-stable across consecutive runs. It still does not prove the branch
bound, because objective ceilings are solver-derived and lack exact/rational
dual multipliers. Next funded hypothesis should be H32b: attempt exact/rational
dual replay for these same three final row systems; if that fails, same-field
branch expansion remains exploration-only and theorem work should return to
the W607 bound-lowering pipeline.

Same-field odd-cycle exact dual replay fails as H32b nearest-rational probe.
Jason approved H32b only as a one-sitting proof-language falsifier: include
all rows actually present in the LP, solve or export a complete dual candidate,
then exact-check rational coverage/objective without hidden solver authority.
New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_odd_cycle_dual_replay.rs`,
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_odd_cycle_dual_replay_support.rs`,
and
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_odd_cycle_duals.rs`.
The checker formulates the node LP dual directly as a primal minimization over
explicit rows: singleton upper-bound rows, edge rows, clique rows, and
odd-cycle rows. It then rationalizes positive dual multipliers with
denominator cap `1000000` and verifies exact candidate-weight coverage and
objective ceilings using BigInt rational arithmetic. Command:
`cargo run --release -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_duals`
prints:
`checked_nodes 3 certified_pruned_nodes 0 explicit_rows 4205 positive_dual_rows 518 root_total_bound 0 max_denominator 999760 min_slack_floor -1 max_objective_excess 0 status DualCoverageFailed theorem_authority false`.
This is a useful failure, not a certificate: the complete dual formulation is
accessible and compact, but nearest rational reconstruction undershoots exact
coverage by less than one unit somewhere before root/pruned-node objective
authority can be admitted. Same-field branch expansion is still not
proof-carrying. A tightly bounded follow-up may test conservative
round-from-above rationalization or explicit singleton deficit repair only if
the exact objective ceilings remain identical; otherwise H32b retires this
route back to branch/terminal export design or the W607 bound-lowering
pipeline.

Same-field one-sided rational dual replay succeeds as H32c scoped node
certificate preflight. Boyle approved the follow-up only as a predeclared
one-sided rationalization test: every positive floating dual multiplier is
rounded upward to a rational with denominator cap `1000000`, with no new rows,
no tolerance-based coverage, and exact objective ceilings required to remain
unchanged. New example:
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_odd_cycle_one_sided_duals.rs`.
Command:
`cargo run --release -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_one_sided_duals`
prints:
`checked_nodes 3 certified_pruned_nodes 2 explicit_rows 11830 positive_dual_rows 1496 root_total_bound 543428 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 status OneSidedOddCycleNodeDualReplayPreflight theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_duals`,
`cargo check -p hadwiger-research --example replay_g27_same_field_mwis_odd_cycle_one_sided_duals`,
and `cargo fmt -p hadwiger-research --check`. This is the first exact/rational
authority layer over the H31 same-field prunes: root remains exactly `543428`,
both threshold-pruned nodes certify under the dominant threshold, and exact
minimum coverage slack is nonnegative. It is still scoped node-certificate
authority only, not a full branch proof or HN theorem claim. Next funded step
should package the entire H31 explored tree/open frontier into a row-carrying
branch certificate with these one-sided rational node duals, then ask whether
the open frontier can be driven below `512933` with proof-carrying nodes.

Same-field branch-prefix packaging succeeds as H33 proof-plumbing preflight.
Fermat approved this only as a branch-semantics wrapper around the already
scoped H32c node certificates, not as bound progress. New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_branch_prefix_replay.rs`
and
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_branch_prefix.rs`.
The replay first composes the H32c one-sided rational dual gate, requiring
two exact certified prunes, root total `543428`, nonnegative exact coverage
slack, and zero objective excess. It then rebuilds the frozen H31 branch
prefix twice, carrying semantic node identity (`branch_included`,
`forced_included`, `excluded`, candidates, chosen weight, and depth), checks
branch-child semantics, and requires identical prefix summaries/digests.
Command:
`cargo run --release -p hadwiger-research --example replay_g27_same_field_mwis_branch_prefix`
prints:
`expanded_nodes 29 pruned_nodes 2 open_frontier_nodes 28 best_open_total_bound 518612 root_total_bound 543428 h32c_certified_prunes 2 status BranchPrefixSemanticsPreflight theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example replay_g27_same_field_mwis_branch_prefix`
and `cargo fmt -p hadwiger-research --check`; the new replay file remains
under the 400-line cap. This validates the asterisk around H31/H32c: the two
exact rational prunes are now embedded in a deterministic replayable branch
prefix, while the 28 open frontier nodes still leave best-open total `518612`,
which is `5679` above target. Next funded step needs proof-carrying branch
continuation or a stronger node-bound family for the open frontier.

Same-field top-frontier shape diagnostic succeeds as H34a and redirects the
continuation plan. Plato warned that expanding only the single worst H33 open
node would be misleading if the second-best frontier node was close. New
files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_frontier_shape.rs`
and
`crates/hadwiger-research/examples/diagnose_g27_same_field_mwis_frontier_shape.rs`.
The diagnostic composes H33, requires the exact prefix summary
`expanded_nodes 29`, `pruned_nodes 2`, `open_frontier_nodes 28`, and
`best_open_total_bound 518612`, then reconstructs the open frontier and reports
the top open totals. Command:
`cargo run --release -p hadwiger-research --example diagnose_g27_same_field_mwis_frontier_shape`
prints:
`open_frontier_nodes 28 tied_band_nodes 10 gap_to_second 69 best_open_total_bound 518612 top_open_totals [518612, 518543, 518471, 518441, 518343, 518123, 518105, 517777, 517741, 517649] top_open_depths [5, 4, 3, 3, 5, 6, 6, 6, 6, 6] status TiedFrontierBandRequiresTopKContinuation theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example diagnose_g27_same_field_mwis_frontier_shape`
and `cargo fmt -p hadwiger-research --check`; the new module is under the
400-line cap. Interpretation: H34 single-peek continuation is retired before
execution because the frontier is tightly tied (`gap_to_second 69`; ten nodes
within `1000`). The next funded step should be a proof-carrying continuation
over the top frontier band, not one worst node.

Same-field top-band collapse preflight retires H34b under the bounded wall
clock. Lovelace approved the hypothesis only as a global best-first continuation
over the tied top band, not round-robin per node. New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_top_band_collapse.rs`
and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_top_band_collapse.rs`.
The checker composes H33, reconstructs the H31 open frontier, selects the
H34a top band (`top_k 10`, `band_width 1000`), and continues with one global
priority queue under caps `20` total expansions, `3` per origin, and `300s`.
It reports success only if the recomposed global best drops by at least `1000`
to `517612` or better, or if a new threshold prune appears for exact dual
promotion. Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_top_band_collapse`
prints:
`initial_best_total 518612 final_best_total 518543 final_tied_band_nodes 9 selected_origin_count 10 expanded_nodes 1 solver_pruned_descendants 0 open_frontier_nodes 29 elapsed_millis 302401 status BoundedContinuationNoUsefulProgress theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_top_band_collapse`
and `cargo fmt -p hadwiger-research --check`; the new module is under the
400-line cap. Interpretation: with current clique+odd-cycle node LPs, the
tied band does not collapse quickly enough to justify this continuation lane.
The release run used almost the full five-minute continuation cap after H33
composition, expanded only the top node once, produced no new solver prunes,
and improved the global best by only `69`. Next funded hypothesis should
change the node-bound family or branching heuristic rather than spend more
wall clock on the same top-band continuation.

Same-field LP-guided branch diagnostic succeeds as H35 and revives the branch
lane with a different heuristic. Ramanujan approved this only as a cheap
falsifier of the branch heuristic, not as a proof lane: compare the current
`degree * weight` branch against an LP-guided branch on the H34a top frontier,
using worst-child odd-cycle totals as the comparator. New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_branch.rs`,
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_branch_support.rs`,
and
`crates/hadwiger-research/examples/diagnose_g27_same_field_mwis_lp_guided_branch.rs`.
The diagnostic composes H33, reconstructs the H31/H34a frontier, and for the
top node compares baseline branch vertex `87` against LP-guided branch vertex
`383`, selected by maximizing `weight * x * (1-x)` from the node-local
clique+odd-cycle LP solution with deterministic tie breaks. Command:
`cargo run --release -p hadwiger-research --example diagnose_g27_same_field_mwis_lp_guided_branch`
prints:
`checked_nodes 1 useful_nodes 1 worse_nodes 0 top_relative_gain 2131 top_absolute_drop 5433 max_regression 0 elapsed_millis 1419354 status LpGuidanceUseful theorem_authority false`
and row:
`row 0 parent_total 518612 baseline_branch 87 lp_branch 383 lp_value_ppm 612331 lp_score 4247232738 baseline_worst_child_total 515310 lp_worst_child_total 513179 relative_gain 2131 absolute_drop 5433`.
Verification also passed:
`cargo check -p hadwiger-research --example diagnose_g27_same_field_mwis_lp_guided_branch`
and `cargo fmt -p hadwiger-research --check`; both new Rust modules are below
the 400-line cap. Interpretation: this passes the predeclared top-node gate
(`relative_gain >= 750` and `absolute_drop >= 1000`) despite runtime limiting
the diagnostic to one node. The LP-guided split gets the top frontier node's
worst child to `513179`, only `246` above the `512933` target, while the
baseline worst child remains `515310`. Next funded hypothesis should rerun a
very small H36 continuation from the top node using LP-guided branching, with
proof-carrying row metadata and a hard target of either a new exact-rational
threshold prune or a local worst-child total below target.

Same-field LP-guided micro-continuation succeeds as H36 and creates two new
prune-certificate candidates. Anscombe approved only a kill-switch
micro-diagnostic: reconstruct the H34a top node, apply the fixed H35
LP-guided branch `383`, then LP-guide one more split on the worse child, with
the micro-phase clock capped after the prefix gate. New files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_micro.rs`
and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_micro.rs`.
Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_micro`
prints:
`parent_total 518612 first_worst_total 513179 second_branch 223 final_worst_total 507877 additional_drop 5302 solver_prune_candidates 2 elapsed_millis 314823 status SolverPruneCandidateNeedsExactReplay theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_micro`
and `cargo fmt -p hadwiger-research --check`; touched Rust files remain below
the 400-line cap. Interpretation: the LP-guided lane now has a concrete
proof-carrying target. The first LP split moved the H34a top node from
`518612` to H35 worst child `513179`; the second LP-guided split at branch
`223` drops the local worst descendant to `507877`, below the global target
`512933`, and both second-split children have solver bounds at or below the
dominant threshold `451278`. This is not theorem authority yet because those
two new prune candidates need H32c-style one-sided rational dual replay and
the rest of the H34a frontier remains open. Next funded hypothesis should
promote exactly these two H36 descendants into deterministic row replay and
exact one-sided rational certificates.

Same-field LP-guided micro-dual replay succeeds as H37 scoped proof plumbing.
Confucius approved this only as a hardening pass for the two H36 solver-prune
candidates, not as a theorem-progress claim over the whole frontier. New
files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_micro_dual.rs`,
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_micro_dual_support.rs`,
and
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_lp_guided_micro_duals.rs`.
The replay composes H33, reconstructs the H34a top node, requires first
LP-guided branch `383`, requires the worse-child LP-guided branch `223`,
requires the H36 final worst total `507877`, validates clique and odd-cycle
row semantics for exactly the two second-split children, builds explicit
singleton/edge/clique/odd-cycle dual rows, applies the H32c one-sided upward
rationalization policy with denominator cap `1000000`, and runs the whole
report twice to require a stable row digest. Command:
`cargo run --release -p hadwiger-research --example replay_g27_same_field_mwis_lp_guided_micro_duals`
prints:
`checked_nodes 2 certified_prunes 2 explicit_rows 6915 positive_dual_rows 929 final_worst_total 507877 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 row_digest 365df93fd126a50e6e0b38eed2e42a1efc8463175b838a9b5383d41e01af6d63 status ExactMicroPrunesCertified theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example replay_g27_same_field_mwis_lp_guided_micro_duals`
and `cargo fmt -p hadwiger-research --check`; touched Rust files remain under
the 400-line cap. Interpretation: H36's two LP-guided descendant prunes are
now exact node certificates, not floating numerology. The scoped local
frontier under the H34a top node is below target with exact rational replay,
but the broader H34a tied frontier remains open, so theorem authority remains
false. Next funded hypothesis should test whether the same LP-guided branch
rule plus exact-dual promotion can collapse/certify the rest of the top tied
frontier band, starting with the second H34a node at `518543`.

Same-field LP-guided second-frontier preflight succeeds as H38 and funds a
top-band framework. Ohm approved testing the H34a frontier index `1` before
generalizing, because H37 could otherwise be a one-node accident. New
files/example:
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_second.rs`
and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_second.rs`.
The checker composes H33, reconstructs the H34a frontier, selects deterministic
frontier index `1`, requires parent total `518543` and depth `4`, records a
semantic parent digest from branch-included/forced-included/excluded/candidates
state, performs two LP-guided splits, and exact-replays the two second-split
children only if both are dominant-threshold prune candidates. Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_second`
prints:
`parent_total 518543 first_branch 223 first_worst_total 514729 second_branch 384 final_worst_total 509857 solver_prune_candidates 2 certified_prunes 2 explicit_rows 7804 positive_dual_rows 986 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest b7170cc1f84843d87ff603f41f73b49da277be5dd4efacabb73ace7292e5c6a7 row_digest 32208444486573d6698ed148dc2b0ffd67b3bb57f897ae9bb80bbf7e262c4fa5 status FundTopKFramework theorem_authority false`.
Verification also passed:
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_second`
and `cargo fmt -p hadwiger-research --check`; touched Rust files remain below
the 400-line cap. Interpretation: the LP-guided exact-prune mechanism now
works on the first two H34a frontier nodes. The second node collapses from
`518543` to an exact-certified local worst total `509857`, below target, with
two exact rational prunes. This strongly funds a top-band LP-guided exact
framework over the remaining tied frontier nodes, while still not proving the
global theorem because the rest of the frontier remains open.

H39 strengthened the top-band framework into a three-leaf local partition
certificate and passed for the next two H34a frontier parents, indices `2` and
`3`. Dirac's pre-run critique found a semantic trap in the original two-child
plan: closing only the worse child's grandchildren would leave the untouched
first-split sibling on a floating bound. Implemented
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_lp_guided_top_prefix.rs`
and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_top_prefix.rs`;
also generalized the shared micro-dual helper so exact replay accepts a bounded
leaf slice rather than only a two-child array, and moved generic child-entry /
node-digest helpers into
`g27_same_field_mwis_lp_guided_branch_support.rs`. The H39 checker composes the
H33 prefix, requires the H34a top totals/depths, requires parent identities
`index 2: total 518471 depth 3` and `index 3: total 518441 depth 3`, performs
two LP-guided splits per parent, and exact-replays all three terminal leaves
per parent. Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_top_prefix`
prints:
`checked_nodes 2 certified_nodes 2 certified_leaves 6 remaining_best_open_total 518343 status TopBandPrefixExactProgress theorem_authority false`;
node `2`:
`parent_total 518471 parent_depth 3 first_branch 384 first_child_totals [509911, 514287] second_branch 223 terminal_totals [509911, 507987, 509987] certified_leaves 3 explicit_rows 10845 positive_dual_rows 1413 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest ff8554b582b3df577de0e4f1a8bbf31f6689f84e7e5a2527af74a49f3f4a884e row_digest b968f1dff685ac9ea0317dd4d2ca892986d9fafd49389528cb7d5ee7cf3f9eec status TopBandPrefixExactProgress`;
node `3`:
`parent_total 518441 parent_depth 3 first_branch 384 first_child_totals [509692, 514115] second_branch 223 terminal_totals [509692, 506730, 509602] certified_leaves 3 explicit_rows 10945 positive_dual_rows 1409 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest 5decf014facce0c904c9874452e20e7152ef4b37452d4504b3284d781f30c90b row_digest 01118ff70f37a3323deed1c1558991e8e5da287b3399348e41d72bb3298f7ac1 status TopBandPrefixExactProgress`.
Verification passed:
`cargo fmt -p hadwiger-research --check`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_top_prefix`,
and the release example. Interpretation: H34a frontier prefix indices `0..3`
now have scoped local exact certificates below target, and the carried
unresolved frontier best shifts to index `4` at `518343`. This is real
proof-carrying progress, but theorem authority remains false because the
remaining H34a frontier is still open and the current best unresolved bound is
above `512933`.

H40 extended the same three-leaf top-prefix exact preflight to H34a frontier
indices `4` and `5`, after Aquinas approved only that crisp two-node scope and
rejected both index-4-only and opportunistic until-failure sweeps. The existing
top-prefix module was generalized into a profile runner, and H40 added
`preflight_g27_same_field_mwis_lp_guided_next_prefix_checked` plus example
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_next_prefix.rs`.
The checker now computes the carried unresolved best from the frontier entry
after the certified prefix and gates it, instead of printing a constant.
Command:
`cargo run --release -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_next_prefix`
prints:
`checked_nodes 2 certified_nodes 2 certified_leaves 6 remaining_best_open_total 518105 status TopBandPrefixExactProgress theorem_authority false`;
node `4`:
`parent_total 518343 parent_depth 5 first_branch 383 first_child_totals [515167, 510402] second_branch 87 terminal_totals [510402, 510922, 512089] certified_leaves 3 explicit_rows 11450 positive_dual_rows 1457 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest b7d0b9f15b994797e087aa76e38739ae6ec04806369051896dd967b823c4d88b row_digest 6099aa6337b45f0987d50d353ae7852e6424cf10973df07738a0a7b51a8a862b status TopBandPrefixExactProgress`;
node `5`:
`parent_total 518123 parent_depth 6 first_branch 383 first_child_totals [511951, 512640] second_branch 223 terminal_totals [511951, 501987, 508398] certified_leaves 3 explicit_rows 9180 positive_dual_rows 1264 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest 060ec36608939a8458fe2e5cb11cc562457c623337ba4e300a6a2bbdb652cb2b row_digest 434ebb2a78631df9757c4c6851673f908a406406ba7a996ecc4ce573a6843016 status TopBandPrefixExactProgress`.
Verification passed:
`cargo fmt -p hadwiger-research --check`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_top_prefix`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_next_prefix`,
and the release example. Interpretation: H34a frontier prefix indices `0..5`
now have scoped local exact certificates below target, including the first
depth-6 parent tested in this mechanism. The carried unresolved frontier best
is index `6` at `518105`, still above target, so theorem authority remains
false.

H41 extended the same three-leaf top-prefix exact preflight to H34a frontier
indices `6` and `7`, after Parfit approved exactly that two-node scope and
rejected both index-6-only and remaining-suffix sweeps. Added
`preflight_g27_same_field_mwis_lp_guided_third_prefix_checked` and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_third_prefix.rs`;
also added a contiguous-index guard to the shared profile runner so carried
best claims cannot silently skip unresolved frontier parents. The first release
attempt failed during rustc/LLVM compilation with out-of-memory before the
example ran; rerunning with `RUSTC_WRAPPER=''` and `CARGO_BUILD_JOBS=1`
succeeded. Command:
`$env:RUSTC_WRAPPER=''; $env:CARGO_BUILD_JOBS='1'; cargo run -p hadwiger-research --release --example preflight_g27_same_field_mwis_lp_guided_third_prefix`
prints:
`checked_nodes 2 certified_nodes 2 certified_leaves 6 remaining_best_open_total 517741 status TopBandPrefixExactProgress theorem_authority false`;
node `6`:
`parent_total 518105 parent_depth 6 first_branch 383 first_child_totals [511910, 512491] second_branch 223 terminal_totals [511910, 502972, 507197] certified_leaves 3 explicit_rows 10465 positive_dual_rows 1388 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest 726df3ae15fac725dfeeef889da562d92a4d0d0ad2a74c8b097dbad9ac880860 row_digest 5561a45b0cf0a5b6b530b4fbf5b2dec4f36bea49ff5b4c20c68d86b655e96161 status TopBandPrefixExactProgress`;
node `7`:
`parent_total 517777 parent_depth 6 first_branch 383 first_child_totals [510729, 511854] second_branch 223 terminal_totals [510729, 502480, 507015] certified_leaves 3 explicit_rows 11028 positive_dual_rows 1444 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest 5af208a1a0bfc1bc49b4480df8f257ca0cbc4bab09a90c69b2f650ca58b21132 row_digest 7383390433cce48b47579d551469af2b8b247486449aa0a1e8617c7c42c6e629 status TopBandPrefixExactProgress`.
Verification passed:
`cargo fmt -p hadwiger-research --check`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_third_prefix`,
and the release example. Interpretation: H34a frontier prefix indices `0..7`
now have scoped local exact certificates below target. The carried unresolved
frontier best is index `8` at `517741`, still above target, so theorem
authority remains false.

H42 passed as the final predeclared H34a top-10 pair certificate. Godel
approved running exactly indices `8` and `9`, but warned not to call the H42
checker itself a standalone top-10 proof unless it re-composed all predecessor
certificates; therefore this is recorded as final-pair exact progress which,
combined with H37-H41, closes H34a frontier indices `0..9`. Added
`preflight_g27_same_field_mwis_lp_guided_final_top_pair_checked` and
`crates/hadwiger-research/examples/preflight_g27_same_field_mwis_lp_guided_final_top_pair.rs`.
The H42 profile computes the actual carried `frontier[10]` total instead of
using a guessed constant. Command:
`$env:RUSTC_WRAPPER=''; $env:CARGO_BUILD_JOBS='1'; cargo run -p hadwiger-research --release --example preflight_g27_same_field_mwis_lp_guided_final_top_pair`
prints:
`checked_nodes 2 certified_nodes 2 certified_leaves 6 remaining_best_open_total 517394 status TopBandPrefixExactProgress theorem_authority false`;
node `8`:
`parent_total 517741 parent_depth 6 first_branch 383 first_child_totals [510908, 512223] second_branch 223 terminal_totals [510908, 501835, 508118] certified_leaves 3 explicit_rows 9746 positive_dual_rows 1321 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest afaf0d5a5cf9f8a3f0b4658dc609bd290f199b33cc7ea03d822e62f487690eae row_digest 08315a5797cea1f9ae06a71012090e1fd57caa42ead039c388615b9cc4eb1ae1 status TopBandPrefixExactProgress`;
node `9`:
`parent_total 517649 parent_depth 6 first_branch 223 first_child_totals [507682, 513301] second_branch 383 terminal_totals [507682, 507419, 507125] certified_leaves 3 explicit_rows 11083 positive_dual_rows 1441 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 parent_digest 6f3693db9a82411d2f596d0fb0ecda47f5d332280516f093fe6e2d1ff2face75 row_digest 17cd07be167a71b4ebfadb6db9baf948900cff134718e9a97775cfaf135e4a52 status TopBandPrefixExactProgress`.
Verification passed:
`cargo fmt -p hadwiger-research --check`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_top_prefix`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_next_prefix`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_third_prefix`,
`cargo check -p hadwiger-research --example preflight_g27_same_field_mwis_lp_guided_final_top_pair`,
and the release example. Interpretation: combined H37-H42 give scoped local
exact certificates below target for the original H34a top-10 frontier band,
indices `0..9`. The actual next unresolved frontier node is index `10` at
`517394`, still above `512933`; theorem authority remains false. Next
hypothesis should inspect the remaining 18-node frontier shape before
continuing the two-node cadence or changing strategy.

Full remaining frontier diagnostic added after H42 to avoid extending the old
top-10 cadence blindly. Implemented
`diagnose_g27_same_field_mwis_full_frontier_shape_checked` and
`crates/hadwiger-research/examples/diagnose_g27_same_field_mwis_full_frontier_shape.rs`.
Command:
`$env:RUSTC_WRAPPER=''; $env:CARGO_BUILD_JOBS='1'; cargo run -p hadwiger-research --release --example diagnose_g27_same_field_mwis_full_frontier_shape`
prints:
`open_frontier_nodes 28 tied_band_nodes 10 gap_to_second 69 best_open_total_bound 518612 frontier_totals [518612, 518543, 518471, 518441, 518343, 518123, 518105, 517777, 517741, 517649, 517394, 517365, 517311, 517204, 517107, 516592, 516221, 516140, 516061, 515994, 515851, 515694, 515464, 514844, 514775, 514411, 513989, 513289] frontier_depths [5, 4, 3, 3, 5, 6, 6, 6, 6, 6, 6, 6, 3, 4, 6, 6, 6, 5, 5, 7, 5, 5, 6, 5, 5, 6, 7, 5] status TiedFrontierBandRequiresTopKContinuation theorem_authority false`.
Verification passed:
`cargo fmt -p hadwiger-research --check` and
`cargo check -p hadwiger-research --example diagnose_g27_same_field_mwis_full_frontier_shape`.
Interpretation: after the top-10 exact prefix, the remaining 18 H34a frontier
nodes are still all above target, from index `10` at `517394` down to index
`27` at `513289`. The next hypothesis should be chosen from this full
remaining-frontier shape, not just by continuing the old top-10 story.

H44 replaced hand-selected pair continuation with a digest-bound campaign scout
artifact over the unresolved H34a suffix `10..27`, after Euclid approved the
campaign-scout hypothesis and explicitly warned not to claim Forge Query
parallel admission unless we actually use that route. Implemented
`scout_g27_same_field_mwis_frontier_closure_campaign_checked`,
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_frontier_closure_campaign.rs`,
support module
`crates/hadwiger-research/src/frontier_seeds/g27_same_field_mwis_frontier_closure_campaign_support.rs`,
and example
`crates/hadwiger-research/examples/scout_g27_same_field_mwis_frontier_closure_campaign.rs`.
The report is a canonical Hadwiger artifact of kind
`g27_mwis_frontier_closure_campaign_scout_report`; it replays the H33 prefix,
freezes all 28 frontier totals/depths/digests, marks indices `0..9` as already
closed by H37-H42, and scouts only suffix nodes `10..27` with the deterministic
two-split LP-guided three-leaf pattern. It performs no exact rational replay.
After initial compiler/resource noise and a traced run, the clean direct
release executable produced:
`artifact_digest 6bd5dc2d60e45cb2f05bf0dd4beb7fcabf2ffae6684ddf8f383a2563601a6865 frontier_nodes 28 scout_rows 18 ready_count 18 failing_count 0 worst_terminal_total 512146 continuation_max_total 0 status campaign_scout_ready theorem_authority false`.
Every suffix row `10..27` classified as
`ready_for_exact_three_leaf_replay`; no suffix node requires deeper continuation
under the current LP-guided scout. The worst terminal total is index `17` leaf
`512146`, still below target `512933`. Verification passed:
`cargo fmt -p hadwiger-research --check`,
`cargo check -p hadwiger-research --example scout_g27_same_field_mwis_frontier_closure_campaign`,
and the release executable
`target/release/examples/scout_g27_same_field_mwis_frontier_closure_campaign.exe`.
Interpretation: H44 is a genuine campaign-level planning artifact. Combined
with H37-H42, it says all remaining H34a frontier parents appear immediately
eligible for the same three-leaf exact promotion; theorem authority remains
false until exact rational replay certifies the suffix leaves and a composition
checker verifies coverage.

H45 implemented the resumable exact replay campaign and certified the first
suffix chunk, indices `10..13`, against the H44 scout digest. Archimedes
approved the shape: recompute H44 from source, require scout artifact digest
`6bd5dc2d60e45cb2f05bf0dd4beb7fcabf2ffae6684ddf8f383a2563601a6865`,
then exact-replay an explicit contiguous chunk without trusting unbound scout
fields such as `worse_child`. Added
`replay_g27_same_field_mwis_frontier_closure_exact_chunk_checked`, support /
gate / payload modules, artifact kind
`g27_mwis_frontier_closure_exact_replay_report`, and example
`crates/hadwiger-research/examples/replay_g27_same_field_mwis_frontier_closure_exact_chunk.rs`.
The first release chunk run sealed:
`artifact_digest ad9d452665ee980c7c1bb7292dee281487a4307e44db5a249163545ded061166 source_scout_digest 6bd5dc2d60e45cb2f05bf0dd4beb7fcabf2ffae6684ddf8f383a2563601a6865 selected_range 10..14 checked_nodes 4 certified_nodes 4 certified_leaves 12 explicit_rows 41308 positive_dual_rows 5464 worst_terminal_total 510968 max_denominator 1000000 min_slack_floor 0 max_objective_excess 0 unresolved_suffix 14..28 status exact_chunk_certified theorem_authority false`.
Node row digests: index `10`
`468dc88348e75167a7f045fb705ec3d19684eb0ff9532123a22097a8486235bf`,
index `11`
`22d5ac4c49adfd473612b99788a9d38a8bbe21ae7bdea0483a2adda2cad989d2`,
index `12`
`356a0bbb0175c38fd81b40f4a97569a9a3039cf39c413d7c1022d0a6352f69ca`,
and index `13`
`854d39fa65d0f5d1790c1dfb165daf3f57dd0993c18c285150d903645177f03d`.
Interpretation: H34a frontier indices `10..13` have moved from scout-ready
to exact/rational suffix closure evidence. Combined H37-H42 plus H45 now
exact-cover frontier indices `0..13`, leaving suffix `14..27` unresolved.
Theorem authority remains false until all suffix chunks are exact-certified
and a final coverage-composition checker composes the prefix and suffix
artifacts.

Budgets: P1 days; P2 days; P3 bounded sweeps with explicit counters per
family, suppression on exhaustion.

## Secondary Track (P0 - closing out)

The Heule-510 edge-criticality closeout runs unattended in the background:
complete the contraction-lane edge map, derive a greedy edge-critical core
with certified UNSAT chain, retain it as a frontier seed. Value: end-to-end
certification that the pipeline produces proof-carrying frontier evidence
(already yielded: 510/510 vertex-criticality, the witness-transfer and
contraction lanes, certified removable edge 1-34). Its endpoint is substrate
validation, not a mathematical headline, and it is priced accordingly - no
further interactive effort beyond harvesting results.

## Suppressed Directions (do not re-fund without new evidence)

- **Pressure-halo hub hypothesis (sign falsified).** The static pressure
  score `degree*100 + triangles` anti-predicts chromatic rigidity: the hub
  is the slackest region (its edges are removable); rigidity concentrates
  in the low-degree rim. Reactivation: only as an explicitly
  inverted/recalibrated signal validated against the criticality atlas.
- **R2 k=5 forcing-gadget ladder toward `chi >= 6`.** Correct attack
  shape, wrong odds at reachable sizes: a fifth color's slack means
  ~510-vertex cores almost certainly force nothing, and the post-2018
  specialist record (stronger solvers, years of effort, no such gadget)
  prices the expected yield near zero. Reactivation: a certified k=5
  forced pair appearing incidentally (e.g., from P3 gluing experiments),
  or new literature demonstrating forcing at small sizes.
- **Integral vertex/edge record chasing.** Beating 509 vertices or 2442
  edges is saturated specialist territory; the P0 closeout artifacts are
  kept as feedstock, not pursued as records.
- **Moser-lattice improvement search.** The 2023 `chi_gf = 4` witness is a
  reproduction target, but not an improvement region. The 2026 Moser-lattice
  coloring result suppresses search confined to the Moser lattice or Moser
  ring unless new evidence shows the candidate escapes the retained coloring
  theorem's scope.

## Standing Assets

- Exact Heule-510 seed (510v/2504e, exact algebraic embedding, retained
  UNSAT certificate) + criticality/rigidity atlas.
- Contraction reformulation: forced-relation queries (`same color in every
  k-coloring?` / `different in every k-coloring?`) as single fast SAT
  calls (~7000x over naive encodings on rigid instances).
- Witness transfer: retained deletion colorings certify mutation outcomes
  with zero solving.
- LP screening lanes (`fractional_chromatic_screening.rs`,
  `lovasz_theta_screening.rs`) wired to `good_lp`/`clarabel`.
