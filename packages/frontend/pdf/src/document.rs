use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;
use pdfium_render::prelude::PdfDocument as PdfDocumentInner;

use crate::{PdfManager, PdfPages};

#[napi]
pub struct PdfDocument {
  inner: SharedReference<PdfManager, PdfDocumentInner<'static>>,
}

#[napi]
impl PdfDocument {
  pub fn new(inner: SharedReference<PdfManager, PdfDocumentInner<'_>>) -> Self {
    Self { inner }
  }

  pub fn get_ref(&self) -> &PdfDocumentInner<'static> {
    &*self.inner
  }

  #[napi]
  pub fn pages(&self, reference: Reference<PdfDocument>, env: Env) -> Result<PdfPages> {
    PdfPages::new(reference, env)
  }

  #[napi]
  pub fn clone(&self, env: Env) -> Result<Self> {
    Ok(Self {
      inner: self.inner.clone(env)?,
    })
  }
}
