use bitflags::Flags;
use napi::{Env, JsString};
use std::path::Path;

pub(crate) fn path_to_js_string(env: &Env, p: &Path) -> crate::Result<JsString> {
  #[cfg(unix)]
  {
    use std::borrow::Borrow;

    let path = p.to_string_lossy();
    let str = env.create_string(path.borrow())?;
    Ok(str)
  }
  #[cfg(windows)]
  {
    use std::os::windows::ffi::OsStrExt;
    let path_buf = p.as_os_str().encode_wide().collect::<Vec<u16>>();
    let str = env.create_string_utf16(path_buf.as_slice())?;
    Ok(str)
  }
}

pub(crate) fn bitflags_contain<T: Flags>(source: T, target: T) -> bool {
  source.contains(target)
}
