use std::sync::Arc;
use eyre::{Result, Report};
use solana_sdk::{
  account::Account,
  signature::{Keypair, Signer, Signature},
  transaction::Transaction,
  message::Message,
  instruction::{Instruction},
  commitment_config::CommitmentConfig,
  borsh::{try_from_slice_unchecked}, pubkey::Pubkey,
};
use solana_client::{
  nonblocking::rpc_client,
  rpc_response::Response,
};

pub struct RpcClient {
  rpc_client: Arc<rpc_client::RpcClient>,
  payer: Option<Arc<Keypair>>,
}

impl RpcClient {
  pub fn new(solana_rpc: String, operator_priv_key: Option<String>) -> Self {
    let rpc_client =  Arc::new(rpc_client::RpcClient::new_with_commitment(solana_rpc, CommitmentConfig::confirmed()));
    let payer = operator_priv_key.map(|key| Arc::new(Keypair::from_base58_string(&key)));

    Self {
      rpc_client,
      payer,
    }
  }

  pub fn payer_key(&self) -> Option<Pubkey> {
    self.payer.as_ref().map(|payer | payer.pubkey())
  }

  pub async fn send_tx(&self, ix: Instruction) -> Result<Signature> {
    if self.payer.is_none() {
      return Err(Report::msg("please provide the payer account"))?
    };

    let payer = self.payer.as_ref().unwrap();
    let latest_blockhash = self.rpc_client.get_latest_blockhash().await.unwrap();
    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&**payer], message, latest_blockhash);
    
    self.rpc_client.send_and_confirm_transaction(&tx).await.map_err(Into::<_>::into)
  }

  pub async fn get_multiple_accounts(&self, pubkeys: &[Pubkey]) -> Result<Vec<Option<Account>>> {
    self.rpc_client.get_multiple_accounts(pubkeys).await.map_err(Into::<_>::into)
  }

  pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
    self.rpc_client.get_account(pubkey).await.map_err(Into::<_>::into)
  }

  pub fn deser_anchor_account_data<T>(mut data: Vec<u8>) -> Result<T>
  where 
    T: borsh::BorshDeserialize
  {
    // Note! the first 8 bytes represent the Anchor account discriminator so we need to get rid of it first
    data.drain(0..8);
    try_from_slice_unchecked::<T>(&data).map_err(Into::<_>::into)
  }

  pub async fn get_anchor_account_data<T>(&self, account: &Pubkey) -> Result<T>
  where 
    T: borsh::BorshDeserialize
  {
    let account = self.get_account(account).await?;
    Self::deser_anchor_account_data(account.data)
  }

  pub async fn get_account_with_commitment(&self, pubkey: &Pubkey, commitment: CommitmentConfig) -> Result<Response<Option<Account>>> {
    self.rpc_client.get_account_with_commitment(pubkey, commitment)
    .await
    .map_err(Into::<_>::into)
  }

  pub async fn account_exists(&self, pubkey: &Pubkey, commitment: CommitmentConfig) -> Result<bool> {
    let response = self.get_account_with_commitment(pubkey, commitment).await?;

    if response.value.is_none() {
      return Ok(false)
    }

    Ok(true)
  }
}
