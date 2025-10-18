use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair_file},
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::processed(),
    );

    let program = client.program(my_program::id()).unwrap();

    // TODO: Add test logic here
}
