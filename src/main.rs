mod models;
mod cpp_boilerplate_generator;
mod slug_finder;

use models::ProblemData;

const API_URL: &str = "https://leetcode.com/graphql";

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

    cpp_boilerplate_generator::generate_boilerplate(q);
}
