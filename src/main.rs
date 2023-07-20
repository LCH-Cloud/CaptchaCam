use chrono::prelude::*;
use clap::Parser;
use nokhwa::query;
use nokhwa::utils::ApiBackend;
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use std::thread;
use std::time::Instant;
use tracing::{debug, error, info, warn};

/// Provides a command line interface for controlling the camera.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A command line interface for controlling the camera."
)]
struct Args {
    /// Sets the delay (in seconds) before the camera starts taking pictures.
    #[structopt(long, default_value = "0")]
    delay: u64,

    /// Sets the filename (as a string) of the image.
    #[structopt(long, default_value = "image")]
    filename: String,

    /// Specifies the camera name.
    #[structopt(long)]
    devname: Option<String>,

    /// Sets the camera number.
    #[structopt(long, default_value = "0")]
    devnum: u32,

    /// Lists all available cameras. [default: false]
    #[structopt(long, default_value = "false")]
    devlist: bool,

    /// Enables or disables the output. [default: false]
    #[structopt(long, default_value = "false")]
    quiet: bool,

    /// Enables debug mode. [default: false]
    #[structopt(long, default_value = "false")]
    debug: bool,

    /// Updates the program. [default: false]
    #[structopt(long, default_value = "false")]
    update: bool,

    /// Updates the program. [default: false]
    #[structopt(long, default_value = "false")]
    selfupdate: bool,
}

fn main() {
    dotenvy::dotenv().ok();

    // We start by capturing the start time for overall performance monitoring.
    let total_start = Instant::now();
    // Parse the command line arguments into an Args struct.
    let args = Args::parse();

    let delay = args.delay;
    let filename = args.filename;
    let devname = args.devname;
    let devnum = args.devnum;
    let devlist = args.devlist;
    let quiet = args.quiet;
    let debug = args.debug;
    let selfupdate = args.selfupdate;
    let mut index = CameraIndex::Index(devnum);

    // Initialize the tracing subscriber for logging, unless it's turned off by 'quiet' or 'debug' flag.
    if !quiet && !debug {
        tracing_subscriber::fmt::fmt().with_ansi(false).init();
    }

    if debug {
        tracing_subscriber::fmt::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_ansi(false)
            .init()
    }

    if args.update {
        if let Err(e) = update() {
            error!("Failed to update: {:?}", e);
        }
        return;
    }

    if selfupdate {
        if let Err(e) = update() {
            error!("Failed to update: {:?}", e);
        }
    }

    // Query available cameras and handle any error.
    let cameras = match query(ApiBackend::Auto) {
        Ok(cameras) => cameras,
        Err(e) => {
            error!("Failed to query cameras: {:?}", e);
            return;
        }
    };

    // If devlist is enabled, list all available cameras.

    if devlist {
        for camera in cameras {
            println!(
                "devname: {} | devnum: {}",
                camera.human_name(),
                camera.index()
            );
        }
        return;
    }

    // If devname is provided, search for the camera in the list of available cameras.
    if let Some(devname) = devname {
        for camera in cameras {
            if camera.human_name() == devname {
                index = camera.index().clone();
            }
        }
    }

    // Sleep for the specified delay time.
    thread::sleep(std::time::Duration::new(delay, 0));

    // Create a request for the highest possible resolution CameraFormat that can be decoded to RGB.
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // Create a new camera instance and handle any error.
    let mut camera = match Camera::new(index, requested) {
        Ok(camera) => camera,
        Err(e) => {
            error!("Failed to create a new camera instance: {:?}", e);
            return;
        }
    };

    let capture_start = Instant::now();

    // Capture a frame from the camera and handle any error.
    let frame = match camera.frame() {
        Ok(frame) => frame,
        Err(e) => {
            error!("Failed to capture frame: {:?}", e);
            return;
        }
    };
    let capture_end = Instant::now();
    let capture_duration = capture_end.duration_since(capture_start);

    debug!(
        "Captured Single Frame of {}, Capture Time: {:?}",
        frame.buffer().len(),
        capture_duration
    );

    let decode_start = Instant::now();

    // Decode the captured frame into an ImageBuffer and handle any error.
    let decoded = match frame.decode_image::<RgbFormat>() {
        Ok(decoded) => decoded,
        Err(e) => {
            error!("Failed to decode image: {:?}", e);
            return;
        }
    };
    let decode_end = Instant::now();
    let decode_duration = decode_end.duration_since(decode_start);

    debug!(
        "Decoded Frame of {}, Decode Time: {:?}",
        decoded.len(),
        decode_duration
    );

    let save_start = Instant::now();

    // Save the decoded image to a file and handle any error.
    if let Err(e) = decoded.save(format!("{}.png", filename)) {
        error!("Failed to save image: {:?}", e);
    };

    let save_end = Instant::now();
    let save_duration = save_end.duration_since(save_start);

    debug!("Image Saved, Save Time: {:?}", save_duration);

    let total_end = Instant::now();
    let total_duration = total_end.duration_since(total_start);

    // Print out the total time for performance monitoring.
    debug!("Total Time: {:?}", total_duration);

    // Print the camera info, image resolution and filename.
    info!("Camera: {}", camera.info().human_name());
    info!("Resolution: {}x{}", decoded.width(), decoded.height());
    info!("Image: {}", format!("{}.png", filename));
}

fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("LCH-Cloud")
        .repo_name("CaptchaCam")
        .bin_name("github")
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true)
        .auth_token(std::env::var("DOWNLOAD_AUTH_TOKEN").unwrap().as_str())
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    println!("Updated to Version: {}!", status.version());
    Ok(())
}
