use std::ffi::OsStr;
use std::path::Path;

pub fn link_ext(filename: &str) -> Option<&str> {
	Path::new(filename).extension().and_then(OsStr::to_str)
}
