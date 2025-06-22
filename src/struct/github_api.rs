// 構造体の種類
// 名前付き構造体
// タプル構造体
// ユニット構造体（主にトレイト実装のためのマーカーとして利用される）

// 構造体へのメソッド実装
// 関連関数（Self）
// 不変メソッド（$self）
// 可変メソッド（&mut self）
// 消費メソッド（self）

use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GitHubSearchParams {
    pub q: String,
    pub sort: String,
    pub order: String,
    pub per_page: u32,
    pub page: u32,
}

#[derive(Debug, Deserialize)]
pub struct GitHubSearchResponse {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<GitHubRepo>
}

#[derive(Debug, Deserialize)]
pub struct  GitHubRepo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
}

impl GitHubSearchParams {
    pub fn new(q: String) -> Self {
        Self { q, sort: "stars".to_string(), order: "desc".to_string() }
    }

    // 関連関数
    pub fn rest_repositories() -> Self {
        Self {
            q: "language:rust".to_string(),
            sort: "stars".to_string(),
            order: "desc".to_string(),
            per_page: 10,
            page: 1,
        }
    }

    // 不変メソッド
    pub async fn search_popular_repos(&self) -> Result<GitHubSearchResponse, Error> {
        let client = reqwest::Client::new();
        
        let response = client
            .get("https://api.github.com/search/repositories")
            .query(&self)
            .header("User-Agent", "Rust-Example")
            .send()
            .await()?;

        if response.status().is_success() {
            let repos = response.json::<GitHubSearchResponse>().await?;
            Ok(repos)
        } else {
            Err(Error::from_str("API request failed"))
        }
    }
}

async fn example_github_api() -> Result<(), Error> {
    let params = GitHubSearchParams::rust_repositories();

    let custom_params = GitHubSearchParams {
        q: "rust web framework".to_string(),
        page: 2,
        ..GitHubSearchParams::defalt()
    };

    match params.search_popular_repos().await {
        OK(reponse) => {
            println!("合計結果数: {}", response.total_count);

            for (i, repo) in response.items.iter().enumerate() {
                println!("{}. {} - ★ {}",
                        i + 1,
                        repo.full_name,
                        repo.stargazers_count
                );

                if let Some(desc) = &repo.description {
                    println!("    {}", desc)
                }


                println!("    {}", repo.html_url);
                println!();
                OK(())
            }
        },
        Err(err) => {
            eprintln!("エラーが発生しました: {}", err);
            Err(err)
        }
    }
}