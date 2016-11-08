#[test]
fn test_decode() {
    let args = vec!["urlq", "-d", "jib=foo%20bar"];
    let app = super::app();

    let mut out = Vec::new();
    super::run(app.get_matches_from(args), &mut out);

    assert_eq!(out.as_slice(), b"jib=foo bar\n");
}

#[test]
fn test_encode() {
    let args = vec!["urlq", "-e", "jib=foo bar"];
    let app = super::app();

    let mut out = Vec::new();
    super::run(app.get_matches_from(args), &mut out);

    assert_eq!(out.as_slice(), b"jib%3Dfoo+bar\n");
}
