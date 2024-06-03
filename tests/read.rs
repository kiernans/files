use files::read::read_file;

#[test]
fn file_has_contents() {
    let contents = read_file("foo.txt").unwrap();

    assert!(!contents.is_empty());
}
