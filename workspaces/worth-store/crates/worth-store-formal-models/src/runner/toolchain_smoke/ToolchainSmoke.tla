--------------------------- MODULE ToolchainSmoke ---------------------------
VARIABLE ready

Init == ready = FALSE

Advance ==
    /\ ready = FALSE
    /\ ready' = TRUE

RemainReady ==
    /\ ready = TRUE
    /\ UNCHANGED ready

Next == Advance \/ RemainReady

Spec == Init /\ [][Next]_ready

TypeInvariant == ready \in BOOLEAN
=============================================================================
