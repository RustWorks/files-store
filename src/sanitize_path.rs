use std::path::{Component, Path};

pub fn sanitize_path(path: &str) -> String {
    let path = Path::new(path);
    let components = path.components().collect::<Vec<_>>();
    dbg!(&components);
    let components = components
        .into_iter()
        .filter_map(|i| match i {
            Component::Normal(v) => v.to_os_string().into_string().ok(),
            _ => None,
        })
        .collect::<Vec<String>>();
    components.join("/")
}

#[test]
fn test_sanitize_path() {
    assert_eq!(
        sanitize_path("/test/../toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
    assert_eq!(
        sanitize_path("./test/../toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
    assert_eq!(
        sanitize_path("test/toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
    assert_eq!(
        sanitize_path("/test/toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
    assert_eq!(
        sanitize_path("/test/../../toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
    assert_eq!(
        sanitize_path("/test/.././toto/file.txt"),
        "test/toto/file.txt".to_string()
    );
}
