pub struct PriceCalculator<'a> {
    current_timestamp_ms: u64,
    last_timestamp_ms: u64,
    currency_list: Vec<&'a str>,
    last_prices: Vec<f32>,
}

impl<'a> PriceCalculator<'a> {
    pub fn new(
        current_timestamp_ms: u64,
        last_timestamp_ms: u64,
        currency_list: Vec<&'a str>,
        last_prices: Vec<f32>,
    ) -> Self {
        Self {
            current_timestamp_ms,
            last_timestamp_ms,
            currency_list,
            last_prices,
        }
    }
}
