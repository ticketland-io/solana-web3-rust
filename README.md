Solana Web3 Rust
===

Example

```rust
use solana_web3_rust::rpc_client::RpcClient;
use program_artifacts::{
  ix::InstructionData,
  event_registry,
};


fn main() {
  let rpc_client = RpcClient::new(solana_rpc, operator_priv_key);
  let accounts = vec![
    AccountMeta::new(state, false),
    AccountMeta::new(event_nft_authority, false),
    AccountMeta::new(event_nft_metadata, false),
    AccountMeta::new(uri_update_operator, true),
    AccountMeta::new_readonly(metaplex_program, false),
  ];

  let data = event_registry::instruction::UpdateEventNftUriIx {
    event_nft,
    new_uri: msg.new_uri.clone(),
  }.data();

  let ix = Instruction {
    program_id: event_registry::program_id(),
    accounts,
    data,
  };

  let result = rpc_client.send_tx(ix).await;
}
```
