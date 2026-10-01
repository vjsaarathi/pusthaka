//! Hardware-independent reading engine.
//!
//! The reader layer may use storage and graphics *traits*, but it must never import a
//! concrete board, ESP peripheral, RTOS type, or vendor controller API.

pub mod document;
pub mod engine;
pub mod txt_engine;
pub mod fonts;
pub mod dictionary;
pub mod layout;
pub mod pagination;
pub mod session;
pub mod shaping;

pub use document::{BookMetadata, DocumentFormat, ReaderPosition};
pub use engine::DocumentEngine;
pub use txt_engine::TxtEngine;
pub use pagination::{Page, PaginationEngine, PaginationState};
pub mod format;
pub mod ingestion;
