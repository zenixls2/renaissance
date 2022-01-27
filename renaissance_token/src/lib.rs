use rust_decimal::prelude::*;

pub struct Fee {
    pub amount_quote: Option<Decimal>,
    pub amount_base: Option<Decimal>,
}

pub struct Token {
    pub symbol: String,
    pub decimal: u32,
    pub name: String,
    pub address: Option<String>,
    pub price: Option<Decimal>,
    pub chain_id: Option<u32>,
    pub cex: Option<String>,
    pub fee: Fee,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
