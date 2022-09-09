use napi::bindgen_prelude::*;
use napi_derive::napi;

use swc_common::BytePos;
use swc_css::{
  codegen::{writer::basic::BasicCssWriter, CodeGenerator, CodegenConfig, Emit},
  minifier::minify as swc_minify,
  parser::parse_str,
};

#[cfg(all(
  not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[napi]
pub fn minify(input: Buffer) -> Result<String> {
  let input = String::from_utf8_lossy(input.as_ref());
  let mut errors = Vec::new();
  let mut ast = parse_str(
    input.as_ref(),
    BytePos(0),
    BytePos(input.len() as u32),
    Default::default(),
    &mut errors,
  )
  .map_err(|err| Error::new(Status::InvalidArg, format!("{:?}", err)))?;
  swc_minify(&mut ast, Default::default());
  let mut output = String::with_capacity(input.len());
  let writer = BasicCssWriter::new(&mut output, None, Default::default());
  let mut codegen = CodeGenerator::new(writer, CodegenConfig { minify: true });
  codegen
    .emit(&ast)
    .map_err(|err| Error::new(Status::InvalidArg, format!("{:?}", err)))?;
  Ok(output)
}
