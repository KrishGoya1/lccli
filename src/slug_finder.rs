use serde::Deserialize;
// use std::collections::HashMap;
use std::error::Error;


const ALL_PROBLEMS_URL: &str = "https://leetcode.com/api/problems/all/";

#[derive(Deserialize, Debug)]
struct AllProblemsResponse {
    stat_status_pairs: Vec<StatStatusPair>,
}

#[derive(Deserialize, Debug)]
struct StatStatusPair {
    stat: Stat,
}

#[derive(Deserialize, Debug)]
struct Stat {
    question_id: i32,              // backend problem id
    question__title_slug: String,  // slug
}

/// Fetches the slug for a given backend problemId (question_id).
pub fn get_slug_by_problem_id(problem_id: i32) -> Result<String, Box<dyn Error>> {
    // Fetch all problems JSON
    let resp = ureq::get(ALL_PROBLEMS_URL)
        .call()?
        .into_string()?;

    // Deserialize
    let data: AllProblemsResponse = serde_json::from_str(&resp)?;

    // Option 1: search directly without building a map
    for pair in data.stat_status_pairs {
        if pair.stat.question_id == problem_id {
            return Ok(pair.stat.question__title_slug);
        }
    }

    Err(format!("No slug found for problemId={}", problem_id).into())
}


