pub fn get_executable_dir() -> std::path::PathBuf {
    let current_exe = std::env::current_exe();
    current_exe
        .expect("failed to get executable path")
        .parent()
        .expect("failed to get executable directory")
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
