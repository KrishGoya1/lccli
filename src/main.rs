use serde::Deserialize;
use std::fs::File;
use std::io::Write;
mod slug_finder;

const API_URL: &str = "https://leetcode.com/graphql";

#[derive(Deserialize, Debug)]
struct ProblemData {
    data: ProblemQuestion,
}

#[derive(Deserialize, Debug)]
struct ProblemQuestion {
    question: ProblemContent,
}

#[derive(Deserialize, Debug)]
struct ProblemContent {
    questionId: String,
    // questionFrontendId: String,
    title: String,
    titleSlug: String,
    content: String,
    isPaidOnly: bool,

    // difficulty: String,
    // acRate: f64,
    // freqBar: Option<String>,
    // status: Option<String>,
    // isFavor: bool,

    // likes: i32,
    // dislikes: i32,
    // isLiked: Option<bool>,

    // stats: String,
    exampleTestcases: Option<String>,
    sampleTestCase: Option<String>,

    // companyTagStats: Option<String>,
    // hints: Option<Vec<String>>,
    // similarQuestions: String,

    // topicTags: Vec<TopicTag>,
    // companyTags: Option<Vec<CompanyTag>>,

    // hasSolution: bool,
    // hasVideoSolution: bool,

    codeSnippets: Vec<CodeSnippet>,
}

#[derive(Deserialize, Debug)]
struct TopicTag {
    name: String,
    id: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
struct CompanyTag {
    name: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
struct CodeSnippet {
    lang: String,
    langSlug: String,
    code: String,
}

fn main() {
    let graphql = include_str!("query.graphql");

    // take id as input
    let id = std::env::args().nth(1).expect("Please provide a problem id");

    let slug = slug_finder::get_slug_by_problem_id(id.parse::<i32>().unwrap()).unwrap();
    let problem_url = format!("https://leetcode.com/problems/{slug}");

    // get csrf token / cookies
    let agent = ureq::agent();
    let _problem_request = agent
        .get(&problem_url)
        .call()
        .expect("failed initial request");

    // request problem data
    let api_request = agent
        .post(API_URL)
        .send_json(ureq::json!({
            "operationName": "questionData",
            "variables": { "titleSlug": slug },
            "query": graphql
        }))
        .expect("failed api request");

    let problem: ProblemData = api_request
        .into_json()
        .expect("failed to parse response");

    let q = problem.data.question;

    // Build filename: <id>_<problem_name>.txt
    let safe_title = q
        .title
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let filename = format!("{}_{}.txt", q.questionId, safe_title);

    // Build output string with all info
    let mut output = String::new();

    output.push_str(&format!("ID: {}\n", q.questionId));
    // output.push_str(&format!("Frontend ID: {}\n", q.questionFrontendId));
    output.push_str(&format!("Title: {}\n", q.title));
    output.push_str(&format!("Slug: {}\n", q.titleSlug));
    // output.push_str(&format!("Difficulty: {}\n", q.difficulty));
    // output.push_str(&format!("Paid only: {}\n", q.isPaidOnly));
    // output.push_str(&format!("AC rate: {}\n", q.acRate));
    // output.push_str(&format!("Status: {:?}\n", q.status));
    // output.push_str(&format!("Is favorite: {}\n", q.isFavor));
    // output.push_str(&format!("Likes: {}\n", q.likes));
    // output.push_str(&format!("Dislikes: {}\n", q.dislikes));
    // output.push_str(&format!("Is liked: {:?}\n", q.isLiked));
    // output.push_str(&format!("Has solution: {}\n", q.hasSolution));
    // output.push_str(&format!("Has video solution: {}\n", q.hasVideoSolution));
    // output.push_str(&format!("Freq bar: {:?}\n", q.freqBar));
    // output.push_str(&format!("Stats: {}\n", q.stats));
    output.push_str(&format!("Example testcases: {:?}\n", q.exampleTestcases));
    output.push_str(&format!("Sample testcase: {:?}\n", q.sampleTestCase));
    // output.push_str(&format!("Company tag stats: {:?}\n", q.companyTagStats));
    // output.push_str(&format!("Hints: {:?}\n", q.hints));
    // output.push_str(&format!("Similar questions: {}\n", q.similarQuestions));

    // output.push_str("\nTopic tags:\n");
    // for t in &q.topicTags {
    //     output.push_str(&format!("  - {} ({} | {})\n", t.name, t.id, t.slug));
    // }

    // output.push_str("\nCompany tags:\n");
    // if let Some(company_tags) = &q.companyTags {
    //     for c in company_tags {
    //         output.push_str(&format!("  - {} ({})\n", c.name, c.slug));
    //     }
    // } else {
    //     output.push_str("  - None\n");
    // }

    output.push_str("\nCode snippets:\n");
    for cs in &q.codeSnippets {
        output.push_str(&format!("  --- {} ({}) ---\n", cs.lang, cs.langSlug));
        output.push_str(&cs.code);
        output.push_str("\n-------------------------\n");
    }

    output.push_str("\nContent (HTML):\n");
    output.push_str(&q.content);
    output.push('\n');

    // Write everything to file
    let mut file = File::create(&filename).expect("failed to create file");
    file.write_all(output.as_bytes())
        .expect("failed to write to file");

    println!("created file: {}", filename);
}
