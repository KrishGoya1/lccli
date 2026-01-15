use regex::Regex;
use std::fs::File;
use std::io::Write;
use crate::models::ProblemContent;

pub fn generate_boilerplate(problem: ProblemContent) {
    let cpp_snippet = problem.codeSnippets.iter()
        .find(|s| s.langSlug == "cpp")
        .expect("C++ snippet not found");

    let original_code = &cpp_snippet.code;
    
    // 1. Extract function implementation from Class Solution if present
    // We want the inner part of: class Solution { public: <THIS> };
    let re_class = Regex::new(r"(?s)class\s+Solution\s*\{\s*public:\s*(.*)\s*\}\s*;").unwrap();
    let method_body = if let Some(cap) = re_class.captures(original_code) {
        cap.get(1).unwrap().as_str().trim().to_string()
    } else {
        original_code.trim().to_string()
    };

    // 2. Parse Function Signature from the method body
    // Typical line: "vector<int> twoSum(vector<int>& nums, int target) {"
    // We look for the last line that ends with "{" or just before the body starts.
    // Actually, it's usually the first line of `method_body`.
    let lines: Vec<&str> = method_body.lines().collect();
    let signature_line = lines.iter()
        .find(|l| l.contains('(') && l.contains(')') && l.trim().ends_with('{'))
        .expect("Could not find function signature line");

    // Regex to parse: "Type Name(Params) {"
    // Captures: 1=Type, 2=Name, 3=Params
    let re_sig = Regex::new(r"^\s*(.+?)\s+(\w+)\s*\((.*)\)\s*\{").unwrap();
    let captures = re_sig.captures(signature_line).expect("Failed to parse function signature");
    
    let return_type = captures.get(1).unwrap().as_str().trim();
    let function_name = captures.get(2).unwrap().as_str().trim();
    let params_str = captures.get(3).unwrap().as_str();

    // 3. Parse Parameters
    // We need types for the testcase tuple.
    // Split params by comma, handling potential commas in template types? 
    // Usually LeetCode params are simple, but `vector<vector<int>>` is common.
    // Naive split by ',' works unless `map<a,b>`.
    // Let's assume standard params.
    let mut param_types: Vec<String> = Vec::new();
    
    // Helper to split params respecting `<>` nesting (basic counter)
    let mut params: Vec<String> = Vec::new();
    let mut current_param = String::new();
    let mut angle_depth = 0;
    
    for c in params_str.chars() {
        if c == '<' { angle_depth += 1; }
        else if c == '>' { angle_depth -= 1; }
        
        if c == ',' && angle_depth == 0 {
            params.push(current_param.trim().to_string());
            current_param.clear();
        } else {
            current_param.push(c);
        }
    }
    if !current_param.trim().is_empty() {
        params.push(current_param.trim().to_string());
    }

    for p in &params {
        // "vector<int>& nums" -> "vector<int>"
        // Remove variable name (last word) and & / *
        let p = p.trim();
        // find last space to separate type and name
        if let Some(space_idx) = p.rfind(' ') {
             let type_part = &p[..space_idx].trim().trim_end_matches('&').trim_end_matches('*').trim();
             param_types.push(type_part.to_string());
        } else {
             param_types.push("auto".to_string());
        }
    }

    // 4. Parse Test Cases from HTML Content
    // We need to be robust. 
    // Strategy: Remove tags, look for "Input:" and "Output:".
    // Note: Some problems use "Input: \n" etc.
    let plain_content = problem.content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("&nbsp;", " ")
        .replace("<p>", "\n")
        .replace("</p>", "\n");

    let re_input = Regex::new(r"Input:\s*(.+?)\n").unwrap();
    let re_output = Regex::new(r"Output:\s*(.+?)\n").unwrap();

    // Iterate and pair them? Or just find all inputs and all outputs.
    // Usually they alternate.
    // Let's try to match blocks: Input: ... Output: ...
    // Note: `.` does not match newline. `(?s)` makes it match.
    // But we want to stop at the next keyword.
    // Regex: `Input:\s*(.*?)\s*Output:\s*(.*?)\s*(?:Explanation|Example|$)`
    let re_block = Regex::new(r"(?s)Input:\s*(.*?)\s*Output:\s*(.*?)\n(?:Explanation|Example|$)").unwrap();

    let mut tcs: Vec<String> = Vec::new();
    let mut ans: Vec<String> = Vec::new();

    // The regex above might be too strict with newlines. 
    // Let's try to capture Input line and Output line.
    
    // Simpler approach: split by "Example "
    let examples: Vec<&str> = plain_content.split("Example ").skip(1).collect(); // skip preamble
    
    for ex in examples {
        // ex contains "1: ... Input: ... Output: ..."
        if let (Some(in_idx), Some(out_idx)) = (ex.find("Input:"), ex.find("Output:")) {
            if in_idx < out_idx {
                let input_str = ex[in_idx+6..out_idx].trim();
                
                // End of output is usually end of line or "Explanation"
                let after_output = &ex[out_idx+7..];
                let output_end = after_output.find("Explanation").unwrap_or(after_output.len());
                // also stop at newline if it looks like the end of value? 
                // Actually `Output: [0,1]\n` is common.
                let mut output_str = after_output[..output_end].trim();
                // If contains newline, maybe take first line? 
                // But sometimes output is multiline matrix.
                
                // Clean input: "nums = [2,7,11,15], target = 9"
                // We want to extract values.
                // If we have N params, we expect N assignments.
                // We can't rely on "var =" names because they might differ.
                // Regex to extract values: `= (value)` ?? No.
                // Better: Just extract properly correctly formated C++ values.
                // Arrays `[...]`
                // Strings `"..."`
                // Numbers
                // Booleans `true/false`
                
                // Let's use a heuristic: replace `[` with `{` and `]` with `}`.
                // And remove variable names.
                // "nums = [2,7...], target = 9" -> "{2,7...}, 9"
                
                // Regex to remove "word ="
                let re_var = Regex::new(r"[a-zA-Z0-9_]+\s*=\s*").unwrap();
                let clean_in = re_var.replace_all(input_str, "").to_string();
                let cpp_in = clean_in.replace('[', "{").replace(']', "}");
                
                if params.len() > 1 {
                    tcs.push(format!("{{ {} }}", cpp_in));
                } else {
                    tcs.push(format!("{}", cpp_in));
                }

                let cpp_out = output_str.replace('[', "{").replace(']', "}");
                ans.push(cpp_out);
            }
        }
    }

    // Types
    let tc_type = if param_types.len() == 1 {
        param_types[0].clone()
    } else {
        format!("tuple<{}>", param_types.join(", "))
    };
    let ans_type = return_type.to_string();

    // Generate output
    let mut file_content = String::new();
    file_content.push_str(&format!("// {} {}\n", problem.questionId, problem.titleSlug));
    file_content.push_str("#include <bits/stdc++.h>\n");
    file_content.push_str("using namespace std;\n\n");
    
    // Printer helper
    file_content.push_str("template<typename T>\n");
    file_content.push_str("ostream& operator<<(ostream& os, const vector<T>& v) {\n");
    file_content.push_str("    os << \"[\";\n");
    file_content.push_str("    for(int i=0; i<v.size(); ++i) {\n");
    file_content.push_str("        os << v[i];\n");
    file_content.push_str("        if(i < v.size()-1) os << \",\";\n");
    file_content.push_str("    }\n");
    file_content.push_str("    os << \"]\";\n");
    file_content.push_str("    return os;\n");
    file_content.push_str("}\n\n");

    file_content.push_str(&method_body);
    file_content.push_str("\n\n");
    
    file_content.push_str("int main() {\n");
    file_content.push_str(&format!("    vector<{}> testcases = {{\n", tc_type));
    for tc in &tcs {
        file_content.push_str(&format!("        {},\n", tc));
    }
    file_content.push_str("    };\n\n");
    
    file_content.push_str(&format!("    vector<{}> answers = {{\n", ans_type));
    for a in &ans {
        file_content.push_str(&format!("        {},\n", a));
    }
    file_content.push_str("    };\n\n");

    file_content.push_str("    int t = testcases.size();\n");
    file_content.push_str("    for(int i = 0; i < t; i++) {\n");
    
    if param_types.len() > 1 {
        file_content.push_str(&format!("        {} ans = std::apply({}, testcases[i]);\n", ans_type, function_name));
    } else {
        file_content.push_str(&format!("        {} ans = {}(testcases[i]);\n", ans_type, function_name));
    }
    
    file_content.push_str("        if(ans == answers[i]) {\n");
    file_content.push_str("            cout << \"Testcase \" << i+1 << \" passed!\\n\";\n");
    file_content.push_str("        } else {\n");
    file_content.push_str("            cout << \"Testcase \" << i+1 << \" failed!\\n\";\n");
    file_content.push_str("            cout << \"Expected: \" << answers[i] << \"\\n\";\n");
    file_content.push_str("            cout << \"Got: \" << ans << \"\\n\";\n");
    file_content.push_str("            break;\n");
    file_content.push_str("        }\n");
    file_content.push_str("    }\n");
    file_content.push_str("    return 0;\n");
    file_content.push_str("}\n");

    let safe_title = problem.title
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let filename = format!("{}_{}.cpp", problem.questionId, safe_title);
    
    let mut file = File::create(&filename).expect("failed to create file");
    file.write_all(file_content.as_bytes()).expect("failed to write file");
    println!("Generated {}", filename);
}
