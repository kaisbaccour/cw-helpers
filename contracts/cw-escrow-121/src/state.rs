use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    /// party A.
    pub party_a: Party,
    /// party B.
    pub party_b: Party,
}

#[cw_serde]
pub struct Party {
    /// The address of party
    pub address: Addr,
    /// The source funds from party
    pub funds: Vec<Coin>,
    /// Are funds deposited?
    pub deposited: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PARTY: Item<Config> = Item::new("party");
