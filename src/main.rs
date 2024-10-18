use crate::rpc::{get_mint_decimals, get_rpc_client, RPC_URL};
use crate::swap::get_swap_client;

use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
};

use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Signer};
use solana_sdk::transaction::VersionedTransaction;

use clap::*;
use env_logger::Env;
use log::info;
use std::error::Error;

mod rpc;
mod swap;

#[derive(Parser)]
#[command(name = "jup-cli")]
#[command(about = "A Jupiter CLI that enables swap, etc.")]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "swap")]
    Swap {
        #[arg(long, required = true)]
        key_pair_path: String,
        #[arg(long, required = true)]
        mint_from: Pubkey,
        #[arg(long, required = true)]
        mint_to: Pubkey,
        #[arg(long, required = true)]
        amount: i32,
        #[arg(long, default_value_t = 50, help = "slippage in basis points")] // 0.5%
        slippage: u16,
        #[arg(long, default_value_t = RPC_URL.to_string())]
        rpc_url: String,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    match cli.command {
        SubCommand::Swap {
            amount,
            slippage,
            mint_from,
            mint_to,
            key_pair_path,
            rpc_url,
            dry_run,
        } => {
            let swap_client = get_swap_client();
            let rpc_client = get_rpc_client(&rpc_url);

            let from_keypair = match read_keypair_file(key_pair_path) {
                Ok(pair) => pair,
                Err(err) => return Err(err),
            };
            let from_pubkey = Signer::pubkey(&from_keypair);

            let mint_from: Pubkey = pubkey!(mint_from);
            let mint_to: Pubkey = pubkey!(mint_to);

            let mint_from_decimals = match get_mint_decimals(&rpc_client, &mint_from) {
                Ok(decimals) => decimals,
                Err(err) => return Err(err),
            };
            let mint_to_decimals = match get_mint_decimals(&rpc_client, &mint_to) {
                Ok(decimals) => decimals,
                Err(err) => return Err(err),
            };

            let quote_request = QuoteRequest {
                amount: amount as u64 * 10_u64.pow(mint_from_decimals.into()),
                input_mint: mint_from,
                output_mint: mint_to,
                slippage_bps: slippage,
                ..QuoteRequest::default()
            };

            let quote_response = swap_client.quote(&quote_request).await?;
            info!(
                "expected amount: {:?}",
                (quote_response.out_amount as f64) / (10_u64.pow(mint_to_decimals.into()) as f64)
            );

            if !dry_run {
                let swap_response = swap_client
                    .swap(&SwapRequest {
                        user_public_key: from_pubkey,
                        quote_response: quote_response.clone(),
                        config: TransactionConfig::default(),
                    })
                    .await?;

                let versioned_transaction: VersionedTransaction =
                    bincode::deserialize(&swap_response.swap_transaction)?;

                let signed_versioned_transaction =
                    VersionedTransaction::try_new(versioned_transaction.message, &[&from_keypair])?;

                match rpc_client.send_and_confirm_transaction(&signed_versioned_transaction) {
                    Ok(sig) => loop {
                        if let Ok(confirmed) = rpc_client.confirm_transaction(&sig) {
                            if confirmed {
                                info!("Transaction: {}", sig);
                                break;
                            }
                        }
                    },
                    Err(err) => return Err(Box::new(err)),
                };
            }
        }
    };

    Ok(())
}
