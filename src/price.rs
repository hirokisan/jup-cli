use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use std::collections::HashMap;
use std::error::Error;

pub struct PriceClient;

const V2_PRICE_API: &str = "https://price.jup.ag/v6/price";

#[derive(Serialize, Deserialize, Debug)]
struct PriceResponse {
    data: HashMap<String, PriceData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceData {
    price: f64,
}

impl PriceClient {
    pub fn new() -> Self {
        PriceClient
    }
    pub async fn get_prices(
        self,
        mints: &[Pubkey],
    ) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        let ids = mints
            .iter()
            .map(|mint| mint.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let url = format!("{V2_PRICE_API}?ids={ids}");
        let res = reqwest::get(url).await?.json::<PriceResponse>().await?;
        let result = res
            .data
            .iter()
            .map(|(k, v)| (k.clone(), v.price))
            .collect::<HashMap<String, f64>>();
        Ok(result)
    }
}
