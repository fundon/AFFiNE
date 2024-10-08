use napi::bindgen_prelude::*;
use napi_derive::napi;
use pdfium_render::prelude::PdfPage as PdfPageInner;

use crate::PdfPages;

#[napi]
pub struct PdfPage {
  inner: SharedReference<PdfPages, PdfPageInner<'static>>,
}

#[napi]
impl PdfPage {
  pub fn new(inner: SharedReference<PdfPages, PdfPageInner<'static>>) -> Self {
    Self { inner }
  }

  #[napi]
  pub fn text(&self) -> Result<String> {
    self
      .inner
      .text()
      .map(|t| t.all())
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn render(&self) {
    // self.inner.render(width, height, rotation)
  }
}
