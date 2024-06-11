#![allow(
    dead_code,
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types
)]
include!(concat!(env!("OUT_DIR"), "/vmaf.rs"));

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[test]
    fn sanity() {
        let conf = VmafConfiguration {
            log_level: VmafLogLevel_VMAF_LOG_LEVEL_INFO,
            n_threads: 0,
            n_subsample: 0,
            cpumask: 0,
            gpumask: 0,
        };

        let mut context: *mut VmafContext = ptr::null_mut();
        assert_eq!(unsafe { vmaf_init(&mut context, conf) }, 0);
        assert_eq!(unsafe { vmaf_close(context) }, 0);
    }
}
