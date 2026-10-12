use chrono::Utc;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    #[pyo3(get)]
    pub index: usize,
    #[pyo3(get)]
    pub timestamp: i64,
    #[pyo3(get)]
    pub operation: String,
    #[pyo3(get)]
    pub payload: String,
    #[pyo3(get)]
    pub prev_hash: String,
    #[pyo3(get)]
    pub hash: String,
}

#[pymethods]
impl LedgerEntry {
    #[new]
    pub fn new(
        index: usize,
        timestamp: i64,
        operation: String,
        payload: String,
        prev_hash: String,
        hash: String,
    ) -> Self {
        Self {
            index,
            timestamp,
            operation,
            payload,
            prev_hash,
            hash,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    entries: Vec<LedgerEntry>,
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl Ledger {
    #[new]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, operation: String, payload: String) -> String {
        let index = self.entries.len();
        let timestamp = Utc::now().timestamp_millis();
        let prev_hash = self
            .entries
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| "0".repeat(64));

        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(operation.as_bytes());
        hasher.update(payload.as_bytes());
        hasher.update(prev_hash.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        let entry = LedgerEntry {
            index,
            timestamp,
            operation,
            payload,
            prev_hash,
            hash: hash.clone(),
        };

        self.entries.push(entry);
        hash
    }

    pub fn record_transition(&mut self, state_hash: String) -> String {
        self.append("TRANSITION".to_string(), state_hash)
    }

    pub fn latest_hash(&self) -> String {
        self.entries
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| "0".repeat(64))
    }

    pub fn verify_integrity(&self) -> bool {
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.index != i {
                return false;
            }

            let expected_prev_hash = if i == 0 {
                "0".repeat(64)
            } else {
                self.entries[i - 1].hash.clone()
            };

            if entry.prev_hash != expected_prev_hash {
                return false;
            }

            let mut hasher = Sha256::new();
            hasher.update(entry.index.to_be_bytes());
            hasher.update(entry.timestamp.to_be_bytes());
            hasher.update(entry.operation.as_bytes());
            hasher.update(entry.payload.as_bytes());
            hasher.update(entry.prev_hash.as_bytes());
            let computed_hash = format!("{:x}", hasher.finalize());

            if entry.hash != computed_hash {
                return false;
            }
        }
        true
    }

    pub fn get_entries(&self) -> Vec<LedgerEntry> {
        self.entries.clone()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.entries)
            .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
    }
}
