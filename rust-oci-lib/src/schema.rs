use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct SchemaV2 {
    pub manifests: Vec<Manifest>,
}

/// Manifests in oci image look like this:
/// {
///    "mimeType": "application/vnd.oci.image.manifest.v1+json",
///    "digest":"sha256:c09f03518e0e67e243b947bf992b8c807723aa56d61408bd3c2630c4784d6a14",
///    "size":349,
///    "annotations": {
///         "org.opencontainers.image.ref.name":"10"
///     }
/// }
///
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub media_type: String,
    pub digest: String,
    pub size: u64,
    pub annotations: Value, // can contain arbitrary key, value pairs
}
