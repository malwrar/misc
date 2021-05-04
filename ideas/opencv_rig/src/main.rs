/// Webcam(s) -> OpenCV -> misc outputs (webui, filesystem, etc)
use opencv::{
    prelude::*,
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

    println!("There are {} available cameras.", cameras.len());

    Ok(())
}
