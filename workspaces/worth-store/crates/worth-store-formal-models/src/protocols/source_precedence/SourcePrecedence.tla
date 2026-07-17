---- MODULE SourcePrecedence ----
EXTENDS Naturals, FiniteSets

CONSTANTS Candidates, AuthorityCandidates, AdvisoryCandidates, QuarantineCandidates
VARIABLES discovered, admitted, advisory, rejected, quarantined, selected, contradiction
vars == <<discovered, admitted, advisory, rejected, quarantined, selected, contradiction>>

Init ==
    /\ discovered = {}
    /\ admitted = {}
    /\ advisory = {}
    /\ rejected = {}
    /\ quarantined = {}
    /\ selected = {}
    /\ contradiction = FALSE

Discover(c) == /\ c \in Candidates /\ discovered' = discovered \cup {c} /\ UNCHANGED <<admitted, advisory, rejected, quarantined, selected, contradiction>>
Admit(c) == /\ c \in discovered \cap AuthorityCandidates /\ admitted' = admitted \cup {c} /\ UNCHANGED <<discovered, advisory, rejected, quarantined, selected, contradiction>>
Advise(c) == /\ c \in discovered \cap AdvisoryCandidates /\ advisory' = advisory \cup {c} /\ UNCHANGED <<discovered, admitted, rejected, quarantined, selected, contradiction>>
Reject(c) == /\ c \in discovered /\ c \notin selected /\ rejected' = rejected \cup {c} /\ UNCHANGED <<discovered, admitted, advisory, quarantined, selected, contradiction>>
Quarantine(c) == /\ c \in discovered \cap QuarantineCandidates /\ c \notin selected /\ quarantined' = quarantined \cup {c} /\ UNCHANGED <<discovered, admitted, advisory, rejected, selected, contradiction>>
PreserveContradiction == /\ Cardinality(discovered) > 1 /\ contradiction' = TRUE /\ UNCHANGED <<discovered, admitted, advisory, rejected, quarantined, selected>>
Select(c) == /\ c \in admitted /\ c \notin rejected /\ c \notin quarantined /\ selected' = {c} /\ UNCHANGED <<discovered, admitted, advisory, rejected, quarantined, contradiction>>
Deny == /\ selected = {} /\ UNCHANGED vars

Next == (\E c \in Candidates: Discover(c) \/ Admit(c) \/ Advise(c) \/ Reject(c) \/ Quarantine(c) \/ Select(c)) \/ PreserveContradiction \/ Deny
Spec == Init /\ [][Next]_vars

TypeOK == /\ discovered \subseteq Candidates /\ admitted \subseteq AuthorityCandidates /\ advisory \subseteq AdvisoryCandidates /\ rejected \subseteq Candidates /\ quarantined \subseteq QuarantineCandidates /\ selected \subseteq AuthorityCandidates /\ contradiction \in BOOLEAN
CandidateClassesAreDisjoint ==
    /\ AuthorityCandidates \cup AdvisoryCandidates \cup QuarantineCandidates = Candidates
    /\ AuthorityCandidates \cap AdvisoryCandidates = {}
    /\ AuthorityCandidates \cap QuarantineCandidates = {}
    /\ AdvisoryCandidates \cap QuarantineCandidates = {}
SelectedWasAdmitted == selected \subseteq admitted
QuarantineCannotWin == selected \cap quarantined = {}
LosingCandidatesRemainVisible == admitted \cup advisory \cup rejected \cup quarantined \subseteq discovered
SingleSelection == Cardinality(selected) <= 1
====
