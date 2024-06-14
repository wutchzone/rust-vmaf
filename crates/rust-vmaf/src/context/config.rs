#[derive(Clone, Copy, Debug, Default)]
pub struct ContextConfig {
    /// How many threads can be used to run feature extractors concurrently. If set to the `None`,
    /// then it means all threads.
    pub threads: Option<u32>,

    /// Compute scores only every N frames. Note that setting an even value for N can lead to
    /// inaccurate results. For more detail, see [#1214](https://github.com/Netflix/vmaf/issues/1214).
    pub subsample: u32,
}
