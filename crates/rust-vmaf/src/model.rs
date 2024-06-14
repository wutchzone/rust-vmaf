use std::{ffi::CString, io::Error, ptr};

use rust_vmaf_sys::{vmaf_model_destroy, vmaf_model_load};

#[derive(Default)]
pub struct VmafModelConfig {
    pub flags: u64,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct VmafModel {
    pub(crate) model_ptr: *mut rust_vmaf_sys::VmafModel,
}

impl VmafModel {
    /// Loads one of the default built-in models.
    pub fn model_load(version: &str, config: VmafModelConfig) -> Result<Self, Error> {
        let mut this = VmafModel {
            model_ptr: ptr::null_mut(),
        };

        let cstring_name = config
            .name
            .map(|name| CString::new(name).map_err(Error::other))
            .transpose()?;
        let cstring_version = CString::new(version).map_err(Error::other)?;

        match unsafe {
            vmaf_model_load(
                &mut this.model_ptr,
                &mut rust_vmaf_sys::VmafModelConfig {
                    name: if let Some(cstring_name) = cstring_name {
                        cstring_name.as_ptr()
                    } else {
                        ptr::null()
                    },
                    flags: config.flags,
                },
                cstring_version.as_ptr(),
            )
        } {
            0 => Ok(this),
            other => Err(Error::from_raw_os_error(other)),
        }
    }
}

impl Drop for VmafModel {
    fn drop(&mut self) {
        unsafe { vmaf_model_destroy(self.model_ptr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_model() {
        assert!(VmafModel::model_load("vmaf_v0.6.1", VmafModelConfig::default()).is_ok());
    }
}
