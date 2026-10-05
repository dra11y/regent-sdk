use serde::{Deserialize, Serialize};

use crate::{hosts::managed_host::InternalApiCallOutcome, state::attribute::Remediation};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedHostStatus {
    pub state: HostState,
    pub actions_taken: Option<Vec<Action>>,
}

impl ManagedHostStatus {
    pub fn already_compliant() -> Self {
        Self {
            state: HostState::AlreadyCompliant,
            actions_taken: None,
        }
    }

    pub fn not_compliant(remediations: Vec<Remediation>) -> Self {
        Self {
            state: HostState::NotCompliant,
            actions_taken: Some(
                remediations
                    .into_iter()
                    .map(|remediation| Action::from(remediation, None))
                    .collect(),
            ),
        }
    }

    pub fn reach_compliance_success(actions: Vec<Action>) -> Self {
        Self {
            state: HostState::ReachComplianceSuccess,
            actions_taken: Some(actions),
        }
    }

    pub fn reach_compliance_failed(actions: Vec<Action>) -> Self {
        Self {
            state: HostState::ReachComplianceFailed,
            actions_taken: Some(actions),
        }
    }

    pub fn is_already_compliant(&self) -> bool {
        matches!(self.state, HostState::AlreadyCompliant)
    }

    pub fn is_not_compliant(&self) -> bool {
        matches!(self.state, HostState::NotCompliant)
    }

    pub fn is_reach_compliance_success(&self) -> bool {
        matches!(self.state, HostState::ReachComplianceSuccess)
    }

    pub fn is_reach_compliance_failed(&self) -> bool {
        matches!(self.state, HostState::ReachComplianceFailed)
    }

    pub fn all_remediations(&self) -> Option<Vec<Remediation>> {
        let actions = self.actions_taken.as_ref()?;
        Some(
            actions
                .iter()
                .filter_map(|action| match &action.remediation {
                    Remediation::None(_) => None,
                    remediation => Some(remediation),
                })
                .cloned()
                .collect(),
        )
    }

    pub fn actions_taken(&self) -> Option<Vec<(Remediation, InternalApiCallOutcome)>> {
        let actions = self.actions_taken.as_ref()?;
        Some(
            actions
                .iter()
                .filter_map(|action| {
                    action
                        .action_result
                        .as_ref()
                        .map(|action_result| (action.remediation.clone(), action_result.clone()))
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HostState {
    AlreadyCompliant,
    NotCompliant,
    ReachComplianceFailed,
    ReachComplianceSuccess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    remediation: Remediation,
    // action_result is Option since it can not have been tried yet
    action_result: Option<InternalApiCallOutcome>,
}

impl Action {
    pub fn from(remediation: Remediation, action_result: Option<InternalApiCallOutcome>) -> Self {
        Self {
            remediation,
            action_result,
        }
    }
}

#[derive(Debug)]
pub enum AttributeComplianceAssessment {
    Compliant,
    NonCompliant(Vec<Remediation>),
}

impl AttributeComplianceAssessment {
    pub fn remediations(&self) -> Vec<&Remediation> {
        match self {
            AttributeComplianceAssessment::Compliant => Vec::new(),
            AttributeComplianceAssessment::NonCompliant(remediations) => {
                remediations.iter().collect()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeComplianceResult {
    status: AttributeComplianceStatus,
    details: Option<Vec<(Remediation, InternalApiCallOutcome)>>,
}

impl AttributeComplianceResult {
    pub fn from(
        status: AttributeComplianceStatus,
        details: Option<Vec<(Remediation, InternalApiCallOutcome)>>,
    ) -> Self {
        AttributeComplianceResult { status, details }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttributeComplianceStatus {
    AlreadyCompliant,
    ReachedCompliance,
    NonCompliant,
    FailedReachedCompliance,
}
