---- MODULE DurabilityAcknowledgmentMutant ----
EXTENDS DurabilityRecovery

VARIABLE mutantEdge
mutantVars == <<vars, mutantEdge>>

MutantInit == Init /\ mutantEdge = "None"
NormalNext == Next /\ UNCHANGED mutantEdge
AcknowledgeBeforeCompleteDurability ==
    /\ ~crashed
    /\ wal = "Memory"
    /\ physicalAcknowledged' = TRUE
    /\ mutantEdge' = "AcknowledgeBeforeCompleteDurability"
    /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>
MutantNext == NormalNext \/ AcknowledgeBeforeCompleteDurability
MutantSpec == MutantInit /\ [][MutantNext]_mutantVars
====
