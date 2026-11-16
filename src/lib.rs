//! SigmaOS Component
//!
//! Placeholder library

#![forbid(unsafe_code)]

pub mod config;

pub use config::*;

/// Main component struct
pub struct Component {
    pub config: Config,
}

impl Component {
    /// Create new component
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load_default()?;
        Ok(Self { config })
    }

    /// Initialize component
    pub async fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
