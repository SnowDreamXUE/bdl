use std::time::Duration;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use url::Url;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
const MANIFEST: &str = "https://github.com/Yuelioi/bdl/releases/latest/download/latest.json";

// The URL template is transport configuration; the embedded updater public key
// remains the authority for installation, including with a user-supplied mirror.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn mirrored(template: &str, original: &str) -> Result<Url, String> {
    let upstream = Url::parse(original).map_err(|e| e.to_string())?;
    if upstream.host_str() != Some("github.com")
        || !upstream.path().starts_with("/Yuelioi/bdl/releases/")
        || upstream.scheme() != "https"
    {
        return Err("更新清单包含非 BDL 官方下载地址。".into());
    }
    let value = if template.contains("{url}") {
        template.replace("{url}", original)
    } else if template.contains("{path}") {
        template.replace("{path}", original.trim_start_matches("https://"))
    } else {
        format!("{}/{original}", template.trim_end_matches('/'))
    };
    let url = Url::parse(&value).map_err(|_| "加速地址无效。")?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err("加速地址须为不含账号和片段的 HTTPS 地址。".into());
    }
    Ok(url)
}

#[tauri::command]
pub async fn update_distribution() -> Result<serde_json::Value, String> {
    let platform = std::env::consts::OS;
    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        other => other,
    };
    reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?
        .get("https://apps.yuelili.com/api/v1/software/bdl/download-config")
        .query(&[
            ("platform", platform),
            ("architecture", arch),
            ("channel", "stable"),
        ])
        .send()
        .await
        .map_err(|e| format!("软件站连接失败：{e}"))?
        .error_for_status()
        .map_err(|e| format!("软件站检测失败：{e}"))?
        .json()
        .await
        .map_err(|e| format!("软件站响应无效：{e}"))
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tauri::command]
pub async fn update_check_mirror(
    webview: tauri::Webview,
    template: String,
    proxy: Option<String>,
) -> Result<Option<serde_json::Value>, String> {
    use tauri::Manager;
    use tauri_plugin_updater::UpdaterExt;
    let endpoint = mirrored(template.trim(), MANIFEST)?;
    let mut builder = webview
        .updater_builder()
        .timeout(Duration::from_secs(20))
        .endpoints(vec![endpoint])
        .map_err(|e| e.to_string())?;
    if let Some(proxy) = proxy.filter(|p| !p.trim().is_empty()) {
        builder = builder.proxy(Url::parse(&proxy).map_err(|e| e.to_string())?);
    }
    let result = builder
        .build()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?;
    let Some(mut update) = result else {
        return Ok(None);
    };
    update.download_url = mirrored(template.trim(), update.download_url.as_str())?;
    let mut metadata = serde_json::json!({
        "currentVersion": update.current_version, "version": update.version,
        "body": update.body, "rawJson": update.raw_json,
    });
    metadata["rid"] = serde_json::json!(webview.resources_table().add(update));
    Ok(Some(metadata))
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[tauri::command]
pub async fn update_check_mirror() -> Result<Option<serde_json::Value>, String> {
    Err("移动端请下载安装包后手动安装。".into())
}

#[cfg(all(test, not(any(target_os = "android", target_os = "ios"))))]
mod tests {
    use super::*;
    #[test]
    fn mirror_templates_cover_manifest_and_asset() {
        for original in [
            MANIFEST,
            "https://github.com/Yuelioi/bdl/releases/download/v1.0.0/BDL.tar.gz",
        ] {
            assert_eq!(
                mirrored("https://mirror.test/", original).unwrap().as_str(),
                format!("https://mirror.test/{original}")
            );
            assert_eq!(
                mirrored("https://mirror.test/{path}", original)
                    .unwrap()
                    .as_str(),
                format!(
                    "https://mirror.test/{}",
                    original.trim_start_matches("https://")
                )
            );
        }
        assert!(mirrored("http://mirror.test", MANIFEST).is_err());
        assert!(
            mirrored(
                "https://mirror.test",
                "https://github.com/other/app/releases/download/v1/a"
            )
            .is_err()
        );
    }
}
