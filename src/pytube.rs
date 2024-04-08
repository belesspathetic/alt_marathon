use inline_python::{python, Context};

pub fn call_pytube(url: String) -> anyhow::Result<(String, String, String)> {
    println!("PROCESS: Calling pytube");
    let c: Context = python! {
        from pytube import YouTube

        yt = YouTube('url)

        stream = yt.streams.filter(progressive=True).get_highest_resolution().url
        thumb = yt.thumbnail_url
        title = yt.title
        (stream, thumb, title)

    };

    let stream = c.get("stream");
    let thumb: String = c.get("thumb");
    let title: String = c.get("title");
    Ok((stream, thumb, title))
}
