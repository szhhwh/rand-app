use log::{debug, error, info, warn};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
// use tauri::{Manager, Theme};

use crate::{app_root, create_file, error::AppError, exists};

const CONF_NAME: &str = "conf.json";

/// APP的配置结构体
///
/// ```
/// // 读取配置文件
/// use rtools::conf::AppConf;
/// AppConf::read();
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConf {
    /// CSV文件路径
    pub csv_path: PathBuf,

    // CSVRand界面参数
    pub csv_animation: bool,
    pub csv_animation_speed: i32,
    pub csv_list: bool,
}

impl Default for AppConf {
    fn default() -> Self {
        Self {
            csv_path: "".into(),
            csv_animation: false,
            csv_animation_speed: 40,
            csv_list: true,
        }
    }
}

impl AppConf {
    pub fn new() -> Self {
        warn!("config_init");
        Default::default()
    }

    /// 返回配置文件路径
    pub fn file_path() -> PathBuf {
        app_root().join(CONF_NAME)
    }

    /// 读取配置文件
    ///
    /// 返回包含配置的 **AppConf** 对象
    ///
    /// # Error
    /// 无法读取配置文件时将返回包含**默认配置**的 AppConf 对象
    pub fn read() -> Self {
        match std::fs::read_to_string(Self::file_path()) {
            Ok(content) => {
                if let Ok(conf) = serde_json::from_str(&content) {
                    conf
                } else {
                    error!("conf_read_parse_error");
                    Self::default()
                }
            }
            Err(err) => {
                error!("conf_read_error: {}", err);
                Self::default()
            }
        }
    }

    /// 写入配置文件
    /// # Error
    /// 若无法写入将在控制台报告错误并返回Self
    pub fn write(self) -> Result<Self, AppError> {
        let path = &Self::file_path();
        // 判断路径是否存在
        if !exists(path) {
            match create_file(path) {
                Ok(_) => (),
                Err(e) => return Err(AppError::Unkown(e)),
            }
            info!("conf_create");
        }
        if let Ok(v) = serde_json::to_string_pretty(&self) {
            debug!("Config context: {}", &v);
            std::fs::write(path, v).unwrap_or_else(|err| {
                error!("conf_write: {}", err);
                Self::default().write().ok();
            });
        } else {
            error!("conf_ser");
        }
        Ok(self)
    }

    /// 传入新的json正文，并更新结构体中的配置数据
    pub fn modify(self, json: Value) -> Self {
        debug!("传入的json对象: {json}");
        // 将结构体转为json
        let config = serde_json::to_value(&self).unwrap();
        let mut config: BTreeMap<String, Value> = serde_json::from_value(config).unwrap();

        // 将新的json转为BTreeMap
        let new_config: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

        // 遍历新的json并将其赋值给config
        for (k, v) in new_config {
            config.insert(k, v);
        }

        // 将json对象反序列化为self
        match serde_json::to_string_pretty(&config) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(a) => {
                    dbg!(&a);
                    a
                }
                Err(err) => {
                    error!("config_modify_parse: {err}");
                    self
                }
            },
            Err(err) => {
                error!("config_modify_str: {err}");
                self
            }
        }
    }

    /// 将结构体转为json
    ///
    /// # Error
    /// 返回 *Result<Value, AppError>*
    pub fn to_json(self) -> Result<Value, AppError> {
        Ok(serde_json::to_value(self)?)
    }
}
