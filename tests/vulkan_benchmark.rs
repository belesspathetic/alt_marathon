use std::{
    process::{Command, Stdio},
    thread,
    time::Duration,
};

#[test]
#[ignore]
fn vulkan_benchmark() {
    let mut command = Command::new("ffmpeg")
        .args(&[
            "-init_hw_device",
            "vulkan=vk:0",
            "-hwaccel",
            "vulkan",
            "-hwaccel_output_format",
            "vulkan",
            "-f",
            "lavfi",
            "-i",
            "testsrc",
            "-f",
            "null",
            "-",
            "-benchmark",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ffmpeg");

    thread::sleep(Duration::from_secs(5));

    command.kill().expect("Failed to kill ffmpeg");
}
