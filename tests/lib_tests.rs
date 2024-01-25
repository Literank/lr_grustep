use tempfile::tempdir;

use lr_grustep::{grep, grep_count, grep_recursive, GrepOptions, MatchItem, MatchResult};

#[test]
fn test_grep() {
    let pattern = "fn grep";
    let file_content = "This is a test line with fn grep in it.";
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let file_path = temp_dir.path().join("test_file.txt");

    std::fs::write(&file_path, file_content).expect("Failed to write to temporary file");

    let options = GrepOptions {
        ignore_case: false,
        invert_match: false,
    };

    let result = grep(pattern, &file_path, &options).unwrap();
    let matched_lines = result.get(file_path.to_string_lossy().as_ref()).unwrap();
    assert_eq!(matched_lines.len(), 1);

    // Check the content of the matched line
    assert_eq!(matched_lines[0].line, file_content.trim());
}

#[test]
fn test_grep_count() {
    let mut result = MatchResult::new();
    let item = MatchItem {
        line_number: 1,
        line: String::from("Test line"),
    };
    result.insert(String::from("test_file.txt"), vec![item]);

    let count = grep_count(&result);
    assert_eq!(count, 1);
}

#[test]
fn test_grep_recursive() {
    let pattern = "TODO";
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let nested_dir = temp_dir.path().join("nested");
    std::fs::create_dir(&nested_dir).expect("Failed to create nested directory");

    let file_content = "TODO: Implement this feature.";
    let file_path = temp_dir.path().join("test_file.txt");
    let nested_file_path = nested_dir.join("nested_file.txt");

    std::fs::write(&file_path, file_content).expect("Failed to write to temporary file");
    std::fs::write(&nested_file_path, file_content).expect("Failed to write to nested file");

    let options = GrepOptions {
        ignore_case: false,
        invert_match: false,
    };

    let result = grep_recursive(pattern, temp_dir.path(), &options).unwrap();
    let matched_lines = result.get(file_path.to_string_lossy().as_ref()).unwrap();
    assert_eq!(matched_lines.len(), 1);

    // Check the content of the matched line
    assert_eq!(matched_lines[0].line, file_content.trim());

    // Similarly, check the content for the nested file
    let nested_matched_lines = result
        .get(nested_file_path.to_string_lossy().as_ref())
        .unwrap();
    assert_eq!(nested_matched_lines.len(), 1);
    assert_eq!(nested_matched_lines[0].line, file_content.trim());
}
