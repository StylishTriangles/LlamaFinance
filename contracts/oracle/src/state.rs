use cw_storage_plus::Map;
use cosmwasm_std::Uint128;


pub const PRICES: Map<&str, Uint128> = Map::new("prices");
