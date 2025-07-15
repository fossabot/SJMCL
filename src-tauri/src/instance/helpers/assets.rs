use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_http::reqwest;

use crate::{
  error::SJMCLResult,
  instance::{helpers::client_json::McClientInfo, models::misc::InstanceError},
  launcher_config::models::GameDirectory,
  resource::{
    helpers::misc::get_download_api,
    models::{ResourceType, SourceType},
  },
  tasks::{download::DownloadParam, PTaskParam},
};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct AssetIndex {
  pub objects: HashMap<String, AssetIndexItem>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct AssetIndexItem {
  pub hash: String,
  pub size: i64,
}

pub async fn download_assets(
  app: &tauri::AppHandle,
  directory: &GameDirectory,
  version_info: &McClientInfo,
  source: SourceType,
) -> SJMCLResult<Vec<PTaskParam>> {
  let client = app.state::<reqwest::Client>();
  // Download asset index
  let asset_index_raw = client
    .get(version_info.asset_index.url.clone())
    .send()
    .await
    .map_err(|_| InstanceError::NetworkError)?
    .json::<serde_json::Value>()
    .await
    .map_err(|_| InstanceError::AssetIndexParseError)?;

  let asset_index_path = directory.dir.join("assets/indexes");

  fs::create_dir_all(&asset_index_path).map_err(|_| InstanceError::FolderCreationFailed)?;
  fs::write(
    asset_index_path.join(format!("{}.json", version_info.asset_index.id)),
    asset_index_raw.to_string(),
  )
  .map_err(|_| InstanceError::FileCreationFailed)?;

  // Download assets (use task)
  let asset_index: AssetIndex =
    serde_json::from_value(asset_index_raw).map_err(|_| InstanceError::AssetIndexParseError)?;
  let assets_download_api = get_download_api(source, ResourceType::Assets)?;

  Ok(
    asset_index
      .objects
      .iter()
      .filter_map(|(_, item)| {
        let path = format!("{}/{}", &item.hash[..2], item.hash.clone());
        let dest = directory.dir.join(format!("assets/objects/{}", path));

        if dest.exists() {
          None
        } else {
          Some(PTaskParam::Download(DownloadParam {
            src: assets_download_api
              .join(&path)
              .map_err(|_| InstanceError::ClientJsonParseError)
              .unwrap(),
            dest,
            filename: None,
            sha1: Some(item.hash.clone()),
          }))
        }
      })
      .collect::<Vec<_>>(),
  )
}
