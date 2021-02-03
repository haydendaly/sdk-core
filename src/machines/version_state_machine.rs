use rustfsm::{fsm, TransitionResult};

fsm! {
    pub(super) name VersionMachine; command VersionCommand; error VersionMachineError;

    Created --(CheckExecutionState, on_check_execution_state) --> Replaying;
    Created --(CheckExecutionState, on_check_execution_state) --> Executing;

    Executing --(Schedule, on_schedule) --> MarkerCommandCreated;
    Executing --(Schedule, on_schedule) --> Skipped;

    MarkerCommandCreated --(CommandRecordMarker, on_command_record_marker) --> ResultNotified;

    MarkerCommandCreatedReplaying --(CommandRecordMarker) --> ResultNotifiedReplaying;

    Replaying --(Schedule, on_schedule) --> MarkerCommandCreatedReplaying;

    ResultNotified --(MarkerRecorded, on_marker_recorded) --> MarkerCommandRecorded;

    ResultNotifiedReplaying --(NonMatchingEvent, on_non_matching_event) --> SkippedNotified;
    ResultNotifiedReplaying --(MarkerRecorded, on_marker_recorded) --> MarkerCommandRecorded;
    ResultNotifiedReplaying --(MarkerRecorded, on_marker_recorded) --> SkippedNotified;

    Skipped --(CommandRecordMarker, on_command_record_marker) --> SkippedNotified;
}

#[derive(thiserror::Error, Debug)]
pub(super) enum VersionMachineError {}

pub(super) enum VersionCommand {}

#[derive(Default, Clone)]
pub(super) struct Created {}

impl Created {
    pub(super) fn on_check_execution_state(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct Executing {}

impl Executing {
    pub(super) fn on_schedule(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct MarkerCommandCreated {}

impl MarkerCommandCreated {
    pub(super) fn on_command_record_marker(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct MarkerCommandCreatedReplaying {}

#[derive(Default, Clone)]
pub(super) struct MarkerCommandRecorded {}

#[derive(Default, Clone)]
pub(super) struct Replaying {}

impl Replaying {
    pub(super) fn on_schedule(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct ResultNotified {}

impl ResultNotified {
    pub(super) fn on_marker_recorded(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct ResultNotifiedReplaying {}

impl ResultNotifiedReplaying {
    pub(super) fn on_non_matching_event(self) -> VersionMachineTransition {
        unimplemented!()
    }
    pub(super) fn on_marker_recorded(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

impl From<MarkerCommandCreatedReplaying> for ResultNotifiedReplaying {
    fn from(_: MarkerCommandCreatedReplaying) -> Self {
        Self::default()
    }
}

#[derive(Default, Clone)]
pub(super) struct Skipped {}

impl Skipped {
    pub(super) fn on_command_record_marker(self) -> VersionMachineTransition {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub(super) struct SkippedNotified {}