---- MODULE SharedReclaimMutant ----
EXTENDS SharedFrontiers

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
ReclaimReachableAuthority ==
    /\ reachability = "Reachable"
    /\ oldAuthorityReachable
    /\ reachability' = "Reused"
    /\ mutantEdge' = "ReclaimReachableAuthority"
    /\ UNCHANGED <<durability, visibility, quarantine, admission,
                    recoveryPrecedence, verificationAdvanced,
                    oldAuthorityReachable, crashed, externalPublished, lastAction>>
MutantNext == NormalNext \/ ReclaimReachableAuthority
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
