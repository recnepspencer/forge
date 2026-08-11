use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const GUARANTEES: [&str; 12] = [
    "C8-P2-ROOT-01",
    "C8-P2-AUTHORITY-01",
    "C8-P2-BINDING-01",
    "C8-P2-IDENTITY-01",
    "C8-P2-SESSION-01",
    "C8-P2-LIMITS-01",
    "C8-P2-SELECTOR-01",
    "C8-P2-COORDINATION-01",
    "C8-P2-EFFECT-01",
    "C8-P2-API-01",
    "C8-P2-COMPILE-01",
    "C8-P2-LEDGER-01",
];

const SOURCE_CLOSURE_SHA256: &str =
    "cdd3163c89e30ebb819a84797c1bd47cdcbad41c15b62acf76174b42c27996e1";

const GUARANTEE_CONTRACTS: [&str; 12] = [
    "C8-P2-ROOT-01|2|Recovery opens only an initialized existing Store root through sealed existing-only namespace identity and lease policies and cannot name the ordinary root-creating backend lane|Store-gated physical backend recovery qualification|absent incomplete exclusive and two-process journeys plus an ordinary-backend compile attack|PROVED|C8-P2-F01 C8-P2-F03 C8-P2-F04 C8-P2-F11 C8-P2-F27|none",
    "C8-P2-AUTHORITY-01|2|One concrete non-cloneable platform authority is minted in-process from qualified media and Store-sampled freshness whose owner-issued identity becomes a process-registered session before coordination|recovery runtime entry authority and Store freshness registration owner|warnings-denied entry journey plus forged witness raw-session unregistered-coordination clone and duplicate-coordinator compile attacks|PROVED|C8-P2-F05 C8-P2-F12 C8-P2-F14 C8-P2-F19 C8-P2-F27|none",
    "C8-P2-BINDING-01|2|Worth Proof bindings compare a separate request presentation across six exact owner-issued entry axes and retain them with persisted Store identity in a seven-axis admitted world without exporting a reusable match token|recovery runtime authority binding|three production-presentable hostile requests six semantic one-axis comparator mutants and a real foreign-Store persisted-selector boundary attack|PROVED|C8-P2-F02 C8-P2-F15 C8-P2-F25 C8-P2-F27|none",
    "C8-P2-IDENTITY-01|2|Stable Store identity is neither backend-bound nor present in platform authority before exclusive ownership and joins once from the persisted namespace record in the admitted-world binding|backend persisted identity admission|initialized Store journey real foreign-Store persisted-selector rejection and post-lease identity-replacement attack|PROVED|C8-P2-F02 C8-P2-F03 C8-P2-F25 C8-P2-F27|none",
    "C8-P2-SESSION-01|2|Each Store-issued recovery session identity is process-registered under concrete Store authority wrapped by one linear runtime resource terminated once and reused as the exact coordination partition binding|Store registration and recovery runtime session owners|registration cancellation owner-visible abandonment independently checked partitions and non-clone unregistered plus duplicate-coordinator compile attacks|PROVED|C8-P2-F12 C8-P2-F14 C8-P2-F17 C8-P2-F19 C8-P2-F27|later recovered blocked and publication-indeterminate terminals arrive with their owning phases",
    "C8-P2-LIMITS-01|2|Every recovery dimension has an explicit finite nonzero admitted bound before recovery allocation or effects|recovery runtime entry limits|zero and above-hard-maximum mutant for each of eighteen dimensions|PROVED|none|phase-specific observed-limit denials arrive with bounded discovery and execution",
    "C8-P2-SELECTOR-01|2|Ordinary Store initialization persists one unlinked fixed-role current selector and every successor root publication replaces linked previous current and bootstrap records under one observed ordered compound root-protocol effect with indeterminate partial-prefix outcomes|physical-format root selector and Store root publication owners|selector format attacks explicit genesis absence real namespace-durable reciprocal linkage and deterministic post-previous plus post-current fault worlds|PROVED|C8-P2-F20 C8-P2-F21 C8-P2-F22 C8-P2-F23 C8-P2-F24 C8-P2-F27|Phase 3 owns selection from partial selector triplets",
    "C8-P2-COORDINATION-01|2|Registered-session admission delegates to a Store owner that installs four typed aspect-native recovery bases independently checked against stable Store and the exact registered runtime session then traverses canonical bounded C5.1 scheduler admission|Store recovery coordination owner|admitted-world journey plus compile attacks prove registration exact observed roles partitions one-shot Signal ownership one admitted and released discovery reservation and quiescent shutdown|PROVED|C8-P2-F06 C8-P2-F10 C8-P2-F12 C8-P2-F14 C8-P2-F17 C8-P2-F19 C8-P2-F27|scheduler work-family submissions and effect execution arrive with their owning phases",
    "C8-P2-EFFECT-01|2|Admission and every refusal path report zero recovery staging publication cleanup or reopen effects from the complete owned C4 mutating-role observation|backend media effect observation joined by recovery admission|success and refusal assertions plus real write append truncate allocate file-create and directory-create mutants read the owner counters|PROVED|C8-P2-F08 C8-P2-F13 C8-P2-F18 C8-P2-F27|C4 lease observation is lifecycle evidence and not a recovery data effect",
    "C8-P2-API-01|2|Phase 2 exposes exactly the production-reachable request admission authority limits outcome admitted-state recovery-media Store-freshness registered-session and Store-coordination facades through the locked freshness port plus admission-only production entry|runtime backend and Store audience facades|live cross-crate facade derivation exact inventory equality two-port lock consuming coordination authority direct-dependency cut and fresh production-binary Store journeys|PROVED|C8-P2-F07 C8-P2-F09 C8-P2-F11 C8-P2-F12 C8-P2-F14 C8-P2-F19 C8-P2-F26 C8-P2-F27|top-level recover and later progression surfaces remain assigned to later phases",
    "C8-P2-COMPILE-01|2|Bare witnesses raw freshness sessions unregistered or duplicate coordination live Store C7 handoff inputs ordinary backend qualification authority duplication and duplicate session termination cannot satisfy entry|recovery runtime compile tests|ten warnings-denied trybuild attacks including one-shot freshness registration and ordinary backend import paths|PROVED|C8-P2-F05 C8-P2-F07 C8-P2-F11 C8-P2-F14 C8-P2-F19|final handoff sealing belongs to Phase 6",
    "C8-P2-LEDGER-01|2|Every Phase 2 guarantee has exact row semantics status causal source membership finding reopening and retained independent audit history|Phase 2 ledger gate|canonical source-set row history audit duplicate omission stale foreign-causality and final-format mutants|PROVED|C8-P2-F09 C8-P2-F10 C8-P2-F11 C8-P2-F12 C8-P2-F13 C8-P2-F14 C8-P2-F15 C8-P2-F16 C8-P2-F17 C8-P2-F18 C8-P2-F19 C8-P2-F20 C8-P2-F21 C8-P2-F22 C8-P2-F23 C8-P2-F24 C8-P2-F25 C8-P2-F26 C8-P2-F27|none",
];

const FINDINGS: [&str; 27] = [
    "C8-P2-F01|Medium|C8-P2-ROOT-01|Recovery-mode branching extended backend owner admission into one 118-line mixed-level function|Decomposed namespace opening directory admission lease acquisition and artifact-root admission into named semantic steps|Rust scrutiny reports no collapsed long owner-admission function and focused backend tests pass",
    "C8-P2-F02|High|C8-P2-BINDING-01 C8-P2-IDENTITY-01|Entry used a hand-written comparison discarded its binding before admission and forced two test denials|Replaced it with Worth Proof six-axis entry and seven-axis admitted-world bindings using real alternate owner worlds|warnings-denied hostile-axis and foreign-Store twins pass without forced-result flags",
    "C8-P2-F03|High|C8-P2-ROOT-01 C8-P2-IDENTITY-01|Recovery classification bound Store identity before the exclusive lease then attempted a second ignored bind|Removed pre-lease binding and admitted plus bound persisted identity once under the live owner lease|post-lease identity-replacement attack proves the admitted identity is read only after ownership",
    "C8-P2-F04|High|C8-P2-ROOT-01|Existing-only namespace identity and mutation-lock legality flowed through positional creation booleans|Installed sealed ordinary and existing-recovery policy types with distinct lock and identity admission functions|absent and incomplete roots remain byte-for-byte non-created in focused backend tests",
    "C8-P2-F05|High|C8-P2-AUTHORITY-01 C8-P2-COMPILE-01|The public Store freshness port minted authority from arbitrary caller-provided nonzero session bytes|Store now samples its private freshness identity and accepts no caller session value|real public mint-path raw-session trybuild attack is rejected",
    "C8-P2-F06|High|C8-P2-COORDINATION-01|Coordination retained an empty SignalGraph and counter recorder rather than a live bounded runtime|Built the four named recovery contract nodes in a live SignalRuntime with backend-qualified bounded scheduler capacity and quiescent shutdown|admitted-world journey observes every contract node and zero active reservations",
    "C8-P2-F07|High|C8-P2-API-01 C8-P2-COMPILE-01|Admission was private the production binary was a placeholder and its first replacement did not compile|Added consuming request admission a bounded admission-only CLI and real child-process Store journeys|production binary admits exact Store identity rejects invalid roots and releases ownership",
    "C8-P2-F08|High|C8-P2-EFFECT-01|Recovery effects were asserted through an atomic that no effect path could increment|Removed the atomic and derive success refusal and cancellation observations from the owned C4 media counters|actual backend effect observation reports zero for success and every-axis refusal",
    "C8-P2-F09|High|C8-P2-API-01 C8-P2-LEDGER-01|Editable source sets nonempty ledger prose and a planned CSV could self-certify implementation and history|Locked exact row finding audit and causal source contracts and parse delivered deep public facades|wrong-row history source-set deep-method extra-export and stale-digest mutants are rejected",
    "C8-P2-F10|High|C8-P2-COORDINATION-01 C8-P2-LEDGER-01|Four string-labelled always-live Signal nodes and a disconnected queue recorder self-certified the C5.1 coordination guarantee|Moved coordination behind a Store owner that installs native aspect contracts exact partitions the production Signal owner and canonical scheduler reservation|production readiness checks exact identities roles families partitions and one admitted released C5.1 discovery reservation",
    "C8-P2-F11|High|C8-P2-ROOT-01 C8-P2-API-01 C8-P2-COMPILE-01 C8-P2-LEDGER-01|Recovery directly depended on a feature-unified backend facade that also exported ordinary root-creating qualification|Removed the direct backend dependency and routed recovery media only through the Store audience facade|cargo direct-edge proof and compile-fail ordinary backend import attack pass",
    "C8-P2-F12|High|C8-P2-AUTHORITY-01 C8-P2-SESSION-01 C8-P2-COORDINATION-01 C8-P2-API-01 C8-P2-LEDGER-01|Coordination partitions used a Store freshness sample that was not the runtime session identity named by the entry binding|Made the Store-issued freshness identity the sole runtime session identity and reused it for every recovery coordination partition|admitted entry binding session and native coordination partitions now share one owner-issued identity",
    "C8-P2-F13|High|C8-P2-EFFECT-01 C8-P2-LEDGER-01|Recovery effect observation omitted completed positioned write append truncate and allocate roles so real mutations could report zero|Joined every completed mutating role to the owner counter observation while retaining explicit lease-lifecycle exclusions|real write append truncate allocation and namespace mutations each make zero-effect evidence nonzero",
    "C8-P2-F14|High|C8-P2-AUTHORITY-01 C8-P2-SESSION-01 C8-P2-COORDINATION-01 C8-P2-API-01 C8-P2-COMPILE-01 C8-P2-LEDGER-01|A third public coordination port contradicted the two-port lock and freshness alone could construct an unregistered coordinator|Removed the third port and required a concrete process-registered Store session authority for coordination admission|unregistered freshness compile attack fails and the admitted journey binds the registered identity into all four native partitions",
    "C8-P2-F15|High|C8-P2-BINDING-01 C8-P2-LEDGER-01|Six-axis evidence rewrote private authority bindings through cfg-test production backdoors while three request comparisons were tautological|Removed every test-only production mutator and compare a separately retained request presentation through the production semantic boundary|three production-presentable drift journeys and six isolated semantic comparator mutants pass without mutating an authority",
    "C8-P2-F16|Medium|C8-P2-LEDGER-01|The frozen final candidate failed the exact workspace formatter check|Applied the workspace formatter to the full scoped Rust set|cargo fmt with the worth-store workspace manifest passes in check mode",
    "C8-P2-F17|High|C8-P2-SESSION-01 C8-P2-COORDINATION-01 C8-P2-LEDGER-01|Signal semantics produced the same partition array that readiness reused as its expected oracle so Store or session omission remained green|Independently validate installed Signal observations from the raw admitted Store identity and registered session before coordination admission|the admitted-world journey fails if either partition input is omitted and exact observed roles families and partitions agree",
    "C8-P2-F18|High|C8-P2-EFFECT-01 C8-P2-LEDGER-01|Complete effect observation omitted successful directory creation while its namespace mutant exercised only file creation|Added completed CreateDirectory observation and the real artifact-tree directory producer chain|real directory creation raises recovery effect evidence after write append truncate allocate and file-create mutations",
    "C8-P2-F19|High|C8-P2-AUTHORITY-01 C8-P2-SESSION-01 C8-P2-COORDINATION-01 C8-P2-API-01 C8-P2-COMPILE-01 C8-P2-LEDGER-01|A registered session authority admitted coordination by shared reference and could be reused or dropped while coordinators remained live|Made coordination admission consume the registration authority and retain it inside the one live coordinator|duplicate coordinator admission fails with E0382 and registration lifetime equals the coordinator lifetime",
    "C8-P2-F20|High|C8-P2-SELECTOR-01 C8-P2-LEDGER-01|Phase 2 closed while its required durable previous-root selector remained only an in-memory retained root|Added a framed fixed-role selector format current-selector initialization and compound successor previous current plus bootstrap publication effect|format attacks and the real generation-one-to-two publication journey prove both successor selectors and their exact linkage",
    "C8-P2-F21|High|C8-P2-SELECTOR-01 C8-P2-LEDGER-01|Selector closure falsely claimed and syntax-certified a previous selector at genesis by reusing the current-selector constructor and write as its oracle|Defined genesis as one unlinked current selector removed initialization from previous-selector causality and retained previous selectors only for successor publication|the real journey asserts genesis previous absence then exact linked previous and current records after generation one-to-two publication",
    "C8-P2-F22|High|C8-P2-SELECTOR-01 C8-P2-LEDGER-01|The governing C8 specification still called the delivered previous-root protocol one of four producer gaps|Reconciled the authoritative persisted-input section to the exact genesis and successor selector protocol and retained only the three undelivered gaps|specification inventory and Phase 2 ledger now name one identical protocol and remaining gap set",
    "C8-P2-F23|High|C8-P2-SELECTOR-01 C8-P2-LEDGER-01|The successor journey checked reciprocal selector identities but not either linked root generation so wrong nonzero generation links remained green|Observed both reciprocal generation links plus shared Store and format identities from the real persisted selector files|the namespace-durable generation-one-to-two journey fails any wrong linked generation Store or format substitution",
    "C8-P2-F24|High|C8-P2-SELECTOR-01 C8-P2-LEDGER-01|Three sequential selector and catalog renames were reported as one compound effect but offered no deterministic boundary after rename one or two so partial crash states were unproved|Retained one semantic operation identity while interposing each ordered rename and classified any nonempty prefix failure as indeterminate|backend fault worlds stop after previous and after current and assert the exact persisted prefix while the specification assigns selection to Phase 3",
    "C8-P2-F25|High|C8-P2-BINDING-01 C8-P2-IDENTITY-01 C8-P2-LEDGER-01|The retained foreign-Store proof still called a cfg-test production binding mutation hook and could stay green if persisted discovery ignored Store identity|Removed the hook and substitute the selector and manifest from a separately initialized real Store through the production persisted-source boundary|the primary admitted world rejects the alternate Store selector records one rejected current root and terminates Blocked with zero effects",
    "C8-P2-F26|High|C8-P2-API-01 C8-P2-LEDGER-01|The delivered facade syntax derivation skipped item kinds public namespace children module aliases and configuration variants and resolved deep dependency bypasses by leaf name|Enumerate the complete facade against the full immutable pre-C8 revision retain exact module and visibility variants require contractual dependency facades resolve canonical impl targets account for every direct public item kind recurse direct public namespaces and fail closed before projection on every module re-export alias plus every unproved associated-item macro extern-crate foreign-module or verbatim surface|every delivered Phase 2 and Phase 3 surface has one exact row and static union extern-crate foreign-module root-macro macro-export public-namespace-macro private-module-control exact-accessor qualified-macro associated-macro nested-public local-module-alias grouped-self-module-alias root-crate-alias dependency-crate-alias private-import-alias alias-file-variant external-module-hostile-twin external-glob self-cycle two-module-cycle repeated-acyclic-alias visibility-order glob outside-alias deep-external canonical-method supported-cfg orphan disabled-module path-remap file-variant inline-variant cfg-attr and baseline mutants are rejected",
    "C8-P2-F27|High|C8-P2-ROOT-01 C8-P2-AUTHORITY-01 C8-P2-BINDING-01 C8-P2-IDENTITY-01 C8-P2-SESSION-01 C8-P2-SELECTOR-01 C8-P2-COORDINATION-01 C8-P2-EFFECT-01 C8-P2-API-01 C8-P2-LEDGER-01|Phase 6 extended publication reopen construction and facade paths inside the inherited Phase 2 causal closures while their source identities still described the pre-publication implementation|Rebound each affected Phase 2 guarantee to its exact current causal sources while preserving the admission-only authority and zero-effect entry contracts|warnings-denied recovery-runtime Phase 2 ledger C8 boundary C7 boundary and Phase 6 publication journeys pass",
];

const AUDITS: [&str; 8] = [
    "/root/c8_phase2_critic|gpt-5.6-sol high|Phase 2 implementation tests composition public entry and ledger|DEFECTS ACCEPTED|C8-P2-F02 C8-P2-F03 C8-P2-F04 C8-P2-F05 C8-P2-F06 C8-P2-F07 C8-P2-F08 C8-P2-F09|root corrections implemented; independent closure re-review pending",
    "/root/c8_phase2_postfix_critic|gpt-5.6-sol high|Frozen Phase 2 post-fix implementation tests dependency audience and composition|DEFECTS ACCEPTED|C8-P2-F10 C8-P2-F11|root corrections implemented; fresh independent closure review pending",
    "/root/c8_phase2_final_critic|gpt-5.6-sol high|Frozen Phase 2 authority binding effects public ports tests and composition|DEFECTS ACCEPTED|C8-P2-F13 C8-P2-F14 C8-P2-F15 C8-P2-F16|root corrections implemented; fresh independent closure review pending",
    "/root/c8_phase2_final_closure_critic|gpt-5.6-sol high|Frozen Phase 2 final source effects coordination authority tests ledger and composition|DEFECTS ACCEPTED|C8-P2-F17 C8-P2-F18 C8-P2-F19|root corrections implemented; exact stable-snapshot closure rerun pending",
    "/root/c8_phase2_final_closure_critic|gpt-5.6-sol high|Post-F19 exact 91-file Phase 2 stable-snapshot closure|CLEAN|none|aggregate 790fbd6e767135fe316df6fd96e19fa82e4e8db0a89199572326421752f90197; all required gates passed with no supported material defect",
    "/root/c8_phase2_final_closure_critic|gpt-5.6-sol high|Post-F24 exact 119-file durable-selector prerequisite closure|CLEAN|none|aggregate 623914e5670f98494248c389f3bc845fa58f6bec669c0278531ebe39b6f349a9; format journey partial-prefix Phase 1 Phase 2 formatting line-cap and composition gates passed",
    "/root/c8_phase3_final_critic|gpt-5.6-sol high|Frozen Phase 3 implementation and inherited Phase 2 binding prerequisite|DEFECTS ACCEPTED|C8-P2-F25|production backdoor removed; final Phase 3 closure re-audit pending",
    "/root/c8_phase3_final_closure_critic_v2|gpt-5.6-sol high|Frozen Phase 3 final functional closure and inherited delivered-facade evidence|DEFECTS ACCEPTED|C8-P2-F26|live delivered-facade derivation implemented; stable closure rerun pending",
];

#[test]
fn phase_two_ledger_is_exact_closed_and_source_bound() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let source_contract = read(&root.join(source_closure_path()));
    validate_source_contract(&root, &source_contract);
    validate_ledger(
        &root,
        &ledger,
        &parse_source_closures(&root, &source_contract),
    );
}

#[test]
fn causal_history_and_contract_mutants_cannot_self_certify() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let source_contract = read(&root.join(source_closure_path()));
    let closures = parse_source_closures(&root, &source_contract);
    let root_row = ledger
        .lines()
        .find(|line| line.starts_with("| C8-P2-ROOT-01"))
        .unwrap();
    let mutants = [
        ledger.replacen(root_row, "", 1),
        ledger.replacen(root_row, &format!("{root_row}\n{root_row}"), 1),
        ledger.replacen("sealed existing-only", "boolean existing-only", 1),
        ledger.replacen("C8-P2-F03 C8-P2-F04", "C8-P2-F09", 1),
        ledger.replacen("| PROVED |", "| ACTIVE |", 1),
        ledger.replacen("| C8-P2-F08 |", "| C8-P2-F88 |", 1),
        ledger.replacen("DEFECTS ACCEPTED", "CLEAN", 1),
    ];
    for (index, mutant) in mutants.into_iter().enumerate() {
        assert!(
            std::panic::catch_unwind(|| validate_ledger(&root, &mutant, &closures)).is_err(),
            "ledger mutant {index} self-certified"
        );
    }
    let foreign = source_contract.replacen(
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
        "<!-- c8-phase2-ledger:start -->",
        "<!-- c8-phase2-ledger:end -->",
        "| C8-P2-",
    );
    assert_eq!(
        rows.len(),
        GUARANTEES.len(),
        "missing or duplicate guarantee row"
    );
    let contracts = rows
        .iter()
        .map(|row| guarantee_contract(row))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        contracts,
        GUARANTEE_CONTRACTS.into_iter().map(str::to_owned).collect()
    );
    let identities = rows
        .iter()
        .map(|row| {
            let cells = cells(row);
            (cells[0].to_owned(), cells[5].to_owned())
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(identities.len(), GUARANTEES.len());
    let mismatches = GUARANTEES
        .into_iter()
        .filter_map(|guarantee| {
            let expected = source_identity(root, &closures[guarantee]);
            (identities[guarantee] != expected).then(|| format!("{guarantee}={expected}"))
        })
        .collect::<Vec<_>>();
    assert!(
        mismatches.is_empty(),
        "stale source identities: {mismatches:#?}"
    );
    assert_eq!(
        history_contracts(
            ledger,
            "## Phase 2 finding history",
            "## Independent audit history",
            "| C8-P2-F"
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
        assert!(
            closures
                .entry(guarantee.to_owned())
                .or_default()
                .insert(path.to_owned()),
            "duplicate causal source {guarantee} {path}"
        );
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
            if line.starts_with("| C8-P2-") && cells(line).len() == 9 {
                let mut values = cells(line)
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>();
                values[5] = "<source-identity>".to_owned();
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
        tail.split_once(end).expect("history end").0
    };
    let rows = body
        .lines()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    let contracts = rows
        .iter()
        .map(|line| cells(line).join("|"))
        .collect::<BTreeSet<_>>();
    assert_eq!(contracts.len(), rows.len(), "duplicate history row");
    contracts
}

fn rows_between<'a>(source: &'a str, start: &str, end: &str, prefix: &str) -> Vec<&'a str> {
    source
        .split_once(start)
        .expect("start marker")
        .1
        .split_once(end)
        .expect("end marker")
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
    "_docs/worth-store/physical-reconstruction-c8-phase-2-closure-ledger.md"
}

fn source_closure_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-2-source-closure.csv"
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repository root")
        .to_path_buf()
}
