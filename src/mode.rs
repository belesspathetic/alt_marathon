use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use fb_poster::{utils::Secrets, video::Video};
use tempdir::TempDir;

use crate::{montage::scale, pytube::call_pytube, search::VideoInfo, tx::gen_text};

const BASE: &str = "https://youtube.com";

pub async fn auto(secrets: Secrets) -> anyhow::Result<()> {
    // Set channels
    let channels_path: &Path = Path::new("./channels.txt");
    let mut channels = Vec::new();
    if !channels_path.exists() {
        File::create(&channels_path)?;
        panic!("Write links in channels.txt file")
    } else {
        let file = File::open(&channels_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            channels.push(line?);
        }
    }

    // Set db
    let mut hrefs = Vec::new();
    let db_path: &Path = Path::new("./db.txt");
    if !db_path.exists() {
        File::create(&db_path)?;
        let db = File::open(&db_path)?;
        let reader = BufReader::new(&db);
        for line in reader.lines() {
            hrefs.push(line?);
        }
        drop(db);
    } else {
        let db = File::open(&db_path)?;
        let reader = BufReader::new(&db);
        for line in reader.lines() {
            hrefs.push(line?);
        }
        drop(db);
    }

    for link in channels {
        // Get video info
        let info = VideoInfo::new(&link).await?;

        if hrefs.contains(&info.href) {
            println!("PROCESS: Video alredy published. Skipping...");
            continue;
        }

        let url = format!("{}{}", BASE, info.href);
        let mut _stream = String::new();
        let mut _thumb = String::new();

        // Get stream
        match call_pytube(url) {
            Ok((stream, thumb, _)) => {
                _stream = stream;
                _thumb = thumb;
            }
            Err(_) => continue,
        }

        println!("PROCESS: Downloading video...");

        // Download video
        let temp_dir = TempDir::new("video_")?;
        let temp_dir_path = temp_dir.path();
        let original_video_path = temp_dir.path().join("original_video.mp4");
        let thumb_path = temp_dir.path().join("thumb.png");
        let thumb_path_str = thumb_path.to_string_lossy().to_string();
        let mut file = File::create(&original_video_path)?;
        let mut thumb_file = File::create(&thumb_path)?;

        // Download thumb
        println!("PROCESS: Downloading thumb...");
        let cl = reqwest::Client::new();
        let resp = cl.get(_stream).send().await?.bytes().await?;

        file.write_all(&resp)?;

        // Download thumb
        let resp = cl.get(_thumb).send().await?.bytes().await?;
        thumb_file.write_all(&resp)?;

        let (top_text, bottom_text, font_size) = gen_text(info.title.clone());

        let output = scale(
            original_video_path,
            temp_dir_path,
            top_text,
            bottom_text,
            font_size,
        )
        .await?;

        let path = output.to_string_lossy().to_string();
        Video::new(secrets.clone())
            .local_video(path)
            .with_title(info.title.clone())
            .with_description(info.title.clone())
            .with_thumbnail(thumb_path_str)
            .send()
            .await?;

        temp_dir.close()?;

        let mut db = OpenOptions::new().write(true).append(true).open(db_path)?;
        let write = format!("{}\n", info.href.clone());
        db.write_all(write.as_bytes())?;
        drop(db);
    }

    Ok(())
}

pub async fn manual(secrets: Secrets, input: String) -> anyhow::Result<()> {
    let (stream, thumb, title) = call_pytube(input.trim().to_string())?;
    println!("PROCESS: Downloading video...");

    // Download video
    let temp_dir = TempDir::new("video_")?;
    let temp_dir_path = temp_dir.path();
    let original_video_path = temp_dir.path().join("original_video.mp4");
    let thumb_path = temp_dir.path().join("thumb.png");
    let thumb_path_str = thumb_path.to_string_lossy().to_string();
    let mut file = File::create(&original_video_path)?;
    let mut thumb_file = File::create(&thumb_path)?;

    // Download thumb
    println!("PROCESS: Downloading thumb...");
    let cl = reqwest::Client::new();
    let resp = cl.get(stream).send().await?.bytes().await?;

    file.write_all(&resp)?;

    // Download thumb
    let resp = cl.get(thumb).send().await?.bytes().await?;
    thumb_file.write_all(&resp)?;

    let (top_text, bottom_text, font_size) = gen_text(title.clone());

    let output = scale(
        original_video_path,
        temp_dir_path,
        top_text,
        bottom_text,
        font_size,
    )
    .await?;

    let path = output.to_string_lossy().to_string();
    Video::new(secrets.clone())
        .local_video(path)
        .with_title(title.clone())
        .with_description(title)
        .with_thumbnail(thumb_path_str)
        .send()
        .await?;

    temp_dir.close()?;
    Ok(())
}

pub fn manual_prompt() -> String {
    let mut input = String::new();
    println!("Enter video url:");
    io::stdin().read_line(&mut input).expect("cant read input");

    input.trim().to_string()
}
