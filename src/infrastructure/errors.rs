use crate::infrastructure::InfrastructureInformation;
use serde::{Deserialize, Serialize};
// use sn_data_types::PublicKey;
// use std::{
//     fmt::Debug
//     result,
// };
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Error {
    /// Target section key provided with message is out of date. Current PK Set is provided in error
    #[error("Target section's public key is outdated. New information has been provided.")]
    TargetSectionInfoOutdated(InfrastructureInformation),

    /// Target section is undergoing churn, a new key set will be agreed upon shortly
    #[error("DKG is in process. New key set will be agreed upon shortly.")]
    DkgInProgress,

    /// Target section is unrecognized
    #[error("Target section key provided is unrecognized")]
    UnrecognizedSectionKey,
}