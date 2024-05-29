use std::io::{BufRead, BufReader, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn render_md_to_catalog() -> Result<Vec<MdInfo>> {
    let path = "assert/md/catalog.md";
    if let Ok(file) = std::fs::File::open(path) {
        let reader = BufReader::new(file);
        let lines = reader.lines().skip(2);
        let mut md_info_vec = Vec::new();
        lines.for_each(|line| {
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
                    _ => {
                        tracing::error!("Failed to create MdInfo from Vec");
                    }
                }
            }
        });
        Ok(md_info_vec)
    } else {
        Err(anyhow::anyhow!("文件打开失败"))
    }
}

pub fn render_md_to_html(name: &str) -> Result<MdContent> {
    let path = format!("assert/md/{}.md", name);
    if let Ok(file) = std::fs::File::open(path) {
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let title = lines.next().unwrap().unwrap();
        let date = lines.next().unwrap().unwrap();
        let _description = lines.next().unwrap().unwrap();
        let content = lines
            .map(|line| line.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        let content = markdown::to_html_with_options(&content, &markdown::Options::gfm())
            .map_err(|_| anyhow::anyhow!("Failed to convert markdown to html"))?;
        Ok(MdContent {
            title,
            date,
            content,
        })
    } else {
        Err(anyhow::anyhow!("文件打开失败"))
    }
}

pub fn add_catalog_by_upload_file(filename: &str, path: &str) -> Result<()> {
    let md_path = format!("assert/md/{}.md", filename);
    if let Ok(_file) = std::fs::File::open(md_path) {
        Ok(())
    } else if let Ok(file) = std::fs::File::open(path) {
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let title = lines.next().unwrap().unwrap();
        let _date = lines.next().unwrap().unwrap();
        let description = lines.next().unwrap().unwrap();
        let catalog_path = "assert/md/catalog.md";
        let mut catalog_file = std::fs::OpenOptions::new()
            .append(true)
            .open(catalog_path)
            .unwrap();
        let content = format!("| {} | {} | {} |\n", filename, title, description);
        catalog_file.write_all(content.as_bytes())?;
        Ok(())
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

#[cfg(test)]
mod tests {
    #[test]
    fn create_catalog_file() {
        let path = "assert/md/index.md";
    }
}
