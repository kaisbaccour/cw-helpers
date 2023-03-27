use thiserror::Error;

use cosmwasm_std::StdError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("The deposited funds are not equal to the agreed funds")]
    FundsNotEqualToConfig,

    #[error("Funds haven't been deposited")]
    FundsHaventBeenDeposited,

    #[error("Funds already deposited")]
    FundsAlreadyDeposited,

    #[error("Invalid Address")]
    InvalidAddress,

    #[error("At least one party's funds are missing. Both parties funds should be in the contract for the exchange to be processed")]
    AtLeastOnePartyFundsAreMissing,

    #[error("Unothorised! Make sure you call this contract from party A or party B ")]
    Unauthorized,
}
