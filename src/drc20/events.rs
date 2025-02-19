use std::fmt;
use std::fmt::Formatter;
use bitcoin::Txid;
use super::*;
use crate::{InscriptionId, SatPoint};
use serde::{Deserialize, Serialize};
use crate::drc20::script_key::ScriptKey;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OperationType {
    Deploy,
    Mint,
    InscribeTransfer,
    Transfer,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Receipt {
    pub inscription_id: InscriptionId,
    pub inscription_number: i64,
    pub old_satpoint: SatPoint,
    pub new_satpoint: SatPoint,
    pub op: OperationType,
    pub from: ScriptKey,
    pub to: ScriptKey,
    pub result: Result<Event, DRC20Error>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Drc20Event {
  pub event: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub txid: Option<Txid>,
  pub vout: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<ScriptKey>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<ScriptKey>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub amt: Option<String>,
  pub tick: Tick,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub supply: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub block: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<usize>,
}

impl Display for Drc20Event {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "Event: {}\nVout: {}\nTick: {}",
      self.event, self.vout, self.tick
    )?;

    if let Some(from) = &self.from {
      write!(f, "\nFrom: {}", from)?;
    }

    if let Some(to) = &self.to {
      write!(f, "\nTo: {}", to)?;
    }

    if let Some(amt) = &self.amt {
      write!(f, "\nAmt: {}", amt)?;
    }

    if let Some(supply) = &self.supply {
      write!(f, "\nSupply: {}", supply)?;
    }

    if let Some(limit) = &self.limit {
      write!(f, "\nLimit: {}", limit)?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Event {
    Deploy(DeployEvent),
    Mint(MintEvent),
    InscribeTransfer(InscribeTransferEvent),
    Transfer(TransferEvent),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeployEvent {
  pub txid: Option<Txid>,
  pub vout: u32,
  pub deployed_by: ScriptKey,
  pub supply: u128,
  pub limit_per_mint: u128,
  pub decimal: u8,
  pub tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MintEvent {
  pub txid: Option<Txid>,
  pub vout: u32,
  pub to: ScriptKey,
  pub tick: Tick,
  pub amount: u128,
  pub msg: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InscribeTransferEvent {
  pub txid: Option<Txid>,
  pub to: ScriptKey,
  pub vout: u32,
  pub tick: Tick,
  pub amount: u128,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransferEvent {
  pub txid: Option<Txid>,
  pub from: ScriptKey,
  pub to: ScriptKey,
  pub vout: u32,
  pub tick: Tick,
  pub amount: u128,
}
