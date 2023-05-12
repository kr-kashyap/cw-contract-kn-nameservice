use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub purchase_price: Option<Coin>,
    pub transfer_price: Option<Coin>,
    pub cw20_contract: String,
}

#[cw_serde]
pub struct NameRecord {
    pub owner: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const NAME_RESOLVER: Map<&[u8], NameRecord> = Map::new("name_resolver");
