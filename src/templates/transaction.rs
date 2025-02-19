use bitcoin::PackedLockTime;
use crate::inscription::InscriptionEvent;
use super::*;

#[derive(Boilerplate)]
pub(crate) struct TransactionHtml {
  blockhash: Option<BlockHash>,
  confirmations: Option<u32>,
  chain: Chain,
  etching: Option<SpacedDune>,
  inscription_count: u32,
  transaction: Transaction,
  txid: Txid,
  drc20_events: Vec<Drc20Event>,
  inscription_events: Vec<InscriptionEvent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TransactionJson {
  pub(crate) blockhash: Option<BlockHash>,
  pub(crate) confirmations: Option<u32>,
  pub(crate) chain: Chain,
  pub(crate) etching: Option<SpacedDune>,
  pub(crate) inscription_count: u32,
  pub(crate) transaction: TransactionWithAddress,
  pub(crate) txid: Txid,
  pub(crate) drc20_events: Vec<Drc20Event>,
  pub(crate) inscription_events: Vec<InscriptionEvent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionWithAddress {
  pub version: i32,
  pub lock_time: PackedLockTime,
  pub input: Vec<TxIn>,
  pub output: Vec<TxOutWithAddress>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TxOutWithAddress {
  pub value: u64,
  pub script_pubkey: Script,
  pub address: Option<Address>,
}

impl TransactionHtml {
  pub(crate) fn new(
    transaction: Transaction,
    blockhash: Option<BlockHash>,
    confirmations: Option<u32>,
    inscription_count: u32,
    chain: Chain,
    etching: Option<SpacedDune>,
    drc20_events: Vec<Drc20Event>,
    inscription_events: Vec<InscriptionEvent>,
  ) -> Self {
    Self {
      txid: transaction.txid(),
      blockhash,
      confirmations,
      chain,
      etching,
      inscription_count,
      transaction,
      drc20_events,
      inscription_events,
    }
  }
}

impl PageContent for TransactionHtml {
  fn title(&self) -> String {
    format!("Transaction {}", self.txid)
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    bitcoin::{blockdata::script, PackedLockTime, TxOut},
  };

  #[test]
  fn html() {
    let transaction = Transaction {
      version: 0,
      lock_time: PackedLockTime(0),
      input: vec![TxIn {
        sequence: Default::default(),
        previous_output: Default::default(),
        script_sig: Default::default(),
        witness: Default::default(),
      }],
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    let txid = transaction.txid();

    pretty_assert_eq!(
      TransactionHtml::new(transaction, None, None, Chain::Mainnet).to_string(),
      format!(
        "
        <h1>Transaction <span class=monospace>{txid}</span></h1>
        <h2>1 Input</h2>
        <ul>
          <li><a class=monospace href=/output/0000000000000000000000000000000000000000000000000000000000000000:4294967295>0000000000000000000000000000000000000000000000000000000000000000:4294967295</a></li>
        </ul>
        <h2>2 Outputs</h2>
        <ul class=monospace>
          <li>
            <a href=/output/{txid}:0 class=monospace>
              {txid}:0
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_0</dd>
            </dl>
          </li>
          <li>
            <a href=/output/{txid}:1 class=monospace>
              {txid}:1
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_PUSHNUM_1</dd>
            </dl>
          </li>
        </ul>
      "
      )
      .unindent()
    );
  }

  #[test]
  fn with_blockhash() {
    let transaction = Transaction {
      version: 0,
      lock_time: PackedLockTime(0),
      input: Vec::new(),
      output: vec![
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(0).into_script(),
        },
        TxOut {
          value: 50 * COIN_VALUE,
          script_pubkey: script::Builder::new().push_int(1).into_script(),
        },
      ],
    };

    assert_regex_match!(
      TransactionHtml::new(transaction, Some(blockhash(0)), None, Chain::Mainnet),
      "
        <h1>Transaction <span class=monospace>[[:xdigit:]]{64}</span></h1>
        <dl>
          <dt>block</dt>
          <dd><a href=/block/0{64} class=monospace>0{64}</a></dd>
        </dl>
        .*
      "
      .unindent()
    );
  }
}
