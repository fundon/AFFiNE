use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;
use pdfium_render::prelude::{PdfDocument as PdfDocumentInner, Pdfium};

use crate::PdfDocument;

struct PdfViewerInner {
  engine: Pdfium,
}

impl PdfViewerInner {
  fn new() -> Result<Self> {
    Self::bind_to_library("./".to_string())
  }

  fn bind_to_library(path: String) -> Result<Self> {
    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&path))
      .or_else(|_| Pdfium::bind_to_system_library())
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let engine = Pdfium::new(bindings);

    Ok(Self { engine })
  }

  fn open<'a>(&'a self, bytes: Vec<u8>, password: Option<&str>) -> Result<PdfDocumentInner<'a>> {
    self
      .engine
      .load_pdf_from_byte_vec(bytes, password)
      .map_err(|e| Error::from_reason(e.to_string()))
  }
}

#[napi]
pub struct PdfViewer {
  inner: PdfViewerInner,
  docs: Arc<RwLock<HashMap<String, PdfDocument>>>,
}

#[napi]
impl PdfViewer {
  fn get_ref(&self) -> &PdfViewerInner {
    &self.inner
  }

  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      inner: PdfViewerInner::new()?,
      docs: Default::default(),
    })
  }

  #[napi]
  pub fn bind_to_library(path: String) -> Result<Self> {
    Ok(Self {
      inner: PdfViewerInner::bind_to_library(path)?,
      docs: Default::default(),
    })
  }

  #[napi]
  pub fn open_with_id(&self, env: Env, id: String) -> Option<PdfDocument> {
    let docs = self.docs.read().ok()?;

    docs.get(&id).and_then(|doc| doc.clone(env).ok())
  }

  #[napi]
  pub fn open(
    &self,
    reference: Reference<PdfViewer>,
    env: Env,
    id: String,
    bytes: Buffer,
    password: Option<&str>,
  ) -> Option<PdfDocument> {
    let result = self.open_with_id(env, id.clone());

    if result.is_some() {
      return result;
    }

    let inner = reference
      .share_with(env, |viewer| {
        viewer.get_ref().open(bytes.to_vec(), password)
      })
      .ok()?;

    let doc = PdfDocument::new(inner);

    let mut docs = self.docs.write().ok()?;

    docs.insert(id, doc.clone(env).ok()?);

    Some(doc)
  }

  #[napi]
  pub fn close(&self, id: String) -> Result<bool> {
    let mut docs = self
      .docs
      .write()
      .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(docs.remove(&id).is_some())
  }
}
