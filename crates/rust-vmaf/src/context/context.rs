use rust_vmaf_sys::vmaf_close;

use super::Initialize;

pub(crate) struct VmafContextPtr(pub *mut rust_vmaf_sys::VmafContext);

pub struct VmafContext<Stage = Initialize> {
    pub(crate) context_ptr: VmafContextPtr,
    pub(crate) stage: Stage,
}

impl Drop for VmafContextPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                // TODO: Should we report the return code somehow?
                vmaf_close(self.0);
            }
        }
    }
}
