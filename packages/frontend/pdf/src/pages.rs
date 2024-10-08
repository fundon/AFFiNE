use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;
use pdfium_render::prelude::PdfPages as PdfPagesInner;

use crate::{PdfDocument, PdfPage};

#[napi]
pub struct PdfPages {
  inner: SharedReference<PdfDocument, &'static PdfPagesInner<'static>>,
}

#[napi]
impl PdfPages {
  pub fn new(reference: Reference<PdfDocument>, env: Env) -> Result<Self> {
    Ok(Self {
      inner: reference.share_with(env, |doc| Ok(doc.get_ref().pages()))?,
    })
  }

  #[napi]
  pub fn len(&self) -> u16 {
    self.inner.len()
  }

  #[napi]
  pub fn get(&self, reference: Reference<PdfPages>, env: Env, index: u16) -> Option<PdfPage> {
    reference
      .share_with(env, |pages| {
        pages
          .inner
          .get(index)
          .map_err(|e| Error::from_reason(e.to_string()))
      })
      .ok()
      .map(PdfPage::new)
  }
}
