---- MODULE DurabilityRecovery ----
EXTENDS Naturals

VARIABLES wal, walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed

vars == <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>

Init ==
    /\ wal = "Absent"
    /\ walFenceCompleted = FALSE
    /\ page = "Clean"
    /\ checkpoint = "Absent"
    /\ directorySync = "Absent"
    /\ recovery = "Absent"
    /\ root = "Absent"
    /\ crashed = FALSE

WalPropose == /\ ~crashed /\ wal = "Absent" /\ wal' = "Proposed" /\ walFenceCompleted' = FALSE /\ UNCHANGED <<page, checkpoint, directorySync, recovery, root, crashed>>
WalWrite == /\ ~crashed /\ wal = "Proposed" /\ wal' = "Memory" /\ UNCHANGED <<walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>
WalFenceRequest == /\ ~crashed /\ wal = "Memory" /\ wal' = "FenceRequested" /\ UNCHANGED <<walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>
WalFenceComplete == /\ ~crashed /\ wal = "FenceRequested" /\ wal' = "FenceCompleted" /\ walFenceCompleted' = TRUE /\ UNCHANGED <<page, checkpoint, directorySync, recovery, root, crashed>>
WalAcknowledge == /\ ~crashed /\ wal = "FenceCompleted" /\ walFenceCompleted /\ wal' = "Acknowledged" /\ UNCHANGED <<walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>

PageRequest == /\ ~crashed /\ page = "Clean" /\ page' = "Requested" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, recovery, root, crashed>>
PageComplete == /\ ~crashed /\ page = "Requested" /\ wal = "Acknowledged" /\ page' = "Durable" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, recovery, root, crashed>>
PageUncertain == /\ ~crashed /\ page = "Requested" /\ page' = "Uncertain" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, recovery, root, crashed>>

CheckpointBegin == /\ ~crashed /\ checkpoint = "Absent" /\ checkpoint' = "Begun" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, recovery, root, crashed>>
CheckpointDurable == /\ ~crashed /\ checkpoint = "Begun" /\ wal = "Acknowledged" /\ page = "Durable" /\ checkpoint' = "Durable" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, recovery, root, crashed>>
DirectorySyncComplete == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync' = "Completed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, recovery, root, crashed>>
DirectorySyncFail == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync' = "Failed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, recovery, root, crashed>>
CheckpointPublish == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync = "Completed" /\ checkpoint' = "Published" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, recovery, root, crashed>>
CheckpointSelect == /\ ~crashed /\ checkpoint = "Published" /\ checkpoint' = "Selected" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, recovery, root, crashed>>

ReplayRequire == /\ ~crashed /\ checkpoint = "Selected" /\ recovery = "Absent" /\ recovery' = "Required" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, root, crashed>>
ReplayApply == /\ ~crashed /\ recovery = "Required" /\ recovery' = "Applied" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, root, crashed>>
ReplaySkip == /\ ~crashed /\ recovery = "Required" /\ recovery' = "Skipped" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, root, crashed>>
ReplayRejectGeneration == /\ ~crashed /\ recovery = "Required" /\ recovery' = "RejectedGeneration" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, root, crashed>>
RootPending == /\ ~crashed /\ checkpoint = "Selected" /\ root = "Absent" /\ root' = "Pending" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, crashed>>
RootComplete == /\ ~crashed /\ root = "Pending" /\ recovery \in {"Applied", "Skipped"} /\ root' = "Completed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, crashed>>

Crash == /\ ~crashed /\ crashed' = TRUE /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, root>>
Reopen ==
    /\ crashed
    /\ crashed' = FALSE
    /\ wal' = IF wal \in {"FenceCompleted", "Acknowledged"} THEN wal ELSE "Absent"
    /\ walFenceCompleted' = (wal \in {"FenceCompleted", "Acknowledged"} /\ walFenceCompleted)
    /\ page' = IF page = "Durable" THEN page ELSE "Clean"
    /\ checkpoint' = IF checkpoint = "Begun" THEN "Absent" ELSE checkpoint
    /\ directorySync' = IF directorySync = "Failed" THEN "Absent" ELSE directorySync
    /\ root' = IF root = "Completed" THEN root ELSE "Absent"
    /\ recovery' = IF root = "Completed" THEN recovery ELSE "Absent"

Next == WalPropose \/ WalWrite \/ WalFenceRequest \/ WalFenceComplete \/ WalAcknowledge
    \/ PageRequest \/ PageComplete \/ PageUncertain
    \/ CheckpointBegin \/ CheckpointDurable \/ DirectorySyncComplete \/ DirectorySyncFail
    \/ CheckpointPublish \/ CheckpointSelect \/ ReplayRequire \/ ReplayApply \/ ReplaySkip
    \/ ReplayRejectGeneration
    \/ RootPending \/ RootComplete \/ Crash \/ Reopen

Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ wal \in {"Absent", "Proposed", "Memory", "FenceRequested", "FenceCompleted", "Acknowledged"}
    /\ walFenceCompleted \in BOOLEAN
    /\ page \in {"Clean", "Requested", "Durable", "Uncertain"}
    /\ checkpoint \in {"Absent", "Begun", "Durable", "Published", "Selected"}
    /\ directorySync \in {"Absent", "Completed", "Failed"}
    /\ recovery \in {"Absent", "Required", "Applied", "Skipped", "RejectedGeneration"}
    /\ root \in {"Absent", "Pending", "Completed"}
    /\ crashed \in BOOLEAN

PageRequiresWal == page = "Durable" => wal = "Acknowledged"
AcknowledgmentRequiresCompletedFence == wal = "Acknowledged" => walFenceCompleted
PublishedCheckpointHasFrontier == checkpoint \in {"Published", "Selected"} => directorySync = "Completed"
ReplayRequiresSelection == recovery \in {"Required", "Applied", "Skipped", "RejectedGeneration"} => checkpoint = "Selected"
RootRequiresReplay == root = "Completed" => recovery \in {"Applied", "Skipped"}
====
