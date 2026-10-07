//! The MS-MPEG4 decoder reports the container's picture size, which every
//! frame has (the bitstream carries none, so it cannot change
//! mid-stream): an odd size, whose planes are macroblock-padded beyond it.
//!
//! `decoded_format/v3_33x17.bin` is one FFmpeg `msmpeg4` (v3) I-frame of
//! `testsrc`.

use oxideav_core::{CodecId, CodecParameters, CodecRegistry, Frame, Packet, PixelFormat, TimeBase};

#[test]
fn reports_the_container_size_every_frame_has() {
    let mut reg = CodecRegistry::new();
    oxideav_msmpeg4::register_codecs(&mut reg);
    let mut params = CodecParameters::video(CodecId::new(oxideav_msmpeg4::CODEC_ID_V3));
    params.width = Some(33);
    params.height = Some(17);
    let mut dec = reg.first_decoder(&params).expect("decoder");
    dec.send_packet(&Packet::new(
        0,
        TimeBase::new(1, 25),
        include_bytes!("decoded_format/v3_33x17.bin").to_vec(),
    ))
    .expect("send");
    let frame = match dec.receive_frame() {
        Ok(Frame::Video(frame)) => frame,
        other => panic!("{other:?}"),
    };
    assert_eq!(dec.output_video_dimensions(), Some((33, 17)));
    assert_eq!(dec.output_pixel_format(), Some(PixelFormat::Yuv420P));
    let planes = frame.image_planes();
    assert_eq!(planes.len(), 3);
    for (i, plane) in planes.iter().enumerate() {
        let (w, h) = if i == 0 { (33, 17) } else { (17, 9) };
        assert!(plane.stride >= w, "plane {i} stride");
        assert!(
            plane.data.len() >= plane.stride * (h - 1) + w,
            "plane {i} rows"
        );
    }
}
