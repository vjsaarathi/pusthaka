/// XTC/XTCH format boundary.
///
/// The architecture intentionally does not invent a byte layout. A real implementation must be
/// driven by the exact XTC format specification and/or corpus fixtures before the parser is completed.
pub struct XtcFormat;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XtcPixelDepth { Mono1, Gray2 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XtcDescriptor {
    pub title: alloc::string::String,
    pub author: alloc::string::String,
    pub page_count: u32,
    pub width: u16,
    pub height: u16,
    pub depth: XtcPixelDepth,
}
