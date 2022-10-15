use std::sync::Arc;
use solana_sdk::{
  signature::{Keypair, Signer, Signature},
  transaction::Transaction,
  message::Message,
  instruction::{Instruction},
  commitment_config::CommitmentConfig,
};
use solana_client::{
  client_error::Result as ClientResult,
  nonblocking::rpc_client,
};

pub struct RpcClient {
  rpc_client: Arc<rpc_client::RpcClient>,
  payer: Arc<Keypair>,
}

impl RpcClient {
  pub fn new(solana_rpc: String, operator_priv_key: String) -> Self {
    let rpc_client =  Arc::new(rpc_client::RpcClient::new_with_commitment(solana_rpc, CommitmentConfig::confirmed()));
    let payer = Arc::new(Keypair::from_base58_string(&operator_priv_key));

    Self {
      rpc_client,
      payer,
    }
  }

  pub async fn send_tx(&self, ix: Instruction) -> ClientResult<Signature> {
    let latest_blockhash = self.rpc_client.get_latest_blockhash().await.unwrap();
    let message = Message::new(&[ix], Some(&self.payer.pubkey()));
    let tx = Transaction::new(&[&*self.payer], message, latest_blockhash);

    self.rpc_client.send_and_confirm_transaction(&tx).await
  }
}
