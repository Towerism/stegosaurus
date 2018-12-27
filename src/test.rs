use std::path::PathBuf;

pub fn embed(cover: &str, input: &str, output: &str, passfile: &str) {
    let argv = vec![
        "stegosaurus".to_string(),
        "embed".to_string(),
        "--cover".to_string(),
        resource(cover),
        "--input".to_string(),
        resource(input),
        "--output".to_string(),
        resource(output),
        "--passfile".to_string(),
        resource(passfile),
    ];
    ::core::run(argv);
}

pub fn extract(cover: &str, output: &str, passfile: &str) {
    let argv = vec![
        "stegosaurus".to_string(),
        "extract".to_string(),
        "--cover".to_string(),
        resource(cover),
        "--output".to_string(),
        resource(output),
        "--passfile".to_string(),
        resource(passfile),
    ];
    ::core::run(argv);
}

pub fn resource(path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    let str = d.into_os_string();
    let str = str.to_string_lossy();
    let mut result = "".to_string();
    result.push_str(&str);
    result.push_str(&format!("/{}", path));
    result
}
