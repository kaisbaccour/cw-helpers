use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    /// The address of party A.
    pub party_a: Addr,
    /// The address of party B.
    pub party_b: Addr,
    /// The source funds from  party A
    pub party_a_funds: Vec<Coin>,
    /// The source funds from  party B
    pub party_b_funds: Vec<Coin>,
}

pub const CONFIG: Item<Config> = Item::new("config");
