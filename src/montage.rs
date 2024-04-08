use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub async fn scale(
    original_video_path: PathBuf,
    temp_dir_path: &Path,
    top_text: String,
    bottom_text: String,
    font_size: String,
) -> anyhow::Result<PathBuf> {
    let path = temp_dir_path.join("scaled.mp4");
    let text_added_path = temp_dir_path.join("texted.mp4");

    let output = temp_dir_path.join("video.mp4");
    // Scale
    let _ = Command::new("ffmpeg")
        .arg("-i")
        .arg(&original_video_path)
        .arg("-vf")
        .arg("scale=1080:608")
        .arg("-map")
        .arg("0:v")
        .arg("-map")
        .arg("0:a")
        .arg("-c:v")
        .arg("h264_qsv")
        .arg("-c:a")
        .arg("copy")
        .arg(&path)
        .spawn()?
        .wait()
        .expect("Failed to start ffmpeg process");

    let font_path = "./font.ttf";

    // Add text
    let _ = Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .arg("-vf")
        .arg(format!("pad=width=1080:height=1080:x=(ow-iw)/2:y=(oh-ih)/2:color=black,drawtext=text='{top_text}':x=(w-text_w)/2:y=(h-text_h)/8:fontsize={font_size}:fontcolor=yellow:fontfile={font_path},drawtext=text='{bottom_text}':x=(w-text_w)/2:y=(h*7)/8:fontsize={font_size}:fontcolor=yellow:fontfile={font_path}"))
        .arg("-map")
        .arg("0:v")
        .arg("-map")
        .arg("0:a")
        .arg("-c:v")
        .arg("h264_qsv")
        .arg("-c:a")
        .arg("copy")
        .arg(&text_added_path)
        .spawn()?
        .wait()
        .expect("Failed to start ffmpeg process");

    // Speed up video
    let _ = Command::new("ffmpeg")
        .arg("-i")
        .arg(&text_added_path)
        .arg("-filter_complex")
        .arg("[0:v]setpts=0.8*PTS[v];[0:a]atempo=1.25[a]")
        .arg("-map")
        .arg("[v]")
        .arg("-map")
        .arg("[a]")
        .arg("-c:v")
        .arg("h264_qsv")
        .arg("-c:a")
        .arg("aac")
        .arg(&output)
        .spawn()?
        .wait()
        .expect("Failed to start ffmpeg process");

    Ok(output)
}
