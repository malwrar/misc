/// Webcam(s) -> OpenCV -> misc outputs (webui, filesystem, etc)
use std::io::prelude::*;
use opencv::{
    core::{Mat},
    imgcodecs::{imencode, IMWRITE_JPEG_QUALITY},
    prelude::*,
    types::{VectorOfi32, VectorOfu8},
    videoio::{VideoCapture, CAP_ANY},
    Result,
};

fn main() -> Result<()> {
    println!("Hello, world!");

    // TODO: set up device listener thread (loop through devices, add to device map)
    // TODO: loop and grab devices, then dump frames as a bundle (concurrently? try timing it)
    // TODO: process frame bundle, attaching any interesting metadata
    // TODO: send frame bundle to consumers (ceph recorder, web video feed & metadata websocket users, kafka, etc)
    // TODO: implement barebones frontend to display frames and chart metadata


    let mut cameras: Vec<VideoCapture> = Vec::new();

    const MAX_CAMERAS: i32 = 10;  // Arbitrary, since I can't find a logical limit.
    for i in 1..MAX_CAMERAS {
        let camera = VideoCapture::new(i, CAP_ANY)?;
        if !camera.is_opened()? {
            continue;
        }

        //if !camera.is_opened() {
        //    break
        //}

        cameras.push(camera);
    }

    println!("There are {} available camera(s).", cameras.len());

    for (i, camera) in cameras.iter_mut().enumerate() {
        let mut frame = Mat::default();
        camera.read(&mut frame)?;

        let mut encode_options = VectorOfi32::new();
        encode_options.push(IMWRITE_JPEG_QUALITY);
        encode_options.push(90);

        let mut jpeg_data = VectorOfu8::new();
        imencode(".jpeg", &mut frame, &mut jpeg_data, &encode_options)?;

        let mut file = std::fs::File::create(format!("camera{}.jpeg", i)).unwrap();
        file.write_all(jpeg_data.as_slice()).unwrap();
    }

    Ok(())
}
