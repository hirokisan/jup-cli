use jupiter_swap_api_client::JupiterSwapApiClient;

const V6_SWAP_API: &str = "https://quote-api.jup.ag/v6";

pub fn get_swap_api_client() -> JupiterSwapApiClient {
    JupiterSwapApiClient::new(V6_SWAP_API.to_string())
}
