use super::*;

#[derive(Debug, Parser)]
pub(crate) struct DupUtxo {
  #[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  fee_rate: FeeRate,
  #[arg(long, help = "Number of UTXO")]
  pub(crate) count: u64,
  #[arg(long, help = "Unspent amount for each UTXO")]
  pub(crate) unspent_each: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
  pub txid: Txid,
}

impl DupUtxo {
  pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
    let mut output = Vec::new();
    for _ in 0..self.count {
      output.push(TxOut {
        script_pubkey: wallet.get_change_address()?.script_pubkey(),
        value: self.unspent_each
      });
    }

    let unfunded_transaction = Transaction {
      version: 2,
      lock_time: LockTime::ZERO,
      input: Vec::new(),
      output,
    };

    wallet.lock_non_cardinal_outputs()?;
    let bitcoin_client = wallet.bitcoin_client();

    let unsigned_transaction =
      fund_raw_transaction(bitcoin_client, self.fee_rate, &unfunded_transaction)?;

    let signed_transaction = bitcoin_client
      .sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
      .hex;


    let transaction = bitcoin_client.send_raw_transaction(&signed_transaction)?;

    Ok(Some(Box::new(Output {
      txid: transaction,
    })))
  }
}
