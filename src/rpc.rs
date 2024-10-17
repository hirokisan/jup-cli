use {
    solana_client::rpc_client::RpcClient, solana_sdk::pubkey::Pubkey,
    spl_token::solana_program::program_pack::Pack, spl_token::state::Mint, std::error::Error,
};

const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub fn get_rpc_client() -> RpcClient {
    let url = RPC_URL.to_string();
    RpcClient::new(url)
}

pub fn get_mint_decimals(client: &RpcClient, mint: &Pubkey) -> Result<u8, Box<dyn Error>> {
    let account = client.get_account(mint)?;
    let mint_account = Mint::unpack_from_slice(&account.data)?;

    Ok(mint_account.decimals)
}
