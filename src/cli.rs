use crate::{detector::ObjectDetectionModel, LogLevel};
use clap::Parser;
use std::{path::PathBuf, time::Duration};

#[derive(Parser)]
#[command(author = "Marcus Asteborg", version=env!("CARGO_PKG_VERSION"), about = "TODO")]
pub struct Cli {
    /// The port on which the server will listen for HTTP requests.
    /// Default is 32168. Example usage: --port 1337
    //#[arg(long, default_value_t = 32168)]
    #[arg(long, default_value_t = 32168)]
    pub port: u16,
    /// Duration to wait for a response from the detection worker.
    /// Ideally, this should be similar to the client's timeout setting.
    #[arg(long, default_value = "15", value_parser = parse_duration)]
    pub request_timeout: Duration,
    /// Worker queue size.
    /// The number of requests that can be queued before the server starts rejecting them.
    /// If not set, the server will estimate the queue size based on the timeout and the
    /// inference performance.
    /// This estimation is based on the timeout and the expected number of requests per second.
    #[arg(long)]
    pub worker_queue_size: Option<usize>,
    /// Path to the ONNX model file.
    /// If not specified, the default rt-detrv2 small model will be used
    /// provided it is available in the directory.
    #[clap(long)]
    pub model: Option<PathBuf>,
    /// Type of model type to use.
    /// Default: rt-detrv2
    #[clap(long, default_value_t = ObjectDetectionModel::RtDetrv2)]
    pub object_detection_model_type: ObjectDetectionModel,
    /// Path to the object classes yaml file
    /// Default: coco_classes.yaml which is the 80 standard COCO classes
    #[clap(long)]
    pub object_classes: Option<PathBuf>,
    /// Filters the results to include only the specified labels. Provide labels separated by ','.
    /// Example: --object_filter "person,cup"
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    pub object_filter: Vec<String>,
    /// Sets the level of logging
    #[clap(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
    /// If log_path is set, then stdout logging will be disabled and it will log to file
    #[clap(long)]
    pub log_path: Option<PathBuf>,
    /// Confidence threshold for object detection
    #[clap(long, default_value_t = 0.5)]
    pub confidence_threshold: f32,
    /// Force using CPU for inference
    #[clap(long, default_value_t = false)]
    pub force_cpu: bool,
    /// Intra thread parallelism max is CPU cores - 1.
    /// On Windows, you can use high thread counts, but if you use too high
    /// thread count on Linux, you will get a BIG performance hit.
    /// So default is 1, then you can increase it if you want to test the
    /// performance.
    #[cfg(target_os = "windows")]
    #[clap(long, default_value_t = 192)]
    pub intra_threads: usize,
    #[cfg(not(target_os = "windows"))]
    #[clap(long, default_value_t = 2)]
    pub intra_threads: usize,
    /// Inter thread parallelism max is CPU cores - 1.
    /// On Windows, you can use high thread counts, but if you use too high
    /// thread count on Linux, you will get a BIG performance hit.
    /// So default is 2, then you can increase it if you want to test the
    /// performance.
    #[cfg(target_os = "windows")]
    #[clap(long, default_value_t = 192)]
    pub inter_threads: usize,
    #[cfg(not(target_os = "windows"))]
    #[clap(long, default_value_t = 2)]
    pub inter_threads: usize,
    /// Optional path to save the processed images
    #[clap(long)]
    pub save_image_path: Option<PathBuf>,
    /// Save the reference image (only if save_image_path is provided)
    #[clap(long, default_value_t = false)]
    pub save_ref_image: bool,
    /// GPU Index, best effort to select the correct one if multiple GPUs exist.
    /// Default is 0. The list and actual GPU index might differ.
    /// If the wrong GPU is selected, try changing this value.
    /// Verify through GPU usage to ensure the correct GPU is selected.
    #[clap(long, default_value_t = 0)]
    pub gpu_index: i32,
    /// Save inference stats to file
    #[clap(long)]
    pub save_stats_path: Option<PathBuf>,
    /// Path to download all models to
    /// This command will only download the models to the specified path
    /// and then exit
    #[clap(long)]
    pub download_model_path: Option<PathBuf>,
}

fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    let secs: u64 = s.parse()?;
    Ok(Duration::from_secs(secs))
}
