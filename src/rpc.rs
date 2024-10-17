use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;

pub fn get_mint_decimals(mint: &Pubkey) -> u8 {
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new(url);
    let account = client.get_account(mint).unwrap();
    let mint_account = Mint::unpack_from_slice(&account.data).unwrap();
    mint_account.decimals
}
