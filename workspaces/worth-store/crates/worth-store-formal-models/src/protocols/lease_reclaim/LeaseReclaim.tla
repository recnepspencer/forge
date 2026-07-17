---- MODULE LeaseReclaim ----
EXTENDS Naturals

VARIABLES lease, leaseGeneration, reclaimed, reused, identityGeneration, crashed, leaked
vars == <<lease, leaseGeneration, reclaimed, reused, identityGeneration, crashed, leaked>>

Init == /\ lease = "Absent" /\ leaseGeneration = 1 /\ reclaimed = FALSE /\ reused = FALSE /\ identityGeneration = 1 /\ crashed = FALSE /\ leaked = FALSE
Acquire == /\ lease = "Absent" /\ ~reclaimed /\ lease' = "Active" /\ UNCHANGED <<leaseGeneration, reclaimed, reused, identityGeneration, crashed, leaked>>
Release == /\ lease = "Active" /\ lease' = "Released" /\ leaseGeneration' = leaseGeneration + 1 /\ leaked' = FALSE /\ UNCHANGED <<reclaimed, reused, identityGeneration, crashed>>
Revoke == /\ lease = "Active" /\ lease' = "Revoked" /\ leaseGeneration' = leaseGeneration + 1 /\ leaked' = FALSE /\ UNCHANGED <<reclaimed, reused, identityGeneration, crashed>>
Expire == /\ lease = "Active" /\ lease' = "ExpiredNoAuthority" /\ UNCHANGED <<leaseGeneration, reclaimed, reused, identityGeneration, crashed, leaked>>
Leak == /\ lease = "Active" /\ leaked' = TRUE /\ UNCHANGED <<lease, leaseGeneration, reclaimed, reused, identityGeneration, crashed>>
Crash == /\ ~crashed /\ crashed' = TRUE /\ UNCHANGED <<lease, leaseGeneration, reclaimed, reused, identityGeneration, leaked>>
Reclaim == /\ lease \in {"Absent", "Released", "Revoked"} /\ reclaimed' = TRUE /\ UNCHANGED <<lease, leaseGeneration, reused, identityGeneration, crashed, leaked>>
Reuse == /\ reclaimed /\ ~reused /\ identityGeneration' = identityGeneration + 1 /\ reused' = TRUE /\ UNCHANGED <<lease, leaseGeneration, reclaimed, crashed, leaked>>
OwnedCopy == /\ lease = "Active" /\ lease' = "Released" /\ leaseGeneration' = leaseGeneration + 1 /\ leaked' = FALSE /\ UNCHANGED <<reclaimed, reused, identityGeneration, crashed>>
DenyReclaim == /\ lease = "Active" /\ UNCHANGED vars
DenyReuse == /\ ~reclaimed /\ UNCHANGED vars

Next == Acquire \/ Release \/ Revoke \/ Expire \/ Leak \/ Crash \/ Reclaim \/ Reuse
        \/ OwnedCopy \/ DenyReclaim \/ DenyReuse
Spec == Init /\ [][Next]_vars
TypeOK == /\ lease \in {"Absent", "Active", "Released", "Revoked", "ExpiredNoAuthority"} /\ leaseGeneration \in Nat /\ reclaimed \in BOOLEAN /\ reused \in BOOLEAN /\ identityGeneration \in Nat /\ crashed \in BOOLEAN /\ leaked \in BOOLEAN
LiveLeaseBlocksReclaim == lease = "Active" => ~reclaimed
ExpiryBlocksReclaim == lease = "ExpiredNoAuthority" => ~reclaimed
ReuseRequiresReclaim == reused => reclaimed
ReuseAdvancesGeneration == reused => identityGeneration > 1
LeakNeverOpensReclaim == leaked => ~reclaimed
====
