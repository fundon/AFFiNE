use napi_derive::napi;
use pdfium_render::prelude::PdfPageRenderRotation;

#[napi]
pub enum Rotation {
  Zero,
  One,
  Two,
  Three,
}

impl Into<PdfPageRenderRotation> for Rotation {
  fn into(self) -> PdfPageRenderRotation {
    match self {
      Self::Zero => PdfPageRenderRotation::None,
      Self::One => PdfPageRenderRotation::Degrees90,
      Self::Two => PdfPageRenderRotation::Degrees180,
      Self::Three => PdfPageRenderRotation::Degrees270,
    }
  }
}
