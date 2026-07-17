---- MODULE LeaseReuseMutant ----
EXTENDS LeaseReclaim

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
ReuseWithLiveLease ==
    /\ lease = "Active"
    /\ reclaimed' = TRUE
    /\ reused' = TRUE
    /\ identityGeneration' = identityGeneration + 1
    /\ mutantEdge' = "ReuseWithLiveLease"
    /\ UNCHANGED <<lease, leaseGeneration, crashed, leaked>>
MutantNext == NormalNext \/ ReuseWithLiveLease
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
