use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

// index key
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Integer(i) => {
                0u8.hash(state);
                i.hash(state);
            }
            Value::Float(f) => {
                1u8.hash(state);

                let bits = if f.is_nan() {
                    0x7ff8000000000000
                } else if *f == 0.0 {
                    0
                } else {
                    f.to_bits()
                };

                bits.hash(state);
            }
            Value::String(s) => {
                2u8.hash(state);
                s.hash(state);
            }
            Value::Boolean(b) => {
                3u8.hash(state);
                b.hash(state);
            }
            Value::Null => {
                4u8.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowVersion {
    pub id: u64,
    pub row: Row,
    pub t_created: u64,
    pub t_deleted: Option<u64>,
}

pub(crate) fn visible(version: &RowVersion, tx_id: u64, active_txs: &HashSet<u64>) -> bool {
    // current transaction created
    if version.t_created == tx_id {
        if let Some(deleted) = version.t_deleted {
            //deleted by current transaction
            if deleted == tx_id {
                return false;
            }
        }
        return true;
    }
    // uncommitted created by transacation
    if active_txs.contains(&version.t_created) {
        return false;
    }
    // featured created by transaction
    if version.t_created > tx_id {
        return false;
    }

    if let Some(deleted) = version.t_deleted {
        if deleted == tx_id {
            return false;
        }

        if !active_txs.contains(&deleted) && deleted < tx_id {
            return false;
        }
    }

    true
}
