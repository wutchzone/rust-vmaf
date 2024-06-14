use std::{io::Error, num::NonZeroUsize, ptr};

use rust_vmaf_sys::{
    vmaf_init,
    vmaf_use_features_from_model,
    VmafConfiguration,
    VmafLogLevel_VMAF_LOG_LEVEL_NONE,
};

use super::{context::VmafContextPtr, ContextConfig, Process, VmafContext};
use crate::model::VmafModel;

pub struct Initialize(());

const UNSIZED_ONE: NonZeroUsize = match NonZeroUsize::new(1) {
    Some(val) => val,
    None => panic!("Cannot fail."),
};

impl VmafContext<Initialize> {
    pub fn new(config: ContextConfig) -> Result<VmafContext<Initialize>, Error> {
        let mut this: VmafContext<Initialize> = VmafContext {
            context_ptr: VmafContextPtr(ptr::null_mut()),
            stage: Initialize(()),
        };

        match unsafe {
            vmaf_init(
                &mut this.context_ptr.0,
                VmafConfiguration {
                    log_level: VmafLogLevel_VMAF_LOG_LEVEL_NONE,
                    n_threads: config.threads.unwrap_or(
                        std::thread::available_parallelism()
                            .unwrap_or(UNSIZED_ONE)
                            .get()
                            .try_into()
                            .unwrap_or(1),
                    ),
                    n_subsample: config.subsample,
                    cpumask: 0,
                    gpumask: 0,
                },
            )
        } {
            0 => Ok(this),
            other => Err(Error::from_raw_os_error(other)),
        }
    }

    pub fn use_features_from_model(self, model: &VmafModel) -> Result<Self, Error> {
        match unsafe { vmaf_use_features_from_model(self.context_ptr.0, model.model_ptr) } {
            0 => Ok(self),
            other => Err(Error::from_raw_os_error(other)),
        }
    }

    #[must_use]
    pub fn start_processing(self) -> VmafContext<Process> {
        VmafContext {
            context_ptr: self.context_ptr,
            stage: Process { index: 0 },
        }
    }
}
