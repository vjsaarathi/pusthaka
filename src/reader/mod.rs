//! Hardware-independent reading engine.
//!
//! The reader layer may use storage and graphics *traits*, but it must never import a
//! concrete board, ESP peripheral, RTOS type, or vendor controller API.

pub mod document;
pub mod epub;
pub mod fonts;
pub mod dictionary;
pub mod layout;
pub mod pagination;
pub mod session;
pub mod shaping;

pub use document::{BookMetadata, DocumentFormat, ReaderPosition};
pub use pagination::{Page, PaginationEngine, PaginationState};
