use std::{io::Error, mem::MaybeUninit};

use rust_vmaf_sys::{
    vmaf_picture_alloc,
    vmaf_picture_unref,
    VmafPicture,
    VmafPixelFormat_VMAF_PIX_FMT_YUV420P,
};

pub struct Yuv420Planar<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> Yuv420Planar<'a> {
    pub fn new_with_combined_planes(
        data: &'a [u8],
        width: u32,
        height: u32,
    ) -> Result<Yuv420Planar<'a>, Error> {
        if data.len()
            != (width * height + width * height / 2)
                .try_into()
                .map_err(Error::other)?
        {
            return Err(Error::other("Invalid width and height for the planes."));
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }
}

pub struct Picture {
    pub(crate) picture: VmafPicture,
}

impl TryFrom<Yuv420Planar<'_>> for Picture {
    type Error = Error;

    fn try_from(frame: Yuv420Planar<'_>) -> Result<Self, Self::Error> {
        let mut picture = unsafe { MaybeUninit::<VmafPicture>::zeroed().assume_init() };

        match unsafe {
            vmaf_picture_alloc(
                &mut picture,
                VmafPixelFormat_VMAF_PIX_FMT_YUV420P,
                8,
                frame.width,
                frame.height,
            )
        } {
            0 => { /* ignore */ },
            other => return Err(Error::from_raw_os_error(other)),
        };

        unsafe {
            frame.data.as_ptr().copy_to(
                picture.data[0].cast::<u8>(),
                (picture.w[0] * picture.h[0])
                    .try_into()
                    .map_err(Error::other)?,
            );
            frame
                .data
                .as_ptr()
                .offset(
                    (frame.width * frame.height)
                        .try_into()
                        .map_err(Error::other)?,
                )
                .copy_to(
                    picture.data[1].cast::<u8>(),
                    (picture.w[1] * picture.h[1])
                        .try_into()
                        .map_err(Error::other)?,
                );
            frame
                .data
                .as_ptr()
                .offset(
                    (frame.width * frame.height + (frame.width * frame.height / 4))
                        .try_into()
                        .map_err(Error::other)?,
                )
                .copy_to(
                    #[allow(clippy::as_conversions)]
                    picture.data[2].cast::<u8>(),
                    (picture.w[2] * picture.h[2])
                        .try_into()
                        .map_err(Error::other)?,
                );
        }

        Ok(Picture { picture })
    }
}

impl Drop for Picture {
    fn drop(&mut self) {
        unsafe { vmaf_picture_unref(&mut self.picture) };
    }
}
