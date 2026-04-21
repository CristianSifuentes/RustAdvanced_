//! Language-pattern module: macros, trait objects, smart pointers, and custom errors.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};

/// Step 5: Declarative macro for domain-safe map construction.
#[macro_export]
macro_rules! telemetry {
    ($($key:literal => $value:expr),* $(,)?) => {{
        let mut map = std::collections::BTreeMap::new();
        $(map.insert($key.to_string(), format!("{}", $value));)*
        map
    }};
}

/// Step 6: Error modeling using a dedicated enum.
#[derive(Debug)]
pub enum LabError {
    EmptyExperiment,
    PoisonedLock,
}

impl Display for LabError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyExperiment => write!(f, "experiment had no measurements"),
            Self::PoisonedLock => write!(f, "shared state lock was poisoned"),
        }
    }
}

impl Error for LabError {}

/// Step 7: Trait + dynamic dispatch (`dyn`) for pluggable analyzers.
pub trait Analyzer: Send + Sync {
    fn name(&self) -> &'static str;
    fn score(&self, sample: &[f64]) -> f64;
}

pub struct MeanAnalyzer;

impl Analyzer for MeanAnalyzer {
    fn name(&self) -> &'static str {
        "mean-analyzer"
    }

    fn score(&self, sample: &[f64]) -> f64 {
        if sample.is_empty() {
            0.0
        } else {
            sample.iter().sum::<f64>() / sample.len() as f64
        }
    }
}

/// Step 8: Interior mutability in concurrent context with `Arc<RwLock<_>>`.
#[derive(Clone, Default)]
pub struct SharedNotebook {
    rows: Arc<RwLock<Vec<String>>>,
}

impl SharedNotebook {
    pub fn push(&self, entry: impl Into<String>) -> Result<(), LabError> {
        self.rows
            .write()
            .map_err(|_| LabError::PoisonedLock)?
            .push(entry.into());
        Ok(())
    }

    pub fn rows(&self) -> Result<Vec<String>, LabError> {
        let rows = self.rows.read().map_err(|_| LabError::PoisonedLock)?;
        Ok(rows.clone())
    }
}
