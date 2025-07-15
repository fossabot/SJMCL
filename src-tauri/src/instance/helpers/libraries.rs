use crate::{
  error::SJMCLResult,
  instance::{helpers::client_json::McClientInfo, models::misc::InstanceError},
  launch::helpers::file_validator::{
    get_native_library_artifacts, get_nonnative_library_artifacts,
  },
  launcher_config::models::GameDirectory,
  resource::{
    helpers::misc::get_download_api,
    models::{ResourceType, SourceType},
  },
  tasks::{download::DownloadParam, PTaskParam},
};

pub async fn get_libraries_download_params(
  directory: &GameDirectory,
  version_info: &McClientInfo,
  source: SourceType,
) -> SJMCLResult<Vec<PTaskParam>> {
  let libraries_download_api = get_download_api(source, ResourceType::Libraries)?;
  let mut artifacts = Vec::new();
  artifacts.extend(get_native_library_artifacts(version_info));
  artifacts.extend(get_nonnative_library_artifacts(version_info));

  Ok(
    artifacts
      .iter()
      .map(|artifact| {
        let file_path = directory.dir.join(&artifact.path);
        PTaskParam::Download(DownloadParam {
          src: libraries_download_api
            .join(&artifact.path)
            .map_err(|_| InstanceError::ClientJsonParseError)
            .unwrap(),
          dest: file_path,
          filename: None,
          sha1: Some(artifact.sha1.clone()),
        })
      })
      .collect(),
  )
}
