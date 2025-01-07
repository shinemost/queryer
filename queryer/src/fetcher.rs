use anyhow::{anyhow, Result};
use tokio::fs;

// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
// #[async_trait]
// Rust 1.75.0 版本支持在trait里定义异步方法，不需要再依赖外部宏 async_trait，该版本于2023年12月28日发布
// 但是在作为特征对象使用时，会报错，此时还需要async_trait宏解决
// 详情可参考async_trait doc以及rust 1.75发布公告
// https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html
pub trait Fetch {
    type Error;

    async fn fetch(&self) -> Result<String, Self::Error>;
}

/// 从文件源或者 http 源中获取数据，返回字符串
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        // 包括 http / https
        "http" => UrlFetcher(name).fetch().await,
        // 处理 file://<filename>
        "file" => FileFetcher(name).fetch().await,
        _ => Err(anyhow!("We only support http/https/file at the moment")),
    }
}

// 针对孤儿原则的解决办法，包装类
struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}