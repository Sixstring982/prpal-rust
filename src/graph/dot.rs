use std::collections::HashMap;

use log::info;

use crate::pulls::pull_request::PullRequest;

pub fn render_dot(prs: &Vec<PullRequest>) -> String {
    info!("Rendering Dot graph from pull requests: {:?}", prs);

    let pr_numbers_by_branch_name: HashMap<&String, u32> = prs
        .into_iter()
        .map(|pr| (&pr.branch_name, pr.number))
        .collect();

    let node_statements = prs
        .into_iter()
        .map(|x| statements_for_node(&pr_numbers_by_branch_name, x))
        .collect::<Vec<String>>()
        .join("\n");

    return vec!["digraph {\n".to_owned(), node_statements, "}".to_owned()].join("");
}

fn statements_for_node(
    pr_numbers_by_branch_name: &HashMap<&String, u32>,
    pr: &PullRequest,
) -> String {
    let attributes = vec![
        format!("    label=\"{}\"", safe_label(&pr.title)),
        format!("    URL=\"{}\"", pr.html_url),
    ]
    .join(",\n");

    let node_definition = vec![
        format!("  {} [", pr.number).to_owned(),
        attributes,
        "  ];".to_owned(),
    ]
    .join("\n");

    let diffbase_links = pr
        .diffbase
        .clone()
        .into_iter()
        .map(|x| format!("  {} -> {};", pr.number, x).to_owned())
        .collect::<Vec<String>>()
        .join("\n");

    let base_branch_link = match pr_numbers_by_branch_name.get(&pr.base_branch_name) {
        Some(base_number) => Some(format!("  {} -> {};", pr.number, base_number).to_owned()),
        None => {
            let base_branch = pr.base_branch_name.clone();
            if base_branch.contains("merge-base") || base_branch.contains("merge_base") {
                None
            } else {
                Some(format!("  {} -> \"{}\";", pr.number, safe_label(&base_branch),).to_owned())
            }
        }
    }
    .unwrap_or("".to_owned());

    return vec![
        node_definition,
        diffbase_links,
        base_branch_link,
        "".to_owned(),
    ]
    .join("\n");
}

fn safe_label(label: &String) -> String {
    label.replace("\"", "'")
}
