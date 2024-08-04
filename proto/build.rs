use std::{env, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let proto_path = Path::new(&current_dir).join("proto");
    let mut proto_files = vec![];
    for entry in fs::read_dir(&proto_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() && entry.path().extension().unwrap() == "proto" {
            proto_files.push(entry.path().as_os_str().to_os_string())
        }
    }

    tonic_build::configure()
        .out_dir("src")
        .build_client(true)
        .build_server(true)
        .compile(
            proto_files.as_slice(), // proto 文件列表
            &[&proto_path],         // proto 依赖文件所在的根目录
        )?;

    Ok(())
}
