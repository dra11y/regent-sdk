pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, wherror::Error)]
#[error(debug)]
pub enum Error {
    FailureToFindGroupContent,
    FailureToParseContent(String),
    FailureToRunCommand(String),
    FailureToEstablishConnection(String),
    FailedInitialization(String),
    FailedTcpBinding(String),
    FailedTaskDryRun(String),
    FailedDryRunEvaluation(String),
    FailedToApplyExpectedState(String),
    MissingInitialization(String),
    GroupNotFound,
    MissingGroupsList,
    WorkFlowNotFollowed(String),
    WrongInitialization(String),
    AnyOtherError(String),
    IncoherentExpectedState(String),
    InternalLogicError(String),
    NotConnectedToHost,
    ConnectionLevel(String),
}
