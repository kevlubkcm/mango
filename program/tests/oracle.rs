#![cfg(feature = "test-bpf")]

use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use flux_aggregator::processor;


#[tokio::test]
async fn test_success() {
    let test_key = Keypair::new();
    let mut test = ProgramTest::new(
        "flux_aggregator",
        Pubkey::new_unique(),
        processor!(flux_aggregator::processor::Processor.process_instruction),
    );

    // limit to track compute unit increase
    test.set_bpf_compute_max_units(35_000);

    // let user_accounts_owner = Keypair::new();
    // let usdc_mint = add_usdc_mint(&mut test);
    // let lending_market = add_lending_market(&mut test, usdc_mint.pubkey);

    // let usdc_reserve = add_reserve(
    //     &mut test,
    //     &user_accounts_owner,
    //     &lending_market,
    //     AddReserveArgs {
    //         user_liquidity_amount: 100 * FRACTIONAL_TO_USDC,
    //         liquidity_amount: 10_000 * FRACTIONAL_TO_USDC,
    //         liquidity_mint_decimals: usdc_mint.decimals,
    //         liquidity_mint_pubkey: usdc_mint.pubkey,
    //         config: TEST_RESERVE_CONFIG,
    //         ..AddReserveArgs::default()
    //     },
    // );

    // let (mut banks_client, payer, _recent_blockhash) = test.start().await;

    // lending_market
    //     .deposit(
    //         &mut banks_client,
    //         &user_accounts_owner,
    //         &payer,
    //         &usdc_reserve,
    //         100 * FRACTIONAL_TO_USDC,
    //     )
    //     .await;
}
