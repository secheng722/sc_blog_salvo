use std::io::{BufRead, BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn render_md_to_catalog() -> Result<Vec<MdInfo>> {
    let path = "assert/md/catalog.md";
    if let Ok(file) = std::fs::File::open(path) {
        tracing::info!("file: {:?}", file);
        let reader = BufReader::new(file);
        let lines = reader.lines().skip(2);
        let mut md_info_vec = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let items: Vec<&str> = line
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();
                match MdInfo::new_from_vec(items) {
                    Ok(md_info) => {
                        md_info_vec.push(md_info);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
        Ok(md_info_vec)
    } else {
        Err(anyhow::anyhow!("文件打开失败"))
    }
}

pub fn render_md_to_html(name: &str) -> Result<MdContent> {
    let path = format!("assert/md/{}.md", name);
    if let Ok(file) = std::fs::File::open(&path) {
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let title = lines.next().unwrap().unwrap();
        let date = lines.next().unwrap().unwrap();
        let content = lines
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        let content = markdown::to_html(&content);
        tracing::info!("content: {:?}", content);
        Ok(MdContent {
            title,
            date,
            content,
        })
    } else {
        Err(anyhow::anyhow!("文件打开失败"))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MdInfo {
    key: String,
    title: String,
    description: String,
}

impl MdInfo {
    pub fn new_from_vec(items: Vec<&str>) -> Result<Self> {
        if items.len() == 3 {
            Ok(Self {
                key: items[0].to_string(),
                title: items[1].to_string(),
                description: items[2].to_string(),
            })
        } else {
            Err(anyhow::anyhow!("items 长度不为 3"))
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MdContent {
    pub title: String,
    pub date: String,
    pub content: String,
}
