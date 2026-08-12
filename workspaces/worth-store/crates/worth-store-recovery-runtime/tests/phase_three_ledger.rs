use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const GUARANTEES: [&str; 12] = [
    "C8-P3-DISCOVERY-01",
    "C8-P3-ROOT-01",
    "C8-P3-PAGE-FACTS-01",
    "C8-P3-CHECKPOINT-01",
    "C8-P3-WAL-01",
    "C8-P3-COMPACTION-01",
    "C8-P3-RESIDUE-01",
    "C8-P3-LIMITS-01",
    "C8-P3-PROGRESSION-01",
    "C8-P3-API-01",
    "C8-P3-EFFECT-01",
    "C8-P3-LEDGER-01",
];

const SOURCE_CLOSURE_SHA256: &str =
    "d96a59ce0a72f501d177dbd109e7f0ac50d8c6173118cda119caa314cea392b2";

const GUARANTEE_CONTRACTS: [&str; 12] = [
    "C8-P3-DISCOVERY-01|3|Discovery reads the two fixed selector cells conditional torn-current bootstrap anchor checkpoint slot exact manifest addresses and one bounded WAL directory while carrying remaining budgets into every read and decoder|backend recovery-media discovery and runtime discovery orchestration|nonempty and multi-block Store journeys exact counters cumulative WAL manifest preallocation twins typed media failures and exact fallback-anchor worlds|PROVED|C8-P3-F02 C8-P3-F07 C8-P3-F11 C8-P3-F13 C8-P3-F14 C8-P3-F15 C8-P3-F17|page and extent payload reads belong to Phase 4 planning",
    "C8-P3-ROOT-01|3|Current and previous roots are admitted by fixed role persisted Store format identity and exact selector linkage while torn-current fallback additionally requires the previous successor links to match the independently decoded bootstrap publication anchor|recovery physics current-previous source precedence|hostile absent missing torn stale-anchor mismatched previous-only publication-prefix and real foreign-Store persisted-selector attacks retain exact typed observation and selection denial evidence together|PROVED|C8-P3-F01 C8-P3-F04 C8-P3-F08 C8-P3-F09 C8-P3-F10 C8-P3-F12 C8-P3-F15 C8-P3-F17|none",
    "C8-P3-PAGE-FACTS-01|3|The selected basis carries every manifest-addressed record placement from a checksum tree and format bound routing-block closure whose remaining leaf and branch cardinalities are checked before vector collection|recovery physics page-fact admission|real nonempty and branched-tree journeys plus reference-bound denials and a malformed crossing-entry mutant proving cardinality wins before entry decode|PROVED|C8-P3-F02 C8-P3-F07 C8-P3-F12 C8-P3-F13 C8-P3-F17|page bytes page headers and pageLSNs are admitted in Phase 4",
    "C8-P3-CHECKPOINT-01|3|A checkpoint participates only after the complete persisted stream footer dirty records binding records and root Store WAL identities verify within bounds and rejected input remains distinct from absence with its exact decoder or binding denial|physical-format checkpoint inspection and recovery physics checkpoint admission|whole-stream round trip truncation integrity count root-bound integration and typed absent-versus-corrupt terminal-evidence attacks|PROVED|C8-P3-F03 C8-P3-F08 C8-P3-F10 C8-P3-F17|persisted checkpoint security binding is delivered in Phase 4",
    "C8-P3-WAL-01|3|WAL selection sorts semantic segment identity and admits one checkpoint-contiguous prefix while every segment scanner receives the cumulative remaining frame budget before retaining a crossing frame|recovery physics WAL-tail admission|reversed enumeration continuity corruption torn-tail scanned-versus-valid and exact one-over versus exact-at-limit bounded scanner tests|PROVED|C8-P3-F06 C8-P3-F08 C8-P3-F10 C8-P3-F12 C8-P3-F13 C8-P3-F17 C8-P3-F18|persisted WAL security binding is delivered in Phase 4",
    "C8-P3-COMPACTION-01|3|The operation-binding compaction product is visible only through a cutover decoded after whole-checkpoint verification and bound to that checkpoint root WAL range role generation and cutoff|physical-format checkpoint cutover and recovery physics compaction admission|verified stream cutover round trip schema-locked role and checkpoint-root type construction|PROVED|C8-P3-F03 C8-P3-F17|operation-fate reconstruction from the admitted product belongs to Phase 4",
    "C8-P3-RESIDUE-01|3|Noncanonical nonregular trailing-empty interrupted-terminal-first-frame and unreferenced recovery artifacts remain typed residue with exact observed bytes and can never supply a missing authoritative source|recovery physics residue classification|plausible newest WAL terminal interruption repeated-selection and compaction residue mutants|PROVED|C8-P3-F06 C8-P3-F08 C8-P3-F17 C8-P3-F18|cleanup eligibility remains post-publication Phase 7 work",
    "C8-P3-LIMITS-01|3|Selector manifest checkpoint cumulative WAL byte frame entry distinct-fact and observation limits are carried into media and format owners and reject before a crossing read allocation vector collection or retained frame|runtime discovery backend bounded media physical-format and WAL admission|one-over and exact-at-limit cumulative WAL and multi-block manifest worlds plus malformed-entry and bounded two-frame decoder mutants|PROVED|C8-P3-F02 C8-P3-F07 C8-P3-F08 C8-P3-F12 C8-P3-F13 C8-P3-F17|later planning staging publication and cleanup dimensions remain owned by their phases",
    "C8-P3-PROGRESSION-01|3|Only consuming admitted to discovered to selected Result transitions reach one source cut while every persisted-source or media denial terminates top-level Blocked with typed address cause and multi-cause evidence after quiescence|recovery runtime typed progression and blocked handoff|warnings-denied journeys exact root manifest anchor and real selector or checkpoint media denials blocked session counter and compiler attacks|PROVED|C8-P3-F05 C8-P3-F06 C8-P3-F08 C8-P3-F09 C8-P3-F10 C8-P3-F11 C8-P3-F12 C8-P3-F14 C8-P3-F15 C8-P3-F17 C8-P3-F18|planning is not yet callable",
    "C8-P3-API-01|3|Phase 3 exposes exactly the production-reachable bounded format and WAL decoders typed backend artifact address deterministic physics progression media and source denials and Blocked outcome surfaces assigned by the destination inventory|runtime backend format WAL and physics facades|path-resolved cross-crate facade derivation revision-derived pre-C8 baseline exact inventory equality cross-owner associated-item mutants topology typed denial bounded-decoder and Store-identity placement gates|PROVED|C8-P3-F05 C8-P3-F10 C8-P3-F11 C8-P3-F12 C8-P3-F13 C8-P3-F14 C8-P3-F15 C8-P3-F16 C8-P3-F17 C8-P3-F18|later phase facades remain unavailable",
    "C8-P3-EFFECT-01|3|Discovery selection cancellation Refused and every persisted-source or exact media-address Blocked terminal perform zero recovery data namespace publication cleanup or reopen effects while retaining every typed cause and counter|backend owned effect observation and runtime terminal handoff|success residue limit malformed-source stale-anchor selector-media checkpoint-media corruption and cancellation paths read the complete owner counter and exact denial|PROVED|C8-P3-F05 C8-P3-F07 C8-P3-F08 C8-P3-F10 C8-P3-F12 C8-P3-F14 C8-P3-F15 C8-P3-F17|exclusive lease lifecycle is not a recovery data effect",
    "C8-P3-LEDGER-01|3|Every Phase 3 guarantee has exact row semantics causal source membership finding history and independent audit retention including preallocation decoders media evidence publication anchors and causal tests|Phase 3 ledger gate|canonical row source-set bounded-decoder typed-media anchor duplicate omission stale identity foreign causality and audit mutants|PROVED|C8-P3-F01 C8-P3-F02 C8-P3-F03 C8-P3-F04 C8-P3-F05 C8-P3-F06 C8-P3-F07 C8-P3-F08 C8-P3-F09 C8-P3-F10 C8-P3-F11 C8-P3-F12 C8-P3-F13 C8-P3-F14 C8-P3-F15 C8-P3-F16 C8-P3-F17 C8-P3-F18|none",
];

const FINDINGS: [&str; 18] = [
    "C8-P3-F01|High|C8-P3-ROOT-01 C8-P3-LEDGER-01|Initial precedence treated the legal previous-only ordered publication prefix as a broken current protocol|Kept the still-valid old current selector authoritative and retained the nonreciprocal new previous selector only as rejected evidence|exact previous-only prefix mutant selects generation one and never promotes the partial successor",
    "C8-P3-F02|High|C8-P3-DISCOVERY-01 C8-P3-PAGE-FACTS-01 C8-P3-LIMITS-01 C8-P3-LEDGER-01|Initial discovery stopped at the root header used a synthetic flat artifact path and omitted nonempty manifest-addressed page or extent facts|Restored the real record-family directories and added bounded checksum-bound routing-tree discovery plus selected fact admission|real nonempty extent journey and missing duplicate over-budget record-count and distinct-fact attacks pass",
    "C8-P3-F03|High|C8-P3-CHECKPOINT-01 C8-P3-COMPACTION-01 C8-P3-LEDGER-01|Compaction cutover visibility initially relied on a live or derived posture rather than a crash-surviving checkpoint record|Added the persisted cutover record to the checkpoint stream and expose it only from whole-stream verification|physical-format round trip and runtime checkpoint cutover journey reject truncated or unbound input",
    "C8-P3-F04|High|C8-P3-ROOT-01 C8-P3-LEDGER-01|Missing and undecodable current selector slots shared one fallback branch so an absent current slot could promote a merely linked previous selector|Restricted previous fallback to an observed undecodable current selector and retained current rejection in the deterministic decision trace|absent-current attack blocks while the torn-current linked-previous case remains selected",
    "C8-P3-F05|High|C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|Persisted-source blockers were collapsed into Refused and the session had no matching Blocked terminal receipt|Added concrete top-level Blocked outcome evidence and a quiescent blocked handoff that consumes the session separately from entry and cancellation refusal|corrupt checkpoint and foreign persisted Store attacks return Blocked retain exact evidence increment the blocked terminal counter and report zero effects",
    "C8-P3-F06|High|C8-P3-WAL-01 C8-P3-RESIDUE-01 C8-P3-PROGRESSION-01 C8-P3-LEDGER-01|A torn first frame in the terminal newest WAL segment rejected the complete valid prefix in prior segments|Classified a strict terminal partial-first-frame as typed interrupted-start residue while preserving prior verified segments and kept nonterminal interruption or complete corruption blocking|physics twins and the real checkpoint plus two-segment journey preserve one complete prior frame and count the 37-byte torn suffix",
    "C8-P3-F07|High|C8-P3-DISCOVERY-01 C8-P3-PAGE-FACTS-01 C8-P3-LIMITS-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|WAL observation and manifest entry or distinct-fact limits were applied per artifact or after aggregate allocation|Carried remaining WAL observation manifest-byte and manifest-entry budgets before every read or decoded-entry extension and admitted facts incrementally|two eight-byte WAL files reject at observed sixteen over admitted twelve and a three-entry branched manifest rejects at three over two while exact-limit twins pass",
    "C8-P3-F08|High|C8-P3-ROOT-01 C8-P3-CHECKPOINT-01 C8-P3-WAL-01 C8-P3-RESIDUE-01 C8-P3-LIMITS-01 C8-P3-PROGRESSION-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|Aggregate success-only counters discarded absent versus rejected inputs torn bytes residue kinds denial context and all evidence on terminal failure|Split counters by role and posture and retain counters exact limit artifact generation and LSN context in PhysicalRecoveryBlock|absent and corrupt checkpoint twins terminal WAL interruption and cumulative limit attacks observe distinct exact evidence after session consumption",
    "C8-P3-F09|High|C8-P3-ROOT-01 C8-P3-PROGRESSION-01 C8-P3-LEDGER-01|The inherited foreign-Store proof used a cfg-test admitted-world mutation hook instead of the persisted production boundary|Removed the production test hook and substituted a selector and manifest from a separately initialized real Store through ordinary persisted discovery|the primary Store admits its own recovery world rejects the alternate persisted Store selector counts one rejected current root and blocks without effects",
    "C8-P3-F10|High|C8-P3-ROOT-01 C8-P3-CHECKPOINT-01 C8-P3-WAL-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|The first failure-evidence correction still collapsed typed root checkpoint and WAL denials into generic artifact text and counted only admitted WAL candidates as scanned segments|Preserved exact source-family denial values and observed identities through a multi-cause Blocked evidence vector and separated canonical WAL segments scanned from valid segments admitted|wrong-role foreign-Store missing-manifest selector-format checkpoint truncation integrity record-count WAL-corruption and segment-gap twins assert distinct typed causes and exact scanned-versus-valid counters",
    "C8-P3-F11|Medium|C8-P3-DISCOVERY-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-LEDGER-01|Source observation and selection were collapsed into long mixed-level orchestration functions that classified every family constructed diagnostics and transferred progression inline|Extracted named root manifest checkpoint WAL and final-cut selection steps plus named source-family observation steps under their exact semantic owners|focused warnings-denied journeys pass and dirty composition scrutiny finds no collapsed Phase 3 discovery or selection responsibility",
    "C8-P3-F12|High|C8-P3-ROOT-01 C8-P3-PAGE-FACTS-01 C8-P3-WAL-01 C8-P3-LIMITS-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|The typed-evidence correction still chose a root-slot cause instead of retaining the resulting selection cause collapsed every manifest-routing observation failure and counted only valid WAL frames as scanned|Always append the root-selection denial preserve reference-bound manifest observation reasons and carry recognizable scanned frame evidence separately from valid frames|torn-current absent and unlinked-previous twins five manifest missing decode format tree checksum twins and digest-corrupt WAL assert every exact cause and one scanned versus zero valid frame",
    "C8-P3-F13|High|C8-P3-DISCOVERY-01 C8-P3-PAGE-FACTS-01 C8-P3-WAL-01 C8-P3-LIMITS-01 C8-P3-API-01 C8-P3-LEDGER-01|Manifest and WAL cardinality limits were compared only after complete routing-entry and frame vectors had already been decoded and allocated|Added remaining-budget contracts to the physical-format routing decoder and WAL segment scanner and reject before collecting or retaining the crossing item|malformed second manifest entry returns the one-entry limit before placement denial and a two-frame WAL returns the one-frame limit while both exact-limit twins pass",
    "C8-P3-F14|High|C8-P3-DISCOVERY-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|Backend observation errors collapsed the exact source address failure kind and operating-system cause into generic MediaObservation text|Carried typed record checkpoint WAL-directory or WAL-member addresses and exact backend plus OS failure kinds into Blocked source evidence|real unreadable current-selector and checkpoint slots produce distinct typed artifacts exact backend causes and zero effects",
    "C8-P3-F15|High|C8-P3-DISCOVERY-01 C8-P3-ROOT-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|An undecodable current selector accepted any previous selector with nonempty successor links without proving it belonged to the completed publication|Conditionally decode the independent bootstrap catalog and require its Store format and current generation to corroborate both previous-selector successor links|exact generation-two anchor selects the generation-one previous root while an otherwise identical stale generation-three anchor blocks with the exact selection denial",
    "C8-P3-F16|High|C8-P3-API-01 C8-P3-LEDGER-01|Exact facade syntax proof skipped item kinds public namespace children module aliases and configuration variants and resolved deep dependency bypasses by leaf name|Enumerate the complete facade against the full immutable pre-C8 revision retain exact module and visibility variants require contractual dependency facades resolve canonical impl targets account for every direct public item kind recurse direct public namespaces and fail closed before projection on every module re-export alias plus every unproved associated-item macro extern-crate foreign-module or verbatim surface|static union extern-crate foreign-module root-macro macro-export public-namespace-macro private-module-control exact-accessor qualified-macro associated-macro nested-public local-module-alias grouped-self-module-alias root-crate-alias dependency-crate-alias private-import-alias alias-file-variant external-module-hostile-twin external-glob self-cycle two-module-cycle repeated-acyclic-alias visibility-order glob outside-alias deep-external canonical-method supported-cfg orphan disabled-module path-remap file-variant inline-variant cfg-attr omission and historical-baseline drift mutants fail independently of destination policy",
    "C8-P3-F17|High|C8-P3-DISCOVERY-01 C8-P3-ROOT-01 C8-P3-PAGE-FACTS-01 C8-P3-CHECKPOINT-01 C8-P3-WAL-01 C8-P3-COMPACTION-01 C8-P3-RESIDUE-01 C8-P3-LIMITS-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-EFFECT-01 C8-P3-LEDGER-01|Phase 6 extended the shared recovery entry progression outcome inventory and publication-anchor causal paths while every inherited Phase 3 source identity still described the pre-publication topology|Rebind every Phase 3 guarantee to the exact current shared sources while preserving discovery and selection as read-only effect-free predecessors of planning publication reopen and construction|warnings-denied Phase 3 discovery denial and ledger suites plus the complete runtime C8 and C7 boundaries pass on the same source graph",
    "C8-P3-F18|High|C8-P3-WAL-01 C8-P3-RESIDUE-01 C8-P3-PROGRESSION-01 C8-P3-API-01 C8-P3-LEDGER-01|Phase 7 cleanup required the selected WAL cut to retain the exact checkpoint-covered artifact identities ranges bytes and cleanup-safe posture but the inherited Phase 3 proof ended at the selected tail and could not bind those later eligibility inputs|Carry checkpoint-covered WAL cleanup facts through the Phase 3 selection product without granting cleanup authority and reconcile the exact delivered facade topology and source closure|warnings-denied Phase 3 selection and Phase 7 cleanup journeys distinguish retained uncovered WAL from checkpoint-covered cleanup candidates while exact facade and ledger gates pass",
];

const AUDITS: [&str; 3] = [
    "/root/c8_phase3_final_critic|gpt-5.6-sol high|Frozen Phase 3 implementation specification artifacts tests and composition|Defects|C8-P3-F05 C8-P3-F06 C8-P3-F07 C8-P3-F08 C8-P3-F09 C8-P3-F10 C8-P3-F11 C8-P3-F12|Root corrections implemented; final frozen re-audit pending",
    "/root/c8_phase3_absolute_closure_critic|gpt-5.6-sol high|Frozen post-F12 Phase 3 source limits media failure topology selector fallback tests and composition|Defects|C8-P3-F13 C8-P3-F14 C8-P3-F15|Root corrections implemented; final stable-snapshot re-audit pending",
    "/root/c8_phase3_final_closure_critic_v2|gpt-5.6-sol high|Frozen post-F15 Phase 3 functional roots facade evidence tests and composition|Defects|C8-P3-F16|Live delivered-facade derivation implemented; final stable-snapshot re-audit pending",
];

#[test]
fn phase_three_ledger_is_exact_closed_and_source_bound() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let source = read(&root.join(source_closure_path()));
    validate_source_contract(&root, &source);
    validate_ledger(&root, &ledger, &parse_source_closures(&root, &source));
}

#[test]
fn causal_history_and_source_contract_mutants_cannot_self_certify() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let source = read(&root.join(source_closure_path()));
    let closures = parse_source_closures(&root, &source);
    let root_row = ledger
        .lines()
        .find(|line| line.starts_with("| C8-P3-ROOT-01"))
        .unwrap();
    for mutant in [
        ledger.replacen(root_row, "", 1),
        ledger.replacen(root_row, &format!("{root_row}\n{root_row}"), 1),
        ledger.replacen("bootstrap publication anchor", "generation maximum", 1),
        ledger.replacen("C8-P3-F01", "C8-P3-F03", 1),
        ledger.replacen("| PROVED |", "| ACTIVE |", 1),
    ] {
        assert!(std::panic::catch_unwind(|| validate_ledger(&root, &mutant, &closures)).is_err());
    }
    let foreign = source.replacen(
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/entry/limits.rs",
        "README.md",
        1,
    );
    assert!(std::panic::catch_unwind(|| validate_source_contract(&root, &foreign)).is_err());
}

fn validate_source_contract(root: &Path, source: &str) {
    assert_eq!(
        sha256(normalize_newlines(source).as_bytes()),
        SOURCE_CLOSURE_SHA256
    );
    let closures = parse_source_closures(root, source);
    assert_eq!(
        closures.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        GUARANTEES.into_iter().collect()
    );
}

fn validate_ledger(root: &Path, ledger: &str, closures: &BTreeMap<String, BTreeSet<String>>) {
    let rows = rows_between(
        ledger,
        "<!-- c8-phase3-ledger:start -->",
        "<!-- c8-phase3-ledger:end -->",
        "| C8-P3-",
    );
    assert_eq!(rows.len(), GUARANTEES.len());
    assert_eq!(
        rows.iter()
            .map(|row| guarantee_contract(row))
            .collect::<BTreeSet<_>>(),
        GUARANTEE_CONTRACTS.into_iter().map(str::to_owned).collect()
    );
    let identities = rows
        .iter()
        .map(|row| {
            let values = cells(row);
            (values[0].to_owned(), values[5].to_owned())
        })
        .collect::<BTreeMap<_, _>>();
    let mismatches = GUARANTEES
        .into_iter()
        .filter_map(|guarantee| {
            let expected = source_identity(root, &closures[guarantee]);
            (identities[guarantee] != expected).then(|| {
                format!(
                    "{guarantee}: actual={} expected={expected}",
                    identities[guarantee]
                )
            })
        })
        .collect::<Vec<_>>();
    assert!(
        mismatches.is_empty(),
        "stale source identities: {mismatches:#?}"
    );
    assert_eq!(
        history_contracts(
            ledger,
            "## Phase 3 finding history",
            "## Independent audit history",
            "| C8-P3-F",
        ),
        FINDINGS.into_iter().map(str::to_owned).collect()
    );
    assert_eq!(
        history_contracts(ledger, "## Independent audit history", "", "| /root/"),
        AUDITS.into_iter().map(str::to_owned).collect()
    );
}

fn parse_source_closures(root: &Path, source: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut closures = BTreeMap::<String, BTreeSet<String>>::new();
    for line in normalize_newlines(source)
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
    {
        let (guarantee, path) = line.split_once(',').expect("two-column source closure");
        assert!(GUARANTEES.contains(&guarantee));
        assert!(root.join(path).is_file(), "missing causal source {path}");
        assert!(closures
            .entry(guarantee.into())
            .or_default()
            .insert(path.into()));
    }
    closures
}

fn source_identity(root: &Path, paths: &BTreeSet<String>) -> String {
    let mut digest = Sha256::new();
    for path in paths {
        digest.update(path.as_bytes());
        digest.update([0]);
        let bytes = if path == ledger_path() {
            canonical_ledger(&read(&root.join(path))).into_bytes()
        } else {
            std::fs::read(root.join(path)).expect("causal source bytes")
        };
        digest.update(bytes);
        digest.update([0]);
    }
    format!("{:x}", digest.finalize())
}

fn canonical_ledger(ledger: &str) -> String {
    normalize_newlines(ledger)
        .lines()
        .map(|line| {
            if line.starts_with("| C8-P3-") && cells(line).len() == 9 {
                let mut values = cells(line)
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
                values[5] = "<source-identity>".into();
                format!("| {} |", values.join(" | "))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn guarantee_contract(line: &str) -> String {
    let row = cells(line);
    assert_eq!(row.len(), 9);
    [
        row[0], row[1], row[2], row[3], row[4], row[6], row[7], row[8],
    ]
    .join("|")
}

fn history_contracts(ledger: &str, start: &str, end: &str, prefix: &str) -> BTreeSet<String> {
    let tail = ledger.split_once(start).expect("history start").1;
    let body = if end.is_empty() {
        tail
    } else {
        tail.split_once(end).unwrap().0
    };
    let rows = body
        .lines()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    let contracts = rows
        .iter()
        .map(|line| cells(line).join("|"))
        .collect::<BTreeSet<_>>();
    assert_eq!(contracts.len(), rows.len());
    contracts
}

fn rows_between<'a>(source: &'a str, start: &str, end: &str, prefix: &str) -> Vec<&'a str> {
    source
        .split_once(start)
        .unwrap()
        .1
        .split_once(end)
        .unwrap()
        .0
        .lines()
        .filter(|line| line.starts_with(prefix))
        .collect()
}

fn cells(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').map(str::trim).collect()
}

fn normalize_newlines(source: &str) -> String {
    source.replace("\r\n", "\n")
}
fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn ledger_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-3-closure-ledger.md"
}
fn source_closure_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-3-source-closure.csv"
}
fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}
fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap()
        .to_path_buf()
}
