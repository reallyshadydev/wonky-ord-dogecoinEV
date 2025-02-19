use bitcoin::Txid;
use super::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Balance {
    pub tick: Tick,
    pub overall_balance: u128,
    pub transferable_balance: u128,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct BalanceWithTxId {
  pub txid: Txid,
  pub balance: Balance,
}

impl Balance {
    pub fn new(tick: &Tick) -> Self {
        Self {
            tick: tick.clone(),
            overall_balance: 0u128,
            transferable_balance: 0u128,
        }
    }
}
