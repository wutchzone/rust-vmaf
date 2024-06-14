use std::{io::Error, mem::ManuallyDrop, ops::Range, ptr};

use rust_vmaf_sys::{
    vmaf_read_pictures,
    vmaf_score_pooled,
    VmafPoolingMethod_VMAF_POOL_METHOD_HARMONIC_MEAN,
    VmafPoolingMethod_VMAF_POOL_METHOD_MAX,
    VmafPoolingMethod_VMAF_POOL_METHOD_MEAN,
    VmafPoolingMethod_VMAF_POOL_METHOD_MIN,
};

use super::VmafContext;
use crate::{model::VmafModel, picture::Picture};

#[derive(Copy, Clone, Debug)]
pub enum PollMethod {
    Min,
    Max,
    Mean,
    HarmonicMean,
}

pub struct Process {
    pub(crate) index: u32,
}

impl VmafContext<Process> {
    pub fn read_pictures(&mut self, pictures: Option<(Picture, Picture)>) -> Result<(), Error> {
        match pictures {
            Some((reference_picture, target_picture)) => {
                // VmafContext will take ownership of both VmafPictures.
                match unsafe {
                    vmaf_read_pictures(
                        self.context_ptr.0,
                        &mut ManuallyDrop::new(reference_picture).picture,
                        &mut ManuallyDrop::new(target_picture).picture,
                        self.stage.index,
                    )
                } {
                    0 => {
                        self.stage.index += 1;
                        Ok(())
                    },
                    other => Err(Error::from_raw_os_error(other)),
                }
            },
            None => {
                match unsafe {
                    vmaf_read_pictures(self.context_ptr.0, ptr::null_mut(), ptr::null_mut(), 0)
                } {
                    0 => Ok(()),
                    other => Err(Error::from_raw_os_error(other)),
                }
            },
        }
    }

    /// Calculates VMAF score for the registered model using `poll_method` with given
    /// `picture_range`, if picture range is `None`, then the VMAF score is calculated for all
    /// submitted pictures.
    pub fn score_pooled(
        &mut self,
        model: &VmafModel,
        poll_method: PollMethod,
        picture_range: Option<Range<u32>>,
    ) -> Result<f64, Error> {
        let mut result = 0f64;
        let (start, end) =
            picture_range.map_or((0u32, self.stage.index - 1), |item| (item.start, item.end));

        match unsafe {
            vmaf_score_pooled(
                self.context_ptr.0,
                model.model_ptr,
                match poll_method {
                    PollMethod::Min => VmafPoolingMethod_VMAF_POOL_METHOD_MIN,
                    PollMethod::Max => VmafPoolingMethod_VMAF_POOL_METHOD_MAX,
                    PollMethod::Mean => VmafPoolingMethod_VMAF_POOL_METHOD_MEAN,
                    PollMethod::HarmonicMean => VmafPoolingMethod_VMAF_POOL_METHOD_HARMONIC_MEAN,
                },
                &mut result,
                start,
                end,
            )
        } {
            0 => Ok(result),
            other => Err(Error::from_raw_os_error(other)),
        }
    }
}
