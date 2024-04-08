use std::{process::{Command, Stdio}, thread, time::Duration};

#[test]
#[ignore]
fn h264_qsv_benchmark() {
    let mut command = Command::new("ffmpeg")
        .args(&[
            "-benchmark",
            "-f",
            "lavfi",
            "-i",
            "testsrc",
            "-c:v",
            "h264_qsv",
            "-f",
            "null",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ffmpeg");

    thread::sleep(Duration::from_secs(5));

    command.kill().expect("Failed to kill ffmpeg");
}
