use vielpork::base::traits::ResourceResolver;
use vielpork::base::structs::ResolvedResource;
use vielpork::base::enums::{DownloadResource,AuthMethod};
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
                    url: url.clone(),
                    headers: vec![],
                    auth: None,
                })
            }
            DownloadResource::Params(params) => {
                // 按顺序从Vec中拿到beatmapset_id、source、username、password
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

                // 判断是否需要osu_login
                let username: String;
                let password: String;

                let url: String;
                if download_source.requires_osu_login {
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
                            url: url.clone(),
                            headers: vec![],
                            auth: Some(AuthMethod::BasicAuth { username, password }),
                        }
                    )
                } else {
                    url = form_url(&base_url, &beatmapset_id, "", "").map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
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

                // 判断是否需要osu_login
                let username: String;
                let password: String;

                let url: String;
                if download_source.requires_osu_login {
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
                    url = form_url(&base_url, &beatmapset_id, &username, &password).map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
                            url: url.clone(),
                            headers: vec![],
                            auth: Some(AuthMethod::BasicAuth { username, password }),
                        }
                    )
                } else {
                    url = form_url(&base_url, &beatmapset_id, "", "").map_err(|e| e.to_string())?;
                    Ok(
                        ResolvedResource{
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
