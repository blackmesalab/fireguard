use std::env;
use std::fs as std_fs;
use std::os::unix::fs::PermissionsExt;

use color_eyre::eyre::{bail, Result};
use guess_host_triple::guess_host_triple;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};

use crate::shell::Shell;
use crate::utils::{build_reqwest_client, current_executable_path, NEW_VERSION_PATH, NEW_VERSION_FILE};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Releases {
    pub url: String,
    #[serde(rename = "assets_url")]
    pub assets_url: String,
    #[serde(rename = "upload_url")]
    pub upload_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(rename = "target_commitish")]
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "published_at")]
    pub published_at: String,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    pub body: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub http_cli: Client,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: Uploader,
    #[serde(rename = "content_type")]
    pub content_type: String,
    pub state: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

impl Releases {
    pub async fn new(url: &str) -> Result<Self> {
        let cli = build_reqwest_client(None, None)?;
        let mut releases = cli.get(url).send().await?.json::<Releases>().await?;
        releases.http_cli = cli;
        Ok(releases)
    }

    pub async fn download_for_triple(self, triple: &str) -> Result<()> {
        let tar_file = format!("fireguard-{}.tar.gz", triple);
        let asset = self.assets.into_iter().find(|a| a.name == tar_file);
        match asset {
            Some(asset) => {
                let data = self.http_cli.get(&asset.browser_download_url).send().await?.bytes().await?;
                info!("Downloaded new version {} from {}", self.tag_name, asset.browser_download_url);
                let tmp_path = env::temp_dir();
                let filename = tmp_path.join(asset.name);
                let mut file = fs::File::create(&filename).await?;
                file.write_all(&data).await?;
                info!("Downloaded new version {} on {}", self.tag_name, filename.display());
                let result = Shell::exec(
                    "tar",
                    &format!("xfvz {}", tar_file),
                    Some(tmp_path.to_str().unwrap_or_default()),
                    false,
                )
                .await;
                if result.success() {
                    fs::copy(&tmp_path.join("fireguard"), NEW_VERSION_FILE.as_path()).await?;
                    info!("Exracted new version {} in {}", self.tag_name, NEW_VERSION_FILE.display());
                    let mut perms = std_fs::metadata(NEW_VERSION_FILE.as_path())?.permissions();
                    perms.set_mode(0o755);
                    std_fs::set_permissions(NEW_VERSION_FILE.as_path(), perms)?;
                } else {
                    bail!(
                        "Unable to extractract new version {} on {}: {}",
                        self.tag_name,
                        NEW_VERSION_FILE.display(),
                        result.stderr()
                    );
                }
            }
            None => bail!("Unable to find a valid asset for release {} on {}", self.tag_name, triple),
        }
        Ok(())
    }

    pub async fn download(self) -> Result<()> {
        let host_triple = match guess_host_triple() {
            Some(t) => {
                info!("Found rustc triple for current host: {}", t);
                t
            }
            None => bail!("Unable to find rustc host triple for current intallation"),
        };
        Ok(self.download_for_triple(host_triple).await?)
    }
}
