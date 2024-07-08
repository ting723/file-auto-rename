use std::env::{self};

use file::{is_video_file, loop_dir};
use rename::FileInfo;

mod file;
mod rename;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        println!("请输入文件需要重命名的目录");
        return Ok(());
    }
    let dir_path = args[0].as_str();
    let files = loop_dir(dir_path)?;
    for f in files {
        if is_video_file(&f)? {
            let file_info = FileInfo::new(f);
            file_info.rename_based_on_md5()?;
        }
    }
    Ok(())
}
