extern crate stegosaurus;

use std::path::PathBuf;
use std::io::Read;
use std::fs;

#[test]
fn img_and_lsb_preserve_message() {
    embed(
        "test.bmp",
        "test_input.txt",
        "test_stega.bmp",
        "test_passphrase"
    );
    extract(
        "test_stega.bmp",
        "test_input_extracted.txt",
        "test_passphrase"
    );

    let mut input_file = fs::File::open(resource("test_input.txt")).unwrap();
    let mut extracted_input_file = fs::File::open(resource("test_input_extracted.txt")).unwrap();
    let mut input = Vec::new();
    let mut extracted_input = Vec::new();
    input_file.read_to_end(&mut input).unwrap();
    extracted_input_file.read_to_end(&mut extracted_input).unwrap();
    assert_eq!(input, extracted_input);
}

fn embed(cover: &str, input: &str, output: &str, passfile: &str) {
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
        resource(passfile)
    ];
    stegosaurus::core::run(argv);
}

fn extract(cover: &str, output: &str, passfile: &str) {
    let argv = vec![
        "stegosaurus".to_string(),
        "extract".to_string(),
        "--cover".to_string(),
        resource(cover),
        "--output".to_string(),
        resource(output),
        "--passfile".to_string(),
        resource(passfile)
    ];
    stegosaurus::core::run(argv);
}

fn resource(path: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    let str = d.into_os_string();
    let str = str.to_string_lossy();
    let mut result = "".to_string();
    result.push_str(&str);
    result.push_str(&format!("/{}", path));
    result
}
