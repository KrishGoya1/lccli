use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProblemData {
    pub data: ProblemQuestion,
}

#[derive(Deserialize, Debug)]
pub struct ProblemQuestion {
    pub question: ProblemContent,
}

#[derive(Deserialize, Debug)]
pub struct ProblemContent {
    pub questionId: String,
    // pub questionFrontendId: String,
    pub title: String,
    pub titleSlug: String,
    pub content: String,
    pub isPaidOnly: bool,

    // pub difficulty: String,
    // pub acRate: f64,
    // pub freqBar: Option<String>,
    // pub status: Option<String>,
    // pub isFavor: bool,

    // pub likes: i32,
    // pub dislikes: i32,
    // pub isLiked: Option<bool>,

    // pub stats: String,
    pub exampleTestcases: Option<String>,
    pub sampleTestCase: Option<String>,

    // pub companyTagStats: Option<String>,
    // pub hints: Option<Vec<String>>,
    // pub similarQuestions: String,

    // pub topicTags: Vec<TopicTag>,
    // pub companyTags: Option<Vec<CompanyTag>>,

    // pub hasSolution: bool,
    // pub hasVideoSolution: bool,

    pub codeSnippets: Vec<CodeSnippet>,
}

#[derive(Deserialize, Debug)]
pub struct TopicTag {
    pub name: String,
    pub id: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct CompanyTag {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct CodeSnippet {
    pub lang: String,
    pub langSlug: String,
    pub code: String,
}
