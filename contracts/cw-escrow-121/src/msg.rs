use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    /// The address of party A.
    pub party_a: String,
    /// The address of party B.
    pub party_b: String,
    /// The source funds from  party A
    pub party_a_funds: Vec<Coin>,
    /// The source funds from  party B
    pub party_b_funds: Vec<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Pays by distributing the funds according to what has been instructed by the gateway
    Deposit {},
    Exchange {},
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the config state
    #[returns(ConfigResponse)]
    Config {},
}

// We define a custom struct for each query response
pub type ConfigResponse = Config;
