use log::{error, info, debug};
use serde_json::Value;
use std::{path::PathBuf, collections::BTreeMap};
// use tauri::{Manager, Theme};

use crate::{app_root, exists, create_file};

const CONF_NAME: &str = "conf.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AppConf {
    /// CSV文件路径
    pub csv_path: PathBuf,
}

impl AppConf {
    pub fn new() -> Self {
        info!("config_init");
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
    pub fn write(self) -> Self {
        let path = &Self::file_path();
        // 判断路径是否存在
        if !exists(path) {
            create_file(path).unwrap();
            info!("conf_create");
        }
        if let Ok(v) = serde_json::to_string_pretty(&self) {
            debug!("Content: {}",&v);
            std::fs::write(path, v).unwrap_or_else(|err| {
                error!("conf_write: {}", err);
                Self::default().write();
            });
        } else {
            error!("conf_ser");
        }
        self
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
        for (k,v) in new_config {
            config.insert(k, v);
        }

        // 将json对象反序列化为self
        match serde_json::to_string_pretty(&config) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(a) => {
                    dbg!(&a);
                    a
                },
                Err(err) => {
                    error!("config_modify_parse: {err}");
                    self
                }
            }
            Err(err) => {
                error!("config_modify_str: {err}");
                self
            }
        }
    }
}