use rust_decimal::prelude::*;

pub enum OrderType {
    Market,
    Limit,
    LimitMaker,
    StopLossLimit,
    TrailingStopLimit,
    TakeProfitLimit,
}
impl Default for OrderType {
    fn default() -> OrderType {
        OrderType::Limit
    }
}

pub enum OrderStatus {
    New,
    Open,
    Closed,
}

pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Side::Buy
    }
}

pub enum MarketType {
    Future,
    Spot,
}

pub struct Position {
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub fee: Decimal,
}

pub struct Order {
    pub id: u64,
    pub group_id: Option<u32>,
    pub client_id: String,
    pub symbol: String,
    pub creation_timestamp: u64,
    pub update_timestamp: u64,
    pub price: Option<Decimal>,
    pub orig_qty: Option<Decimal>,
    pub executed_qty: Option<Decimal>,
    pub type_name: OrderType,
    pub status: OrderStatus,
    pub side: Side,
}
