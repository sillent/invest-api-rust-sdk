use std::{fs::read_dir, io, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PROTO_DIR: &str = "proto/investment-api";
    let files = files_in_dir(PROTO_DIR)?; // Сохраняем Vec<String>
    let file_refs: Vec<&str> = files.iter().map(String::as_str).collect(); // Создаём ссылки
    let file_slice: &[&str] = &file_refs;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_protos(file_slice, &[PROTO_DIR])?;

    Ok(())
}

fn files_in_dir<P: AsRef<Path>>(dir: P) -> io::Result<Vec<String>> {
    let files: Vec<String> = read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| entry.path().to_str().map(String::from))
        .collect();
    Ok(files)
}
