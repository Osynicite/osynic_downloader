use vielpork::base::traits::ResourceResolver;
use vielpork::base::structs::ResolvedResource;
use vielpork::base::enums::{DownloadResource,AuthMethod};
use vielpork::base::algorithms::generate_task_id;
use async_trait::async_trait;

use crate::sources::{DownloadSource, DownloadSourceType};
use crate::url::form_url;

#[derive(Debug,Clone)]
pub struct OsuBeatmapsetResolver {}

impl OsuBeatmapsetResolver {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ResourceResolver for OsuBeatmapsetResolver {
    async fn resolve(&self, resource: &DownloadResource) -> vielpork::error::Result<ResolvedResource> {
        match resource {
            DownloadResource::Url(url) => {
                Ok(ResolvedResource{
                    id: generate_task_id(url),
                    url: url.clone(),
                    headers: vec![],
                    auth: None,
                })
            }
            DownloadResource::Id(id) => {
                let beatmapset_id: u32;
                match id.parse::<u32>() {
                    Ok(id) => {
                        beatmapset_id = id;
                    }
                    Err(_) => {
                        return Err("Invalid beatmapset id".into());
                    }
                }
                let download_source = DownloadSource::from(DownloadSourceType::Default);
                let base_url = download_source.base_url.clone();
                let url = form_url(&base_url, &beatmapset_id, "", "").map_err(|e| e.to_string())?;
                Ok(ResolvedResource{
                    id: beatmapset_id,
                    url: url.clone(),
                    headers: vec![],
                    auth: None,
                })
            }
            DownloadResource::Params(params) => {
                let beatmapset_id: u32;
                let source: String;

                match params.get(0) {
                    Some(id) => {
                        match id.parse::<u32>() {
                            Ok(id) => {
                                beatmapset_id = id;
                            }
                            Err(_) => {
                                return Err("Invalid beatmapset id".into());
                            }
                        }
                    }
                    None => {
                        return Err("Missing beatmapset_id".into());
                    }
                }
                match params.get(1) {
                    Some(src) => {
                        source = src.clone();
                    }
                    None => {
                        return Err("Missing source".into());
                    }
                }
                let download_source = DownloadSource::from(DownloadSourceType::from(source));
                let base_url = download_source.base_url.clone();

                let username: String;
                let password: String;

                let url: String;
                if download_source.requires_osu_credentials {
                    match params.get(2) {
                        Some(name) => {
                            username = name.clone();
                        }
                        None => {
                            return Err("Missing username".into());
                        }
                    }
                    match params.get(3) {
                        Some(pass) => {
                            password = pass.clone();
                        }
                        None => {
                            return Err("Missing password".into());
                        }
                    }
                    url = form_url(&base_url, &beatmapset_id, &username, &password).map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
                            id:beatmapset_id,
                            url: url.clone(),
                            headers: vec![],
                            auth: Some(AuthMethod::Basic { username, password }),
                        }
                    )
                } else {
                    url = form_url(&base_url, &beatmapset_id, "", "").map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
                            id:beatmapset_id,
                            url: url.clone(),
                            headers: vec![],
                            auth: None,
                        }
                    )
                }

            }
            DownloadResource::HashMap(hashmap) => {
                let beatmapset_id: u32;
                let source: String;
                match hashmap.get("beatmapset_id") {
                    Some(id) => {
                        match id.parse::<u32>() {
                            Ok(id) => {
                                beatmapset_id = id;
                            }
                            Err(_) => {
                                return Err("Invalid beatmapset id".into());
                            }
                        }
                    }
                    None => {
                        return Err("Missing beatmapset_id".into());
                    }
                }
                match hashmap.get("source") {
                    Some(src) => {
                        source = src.clone();
                    }
                    None => {
                        return Err("Missing source".into());
                    }
                }
                let download_source = DownloadSource::from(DownloadSourceType::from(source));
                let base_url = download_source.base_url.clone();

                let username: String;
                let password: String;

                let url: String;
                if download_source.requires_osu_credentials {
                    match hashmap.get("username") {
                        Some(name) => {
                            username = name.clone();
                        }
                        None => {
                            return Err("Missing username".into());
                        }
                    }
                    match hashmap.get("password") {
                        Some(pass) => {
                            password = pass.clone();
                        }
                        None => {
                            return Err("Missing password".into());
                        }
                    }
                    if download_source.requires_basic_auth{
                        url = form_url(&base_url, &beatmapset_id, "","").map_err(|e| e.to_string())?;
                        Ok(
                            ResolvedResource{
                                id:beatmapset_id,
                                url: url.clone(),
                                headers: vec![],
                                auth: Some(AuthMethod::Basic { username, password }),
                            }
                        )
                    } else {
                        url = form_url(&base_url, &beatmapset_id, &username, &password).map_err(|e| e.to_string())?;
                        Ok(
                            ResolvedResource{
                                id:beatmapset_id,
                                url: url.clone(),
                                headers: vec![],
                                auth: None,
                            }
                        )
                    }
                } else {
                    url = form_url(&base_url, &beatmapset_id, "", "").map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
                            id:beatmapset_id,
                            url: url.clone(),
                            headers: vec![],
                            auth: None,
                        }
                    )
                }                
            }
            DownloadResource::Resolved(resolved) => {
                Ok(resolved.clone())
            }
        }
    }
}
