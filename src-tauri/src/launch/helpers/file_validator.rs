use crate::{
  error::SJMCLResult,
  instance::{
    helpers::client_json::{DownloadsArtifact, FeaturesInfo, IsAllowed, McClientInfo},
    models::misc::{AssetIndex, InstanceError},
  },
  resource::{
    helpers::misc::get_download_api,
    models::{ResourceType, SourceType},
  },
  tasks::{download::DownloadParam, PTaskParam},
  utils::fs::validate_sha1,
};
use futures;
use std::collections::HashSet;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tokio::fs;
use zip::ZipArchive;

use super::misc::get_natives_string;

pub fn get_nonnative_library_artifacts(client_info: &McClientInfo) -> Vec<DownloadsArtifact> {
  let mut artifacts = HashSet::new();
  let feature = FeaturesInfo::default();
  for library in &client_info.libraries {
    if !library.is_allowed(&feature).unwrap_or(false) {
      continue;
    }
    if library.natives.is_some() {
      continue;
    }
    if let Some(ref downloads) = &library.downloads {
      if let Some(ref artifact) = &downloads.artifact {
        artifacts.insert(artifact.clone());
      }
    }
  }
  artifacts.into_iter().collect()
}

pub fn get_native_library_artifacts(client_info: &McClientInfo) -> Vec<DownloadsArtifact> {
  let mut artifacts = HashSet::new();
  let feature = FeaturesInfo::default();

  for library in &client_info.libraries {
    if !library.is_allowed(&feature).unwrap_or(false) {
      continue;
    }
    if let Some(natives) = &library.natives {
      if let Some(native) = get_natives_string(natives) {
        if let Some(ref downloads) = &library.downloads {
          if let Some(ref classifiers) = &downloads.classifiers {
            if let Some(artifact) = classifiers.get(&native) {
              artifacts.insert(artifact.clone());
            }
          }
        }
      } else {
        println!("natives is None");
      }
    }
  }
  artifacts.into_iter().collect()
}

pub async fn get_invalid_library_files(
  source: SourceType,
  library_path: &Path,
  client_info: &McClientInfo,
  check_hash: bool,
) -> SJMCLResult<Vec<PTaskParam>> {
  let mut artifacts = Vec::new();
  artifacts.extend(get_native_library_artifacts(client_info));
  artifacts.extend(get_nonnative_library_artifacts(client_info));

  let library_download_api = get_download_api(source, ResourceType::Libraries)?;

  Ok(
    artifacts
      .iter()
      .filter_map(|artifact| {
        let file_path = library_path.join(&artifact.path);
        if file_path.exists()
          && (!check_hash || validate_sha1(file_path.clone(), artifact.sha1.clone()).is_ok())
        {
          None
        } else {
          Some(PTaskParam::Download(DownloadParam {
            src: library_download_api.join(&artifact.path).ok()?,
            dest: file_path.clone(),
            filename: None,
            sha1: Some(artifact.sha1.clone()),
          }))
        }
      })
      .collect::<Vec<_>>(),
  )
}

pub fn convert_library_name_to_path(name: &String, native: Option<String>) -> SJMCLResult<String> {
  let name_exts = name.split('@').collect::<Vec<_>>();
  let file_ext = if name_exts.len() > 1 {
    name_exts[1].to_string()
  } else {
    "jar".to_string()
  };
  let mut name_split: Vec<String> = name_exts[0].split(":").map(|s| s.to_string()).collect();
  if name_split.len() < 3 {
    println!("name = {}", name);
    Err(InstanceError::ClientJsonParseError.into())
  } else {
    if let Some(n) = native {
      name_split.push(n);
    }
    let pack_name = &name_split[1];
    let pack_version = &name_split[2];
    let jar_file_name = name_split[1..].join("-") + "." + &file_ext;
    let lib_path = name_split[0].replace('.', "/");
    Ok(format!(
      "{}/{}/{}/{}",
      lib_path, pack_name, pack_version, jar_file_name
    ))
  }
}

pub fn get_nonnative_library_paths(
  client_info: &McClientInfo,
  library_path: &Path,
) -> SJMCLResult<Vec<PathBuf>> {
  let mut result = Vec::new();
  let feature = FeaturesInfo::default();
  for library in &client_info.libraries {
    if !library.is_allowed(&feature).unwrap_or(false) {
      continue;
    }
    if library.natives.is_some() {
      continue;
    }
    result.push(library_path.join(convert_library_name_to_path(&library.name, None)?));
  }
  Ok(result)
}

pub fn get_native_library_paths(
  client_info: &McClientInfo,
  library_path: &Path,
) -> SJMCLResult<Vec<PathBuf>> {
  let mut result = Vec::new();
  let feature = FeaturesInfo::default();
  for library in &client_info.libraries {
    if !library.is_allowed(&feature).unwrap_or(false) {
      continue;
    }
    if let Some(natives) = &library.natives {
      if let Some(native) = get_natives_string(natives) {
        let path = convert_library_name_to_path(&library.name, Some(native))?;
        result.push(library_path.join(path));
      } else {
        println!("natives is None");
      }
    }
  }
  Ok(result)
}

pub async fn extract_native_libraries(
  client_info: &McClientInfo,
  library_path: &Path,
  natives_dir: &PathBuf,
) -> SJMCLResult<()> {
  if !natives_dir.exists() {
    fs::create_dir(natives_dir).await?;
  }
  let native_libraries = get_native_library_paths(client_info, library_path)?;
  let tasks: Vec<tokio::task::JoinHandle<SJMCLResult<()>>> = native_libraries
    .into_iter()
    .map(|library_path| {
      let patches_dir_clone = natives_dir.clone();

      tokio::spawn(async move {
        let file = Cursor::new(fs::read(library_path).await?);
        let mut jar = ZipArchive::new(file)?;
        jar.extract(&patches_dir_clone)?;
        Ok(())
      })
    })
    .collect();

  let results = futures::future::join_all(tasks).await;

  for result in results {
    if let Err(e) = result {
      println!("Error handling artifact: {:?}", e);
      return Err(crate::error::SJMCLError::from(e)); // Assuming e is of type SJMCLResult
    }
  }

  Ok(())
}

pub async fn get_invalid_assets(
  source: SourceType,
  asset_path: &Path,
  asset_index: &AssetIndex,
  check_hash: bool,
) -> SJMCLResult<Vec<PTaskParam>> {
  let assets_download_api = get_download_api(source, ResourceType::Assets)?;

  Ok(
    asset_index
      .objects
      .iter()
      .filter_map(|(_, item)| {
        let path = format!("{}/{}", &item.hash[..2], item.hash.clone());
        let dest = asset_path.join(format!("objects/{}", path));

        if dest.exists() && (!check_hash || validate_sha1(dest.clone(), item.hash.clone()).is_ok())
        {
          None
        } else {
          Some(PTaskParam::Download(DownloadParam {
            src: assets_download_api.join(&path).ok()?,
            dest,
            filename: None,
            sha1: Some(item.hash.clone()),
          }))
        }
      })
      .collect::<Vec<_>>(),
  )
}
