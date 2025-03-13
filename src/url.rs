use crate::error::Result;

// 函数二点五、校验DownloadConfig的base_url字段是否符合要求，传入base_url，
// 分3种情况，1、字符串其中的"{}"如果只有一个，那么则要求requires_osu_login为false，
// 2、字符串中若同时存在{beatmapset_id}、{username}和{hashed_password}，则要求requires_osu_login为true
// 3、字符串不满足上述两种情况，则返回错误
pub fn validate_dapi_base_url(base_url: &str, requires_osu_login: bool) -> Result<()> {
    let url_rule = "If base_url contains only one '{}' then requires_osu_login must be false. If base_url contains '{beatmapset_id}', '{username}' and '{hashed_password}' then requires_osu_login must be true. Other cases are invalid";
    if base_url.matches("{}").count() == 1 {
        if requires_osu_login {
            return Err(format!("Invalid base_url: {}. {}", base_url, url_rule).into());
        }
    } else if base_url.matches("{beatmapset_id}").count() == 1
        && base_url.matches("{username}").count() == 1
        && base_url.matches("{hashed_password}").count() == 1
    {
        if !requires_osu_login {
            return Err(format!("Invalid base_url: {}. {}", base_url, url_rule).into());
        }
    } else {
        return Err(format!("Invalid base_url: {}. {}", base_url, url_rule).into());
    }
    Ok(())
}

pub fn form_url_no_osu_login(base_url: &str, beatmapset_id: &u32) -> Result<String> {
    validate_dapi_base_url(base_url, false)?;
    let url = base_url.replace("{}", &beatmapset_id.to_string());
    Ok(url)
}

pub fn form_url_requires_osu_login(
    base_url: &str,
    beatmapset_id: &u32,
    username: &str,
    hashed_password: &str,
) -> Result<String> {
    validate_dapi_base_url(base_url, true)?;
    let url = base_url
        .replace("{beatmapset_id}", &beatmapset_id.to_string())
        .replace("{username}", username)
        .replace("{hashed_password}", hashed_password);
    Ok(url)
}

pub fn form_url(
    base_url: &str,
    beatmapset_id: &u32,
    username: &str,
    hashed_password: &str,
) -> Result<String> {
    if username.is_empty() || hashed_password.is_empty() {
        form_url_no_osu_login(base_url, beatmapset_id)
    } else {
        form_url_requires_osu_login(base_url, beatmapset_id, username, hashed_password)
    }
}

pub fn form_osu_direct_url(beatmapset_id: &u32, username: &str, hashed_password: &str) -> String {
    format!(
        "https://osu.ppy.sh/d/{beatmapset_id}?u={username}&h={hashed_password}&vv=2",
        beatmapset_id = beatmapset_id,
        username = username,
        hashed_password = hashed_password
    )
}

pub fn form_osu_apiv2_url(beatmapset_id: &u32) -> String {
    format!("https://osu.ppy.sh/beatmapsets/{}/download", beatmapset_id)
}

pub fn form_sayo_api_url(beatmapset_id: &u32) -> String {
    format!(
        "https://txy1.sayobot.cn/beatmaps/download/full/{}?server=auto",
        beatmapset_id
    )
}
