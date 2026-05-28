#![allow(
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::type_complexity,
    dead_code
)]
//! # Sheaf Persistence Bundle
//!
//! Cross-modal topological data fusion via sheaf-theoretic persistence.
//! Multi-parameter persistence, spectral sequences, and fiber bundles over simplicial complexes.

mod bundle;
mod fusion;
mod multiparam;
mod spectral;

pub use bundle::SheafBundle;
pub use fusion::CrossModalFusion;
pub use multiparam::MultiparameterPersistence;
pub use spectral::SpectralSequence;
