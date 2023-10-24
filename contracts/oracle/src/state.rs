use cw_storage_plus::Map;


pub const PRICES: Map<&str, u128> = Map::new("prices");
