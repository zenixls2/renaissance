use crate::common::*;
use renaissance_token::Token;

pub struct OrderBook {
    pub token1: Token,
    pub token2: Token,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}
