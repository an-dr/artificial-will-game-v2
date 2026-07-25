use super::*;
use serde_json::json;

#[test]
fn dist_group_accepts_the_two_shipped_directories() {
    for group in ["core", "levels"] {
        let package = json!({
            "metadata": {
                "artificial-will": {
                    "dist-group": group
                }
            }
        });
        assert_eq!(dist_group(&package, "example"), Ok(group));
    }
}

#[test]
fn dist_group_rejects_missing_metadata() {
    assert_eq!(
        dist_group(&json!({}), "example"),
        Err("extension package example has no dist-group".to_owned())
    );
}

#[test]
fn dist_group_rejects_unknown_directories() {
    let package = json!({
        "metadata": {
            "artificial-will": {
                "dist-group": "extensions"
            }
        }
    });
    assert_eq!(
        dist_group(&package, "example"),
        Err("extension package example has unknown dist-group extensions".to_owned())
    );
}
