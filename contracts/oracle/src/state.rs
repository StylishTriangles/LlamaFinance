use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Uint128};


pub const PRICES: Map<&str, Uint128> = Map::new("prices");
pub const ADMIN: Item<Addr> = Item::new("admin");
