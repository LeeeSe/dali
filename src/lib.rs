use std::{
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

use regex_lite::Regex;
use rfd::FileDialog;

pub fn choose_video_file() -> Option<PathBuf> {
    let path = FileDialog::new()
        .add_filter("视频文件", &["mp4", "webm", "wav"])
        .pick_file();
    path
}

pub fn compress_video(input: &Path, video_type: &str, tx: &mpsc::Sender<f32>) {
    let input_str = input.as_os_str().to_str().unwrap().to_owned();
    let input_folder = input.parent().unwrap();
    let output_name = format!("output.{}", video_type);
    let output_str = input_folder.join(Path::new(&output_name));
    let tx = tx.clone();
    let mut total_sec = 0;
    thread::spawn(move || {
        let mut progress_command = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(&input_str)
            .arg("-vcodec")
            .arg("h264")
            .arg("-progress")
            .arg("pipe:1")
            .arg(output_str)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut meta_command = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_str)
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(stderr) = meta_command.stderr.take() {
            let reader = io::BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("Duration") {
                        total_sec = get_duration_sec(line.to_owned().as_str());
                        println!("total_sec: {}", total_sec);
                    }
                }
            }
        }

        if let Some(stderr) = progress_command.stdout.take() {
            let reader = io::BufReader::new(stderr);
            for line in reader.lines() {
                println!("当前_line: {:?}", line);
                if let Ok(line) = line {
                    if line.contains("out_time") {
                        update_progress(&line, total_sec, tx.clone());
                    } else if line.contains("progress=end") {
                        tx.send(1.0).unwrap();
                    }
                }
            }
        }
    });
}

fn update_progress(line: &str, total_sec: u32, tx: std::sync::mpsc::Sender<f32>) {
    // 提取当前out_time
    let pattern = r"out_time=(\d{2}):(\d{2}):(\d{2})\.(\d{2})";
    let regex = Regex::new(pattern);
    if let Some(captures) = regex.unwrap().captures(line) {
        let hour = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let min = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let sec = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let out_time_sec = hour * 3600 + min * 60 + sec;
        // 计算进度
        let progress = out_time_sec as f32 / total_sec as f32;
        match tx.send(progress) {
            Ok(_) => {}
            Err(e) => {
                println!("发送进度失败: {}", e);
            }
        }
    }
}

fn get_duration_sec(line: &str) -> u32 {
    // 利用正则提取视频秒时长
    let pattern = r"Duration: (\d{2}):(\d{2}):(\d{2})\.(\d{2})";
    let regex = Regex::new(pattern);
    if let Some(captures) = regex.unwrap().captures(line) {
        let hour = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let min = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let sec = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let duration_sec = hour * 3600 + min * 60 + sec;
        return duration_sec;
    }
    return 0;
}
