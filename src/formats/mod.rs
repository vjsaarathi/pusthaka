//! Binary and persistent format contracts.

pub mod caches;
pub mod cpfont;
pub mod stardict;
pub mod xtc;

pub use caches::{MetaCacheFormat, ProgressFormat};
pub use cpfont::CpfontFormat;
pub use stardict::StarDictFormat;
pub use xtc::XtcFormat;
