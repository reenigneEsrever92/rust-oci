use std::fs::read_to_string;

#[cfg(test)]

#[test]
fn test_read_index() {
    let contents = read_to_string("resources/tests/index.json").unwrap();

    let schema: rust_oci_lib::schema::SchemaV2 = serde_json::from_str(contents.as_str()).unwrap();

    assert_eq!(schema.manifests[0].digest, "sha256:c09f03518e0e67e243b947bf992b8c807723aa56d61408bd3c2630c4784d6a14");
    
}