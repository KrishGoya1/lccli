use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemData {
    pub title: String,
    pub title_slug: String,
    pub difficulty: String,
    pub question_id: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub example_testcases: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub lang: String,
    pub langSlug: String,
    pub code: String,
}

pub struct LeetCodeClient {
    client: reqwest::Client,
}

#[derive(Deserialize)]
struct ProblemsAllResponse {
    stat_status_pairs: Vec<StatStatusPair>,
}

#[derive(Deserialize)]
struct StatStatusPair {
    stat: Stat,
}

#[derive(Deserialize)]
struct Stat {
    // frontend_question_id can be string or int in JSON
    frontend_question_id: Value,
    #[serde(rename = "question__title")]
    _question_title: String,
    #[serde(rename = "question__title_slug")]
    question_title_slug: String,
}

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    data: T,
}

#[derive(Deserialize)]
struct QuestionDataResponse {
    question: Question,
}

#[derive(Deserialize)]
struct Question {
    questionId: String,
    title: String,
    titleSlug: String,
    content: String,
    difficulty: String,
    codeSnippets: Vec<CodeSnippet>,
    exampleTestcases: String,
}

impl LeetCodeClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .build()
                .unwrap(),
        }
    }

    pub async fn resolve_problem_id(&self, frontend_id: &str) -> Result<ProblemData> {
        let url = "https://leetcode.com/api/problems/all/";
        let resp = self.client.get(url).send().await?
            .json::<ProblemsAllResponse>().await
            .context("Failed to parse problems list from LeetCode")?;

        let slug = resp.stat_status_pairs.iter()
            .find(|pair| {
                let id_str = match &pair.stat.frontend_question_id {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    _ => String::new(),
                };
                id_str == frontend_id
            })
            .map(|pair| pair.stat.question_title_slug.clone())
            .ok_or_else(|| anyhow::anyhow!("Problem ID {} not found", frontend_id))?;

        let query = r#"
            query questionData($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    questionId
                    title
                    titleSlug
                    content
                    difficulty
                    codeSnippets {
                        lang
                        langSlug
                        code
                    }
                    exampleTestcases
                }
            }
        "#;
        
        let variables = serde_json::json!({
            "titleSlug": slug
        });

        let gql_body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        let gql_resp = self.client.post("https://leetcode.com/graphql")
            .json(&gql_body)
            .send().await?
            .json::<GraphQLResponse<QuestionDataResponse>>().await
            .context("Failed to fetch/parse question data")?;

        let q = gql_resp.data.question;

        Ok(ProblemData {
            title: q.title,
            title_slug: q.titleSlug,
            difficulty: q.difficulty,
            question_id: q.questionId,
            content: q.content,
            code_snippets: q.codeSnippets,
            example_testcases: q.exampleTestcases,
        })
    }
}
