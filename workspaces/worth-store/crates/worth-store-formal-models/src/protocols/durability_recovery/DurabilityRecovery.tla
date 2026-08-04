---- MODULE DurabilityRecovery ----
EXTENDS Naturals

VARIABLES wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed

vars == <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>

Init ==
    /\ wal = "Absent"
    /\ walFenceCompleted = FALSE
    /\ page = "Clean"
    /\ checkpoint = "Absent"
    /\ directorySync = "Absent"
    /\ physicalAcknowledged = FALSE
    /\ recovery = "Absent"
    /\ root = "Absent"
    /\ crashed = FALSE

WalPropose == /\ ~crashed /\ wal = "Absent" /\ wal' = "Proposed" /\ walFenceCompleted' = FALSE /\ UNCHANGED <<page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>
WalWrite == /\ ~crashed /\ wal = "Proposed" /\ wal' = "Memory" /\ UNCHANGED <<walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>
WalFenceRequest == /\ ~crashed /\ wal = "Memory" /\ wal' = "FenceRequested" /\ UNCHANGED <<walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>
WalFenceComplete == /\ ~crashed /\ wal = "FenceRequested" /\ wal' = "FenceCompleted" /\ walFenceCompleted' = TRUE /\ UNCHANGED <<page, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>

PageRequest == /\ ~crashed /\ wal = "FenceCompleted" /\ walFenceCompleted /\ page = "Clean" /\ page' = "Requested" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>
PageComplete == /\ ~crashed /\ page = "Requested" /\ wal = "FenceCompleted" /\ walFenceCompleted /\ page' = "Durable" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>
PageUncertain == /\ ~crashed /\ page = "Requested" /\ page' = "Uncertain" /\ UNCHANGED <<wal, walFenceCompleted, checkpoint, directorySync, physicalAcknowledged, recovery, root, crashed>>

CheckpointBegin == /\ ~crashed /\ checkpoint = "Absent" /\ checkpoint' = "Begun" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, physicalAcknowledged, recovery, root, crashed>>
CheckpointDurable == /\ ~crashed /\ checkpoint = "Begun" /\ wal = "FenceCompleted" /\ page = "Durable" /\ checkpoint' = "Durable" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, physicalAcknowledged, recovery, root, crashed>>
DirectorySyncComplete == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync' = "Completed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, physicalAcknowledged, recovery, root, crashed>>
DirectorySyncFail == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync' = "Failed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, physicalAcknowledged, recovery, root, crashed>>
CheckpointPublish == /\ ~crashed /\ checkpoint = "Durable" /\ directorySync = "Completed" /\ checkpoint' = "Published" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, physicalAcknowledged, recovery, root, crashed>>
CheckpointSelect == /\ ~crashed /\ checkpoint = "Published" /\ checkpoint' = "Selected" /\ UNCHANGED <<wal, walFenceCompleted, page, directorySync, physicalAcknowledged, recovery, root, crashed>>
PhysicalAcknowledge == /\ ~crashed /\ ~physicalAcknowledged /\ wal = "FenceCompleted" /\ walFenceCompleted /\ page = "Durable" /\ checkpoint = "Published" /\ physicalAcknowledged' = TRUE /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, recovery, root, crashed>>

ReplayRequire == /\ ~crashed /\ checkpoint = "Selected" /\ recovery = "Absent" /\ recovery' = "Required" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, root, crashed>>
ReplayApply == /\ ~crashed /\ recovery = "Required" /\ recovery' = "Applied" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, root, crashed>>
ReplaySkip == /\ ~crashed /\ recovery = "Required" /\ recovery' = "Skipped" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, root, crashed>>
ReplayRejectGeneration == /\ ~crashed /\ recovery = "Required" /\ recovery' = "RejectedGeneration" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, root, crashed>>
RootPending == /\ ~crashed /\ checkpoint = "Selected" /\ root = "Absent" /\ root' = "Pending" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, crashed>>
RootComplete == /\ ~crashed /\ root = "Pending" /\ recovery \in {"Applied", "Skipped"} /\ root' = "Completed" /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, crashed>>

Crash == /\ ~crashed /\ crashed' = TRUE /\ UNCHANGED <<wal, walFenceCompleted, page, checkpoint, directorySync, physicalAcknowledged, recovery, root>>
Reopen ==
    /\ crashed
    /\ crashed' = FALSE
    /\ wal' = IF wal = "FenceCompleted" THEN wal ELSE "Absent"
    /\ walFenceCompleted' = (wal = "FenceCompleted" /\ walFenceCompleted)
    /\ page' = IF page = "Durable" THEN page ELSE "Clean"
    /\ checkpoint' = IF checkpoint = "Begun" THEN "Absent" ELSE checkpoint
    /\ directorySync' = IF directorySync = "Failed" THEN "Absent" ELSE directorySync
    /\ physicalAcknowledged' = physicalAcknowledged
    /\ root' = IF root = "Completed" THEN root ELSE "Absent"
    /\ recovery' = IF root = "Completed" THEN recovery ELSE "Absent"

Next == WalPropose \/ WalWrite \/ WalFenceRequest \/ WalFenceComplete
    \/ PageRequest \/ PageComplete \/ PageUncertain
    \/ CheckpointBegin \/ CheckpointDurable \/ DirectorySyncComplete \/ DirectorySyncFail
    \/ CheckpointPublish \/ CheckpointSelect \/ PhysicalAcknowledge
    \/ ReplayRequire \/ ReplayApply \/ ReplaySkip \/ ReplayRejectGeneration
    \/ RootPending \/ RootComplete \/ Crash \/ Reopen

Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ wal \in {"Absent", "Proposed", "Memory", "FenceRequested", "FenceCompleted"}
    /\ walFenceCompleted \in BOOLEAN
    /\ page \in {"Clean", "Requested", "Durable", "Uncertain"}
    /\ checkpoint \in {"Absent", "Begun", "Durable", "Published", "Selected"}
    /\ directorySync \in {"Absent", "Completed", "Failed"}
    /\ physicalAcknowledged \in BOOLEAN
    /\ recovery \in {"Absent", "Required", "Applied", "Skipped", "RejectedGeneration"}
    /\ root \in {"Absent", "Pending", "Completed"}
    /\ crashed \in BOOLEAN

PageDispatchRequiresWal == page \in {"Requested", "Durable"} => wal = "FenceCompleted" /\ walFenceCompleted
PhysicalAcknowledgmentRequiresCompleteDurability == physicalAcknowledged => wal = "FenceCompleted" /\ walFenceCompleted /\ page = "Durable" /\ checkpoint \in {"Published", "Selected"} /\ directorySync = "Completed"
PublishedCheckpointHasFrontier == checkpoint \in {"Published", "Selected"} => directorySync = "Completed"
ReplayRequiresSelection == recovery \in {"Required", "Applied", "Skipped", "RejectedGeneration"} => checkpoint = "Selected"
RootRequiresReplay == root = "Completed" => recovery \in {"Applied", "Skipped"}
====
