use crate::{
    model::{VmafModel, VmafModelConfig},
    picture::{Picture, Yuv420Planar},
    ContextConfig,
    PollMethod,
    VmafContext,
};

const BUNNY_YUV: &[u8] = include_bytes!("../../../assets/bunny.yuv");
const BUNNY_WIDTH: u32 = 1920;
const BUNNY_HEIGHT: u32 = 1080;

#[test]
fn basic_vmaf_calculate_identity() {
    let ctx = VmafContext::new(ContextConfig::default()).unwrap();
    let model = VmafModel::model_load("vmaf_v0.6.1", VmafModelConfig::default()).unwrap();
    let mut ctx = ctx
        .use_features_from_model(&model)
        .unwrap()
        .start_processing();

    let mut frames = BUNNY_YUV.chunks_exact(((BUNNY_WIDTH * BUNNY_HEIGHT) as f64 * 1.5) as _);
    let total_frames = frames.len();
    assert_eq!(total_frames, 60);
    for frame in &mut frames {
        let reference = Picture::try_from(
            Yuv420Planar::new_with_combined_planes(frame, BUNNY_WIDTH, BUNNY_HEIGHT).unwrap(),
        )
        .unwrap();
        let target = Picture::try_from(
            Yuv420Planar::new_with_combined_planes(frame, BUNNY_WIDTH, BUNNY_HEIGHT).unwrap(),
        )
        .unwrap();
        ctx.read_pictures(Some((reference, target))).unwrap();
    }
    ctx.read_pictures(None).unwrap();
    assert!(frames.remainder().is_empty());

    let score = ctx.score_pooled(&model, PollMethod::Mean, None).unwrap();

    println!("VMAF score: {score} with {total_frames} frames.");
    assert!(score > 98f64);
}
