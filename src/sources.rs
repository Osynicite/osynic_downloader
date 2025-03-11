use serde::{Deserialize, Serialize};

#[derive(Clone,Debug,Default,PartialEq,Deserialize, Serialize)]
pub enum DownloadSourceType {
    #[default]
    Default,
    OsuDirect,
    OsuApiV2, // lazor! only
    SayoApi,
    ChimuApi,
    Custom(DownloadSource),
}

impl DownloadSourceType{
    pub fn to_string(&self) -> String {
        match self {
            DownloadSourceType::OsuDirect => "OsuDirect".to_string(),
            DownloadSourceType::OsuApiV2 => "OsuApiV2".to_string(),
            DownloadSourceType::SayoApi => "SayoApi".to_string(),
            DownloadSourceType::ChimuApi => "ChimuApi".to_string(),
            DownloadSourceType::Custom(_) => "Custom".to_string(),
            _ => "Default".to_string(),
        }
    }
        
}

impl From<i32> for DownloadSourceType {
    fn from(i: i32) -> Self {
        match i {
            0 => DownloadSourceType::OsuDirect,
            1 => DownloadSourceType::OsuApiV2,
            2 => DownloadSourceType::SayoApi,
            3 => DownloadSourceType::ChimuApi,
            _ => DownloadSourceType::Default,
        }
    }
}

impl From<&str> for DownloadSourceType {
    fn from(s: &str) -> Self {
        match s {
            "OsuDirect" => DownloadSourceType::OsuDirect,
            "OsuApiV2" => DownloadSourceType::OsuApiV2,
            "SayoApi" => DownloadSourceType::SayoApi,
            "ChimuApi" => DownloadSourceType::ChimuApi,
            _ => DownloadSourceType::Default,
        }
    }
}

impl From<String> for DownloadSourceType {
    fn from(s: String) -> Self {
        DownloadSourceType::from(s.as_str())
    }
}


#[derive(Clone,Debug,PartialEq,Deserialize, Serialize)]
pub struct DownloadSource {
    pub id: u32,
    pub name: String,
    pub owner: String,
    pub site: String,
    pub description: String,
    pub base_url: String,
    pub referer: String,
    pub limit_pre_minute: u32,
    pub limit_threads: u32,
    pub timeout: u32,
    pub support_range: bool,
    pub requires_osu_credentials: bool,
    pub requires_basic_auth: bool,
    pub requires_oauth_token: bool
}

impl From<DownloadSourceType> for DownloadSource {
    fn from(t: DownloadSourceType) -> Self {
        match t {
            DownloadSourceType::OsuDirect => DownloadSource::osu_direct(),
            DownloadSourceType::OsuApiV2 => DownloadSource::osu_apiv2(),
            DownloadSourceType::SayoApi => DownloadSource::sayo_api(),
            DownloadSourceType::ChimuApi => DownloadSource::chimu_api(),
            DownloadSourceType::Custom(s) => s,
            _ => DownloadSource::default()
        }
    }
}

impl Default for DownloadSource {
    fn default() -> Self {
        DownloadSource {
            id: 2,
            name: "sayo! api".to_string(),
                owner : "sayobot <https://github.com/SoulDee/>".to_string(),
                site: "https://osu.sayobot.cn/".to_string(),
                description: "download from sayo! mirror servers.\r\nNo login required.".to_string(),
                base_url: "https://txy1.sayobot.cn/beatmaps/download/full/{}?server=auto".to_string(),
                referer: "https://osu.sayobot.cn/".to_string(),
                limit_pre_minute: 120,
                limit_threads: 3,
                timeout: 5000,
                support_range: false,
                requires_osu_credentials: false,
                requires_basic_auth: false,
                requires_oauth_token: false
        }
    }
}

impl DownloadSource {
    pub fn osu_direct() -> Self {
        DownloadSource {
            id: 0,
            name: "osu! direct".to_string(),
            owner: "osu! <https://osu.ppy.sh/u/2>".to_string(),
            site: "https://osu.ppy.sh/".to_string(),
            description: "directly download from osu! servers.\r\nLogin required.".to_string(),
            base_url: "https://osu.ppy.sh/d/{beatmapset_id}?u={username}&h={hashed_password}&vv=2".to_string(),
            referer: "https://osu.ppy.sh/beatmapsets/".to_string(),
            limit_pre_minute: 60,
            limit_threads: 1,
            timeout: 5000,
            support_range: true,
            requires_osu_credentials: true,
            requires_basic_auth: false,
            requires_oauth_token: false,
        }
    }

    pub fn osu_apiv2() -> Self {
        DownloadSource {
            id: 1,
            name: "osu! apiv2".to_string(),
            owner: "osu! <https://osu.ppy.sh/u/2>".to_string(),
            site: "https://osu.ppy.sh/".to_string(),
            description: "official map download source.\r\nIn order to download maps you need to login using osu! cookies.\r\nInstructions: https://streamable.com/lhlr3d \r\nIf you still need help join discord and read pinned message in #cm-help.\r\nhttps://discord.gg/N854wYZ".to_string(),
            base_url: "https://osu.ppy.sh/beatmapsets/{}/download".to_string(),
            referer: "https://osu.ppy.sh/beatmapsets/".to_string(),
            limit_pre_minute: 60,
            limit_threads: 1,
            timeout: 5000,
            support_range: true,
            requires_osu_credentials: true,
            requires_basic_auth: true,
            requires_oauth_token: false,
        }
    }

    pub fn sayo_api() -> Self {
        DownloadSource {
            id: 2,
            name: "sayo! api".to_string(),
                owner : "sayobot <https://github.com/SoulDee/>".to_string(),
                site: "https://osu.sayobot.cn/".to_string(),
                description: "download from sayo! mirror servers.\r\nNo login required.".to_string(),
                base_url: "https://txy1.sayobot.cn/beatmaps/download/full/{}?server=auto".to_string(),
                referer: "https://osu.sayobot.cn/".to_string(),
                limit_pre_minute: 120,
                limit_threads: 3,
                timeout: 5000,
                support_range: true,
                requires_osu_credentials: false,
                requires_basic_auth: false,
                requires_oauth_token: false
        }
    }

    pub fn chimu_api() -> Self {
        DownloadSource {
            id: 3,
            name: "chimu! api".to_string(),
            owner: "Chimu <>".to_string(),
            site: "https://chimu.moe/".to_string(),
            description: "download from chimu! mirror servers.\r\nNo login required.".to_string(),
            base_url: "https://catboy.best/d/{}".to_string(), // beatmap_id
            referer: "https://catboy.best/search".to_string(),
            limit_pre_minute: 120,
            limit_threads: 3,
            timeout: 5000,
            support_range: true,
            requires_osu_credentials: false,
            requires_basic_auth: false,
            requires_oauth_token: false,
        }
    }

}
