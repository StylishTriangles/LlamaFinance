use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;

use crate::msg::PriceResponse;

pub const ITEMS: Map<&str, PriceResponse> = Map::new("items");
pub const ADMIN: Item<Addr> = Item::new("admin");
