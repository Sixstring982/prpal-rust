use std::collections::HashMap;

use log::info;

use crate::pulls::pull_request::PullRequest;

pub enum OutputFormat {
    Mermaid,
}

pub fn render_graph(output_format: OutputFormat, prs: &Vec<PullRequest>) -> String {
    match output_format {
        OutputFormat::Mermaid => render_mermaid(prs),
    }
}

fn render_mermaid(prs: &Vec<PullRequest>) -> String {
    info!("Rendering mermaid chart from pull requests: {:?}", prs);

    let pr_numbers_by_branch_name: HashMap<&String, u32> = prs
        .into_iter()
        .map(|pr| (&pr.branch_name, pr.number))
        .collect();

    let mut lines: Vec<String> = vec!["graph TD".to_owned()];

    prs.into_iter()
        .flat_map(|x| get_graph_definition(&pr_numbers_by_branch_name, &x).into_iter())
        .for_each(|x| lines.push(x));

    lines.join("\n")
}

fn get_graph_definition(
    pr_numbers_by_branch_name: &HashMap<&String, u32>,
    pr: &PullRequest,
) -> Vec<String> {
    let number = pr.number;
    let title = safe_pr_title(pr.title.clone());
    let url = &pr.html_url;

    vec![
        format!("  {number}[#{number} {title}]").to_owned(),
        format!("  click {number} \"{url}\"").to_owned(),
    ]
    .into_iter()
    .chain(get_pr_links(pr_numbers_by_branch_name, pr))
    .chain(vec!["".to_owned()])
    .collect()
}

fn get_pr_links(
    pr_numbers_by_branch_name: &HashMap<&String, u32>,
    pr: &PullRequest,
) -> Vec<String> {
    get_diffbase_links(pr)
        .into_iter()
        .chain(get_base_branch_link(pr_numbers_by_branch_name, pr).into_iter())
        .collect::<std::collections::HashSet<String>>()
        .into_iter()
        .collect()
}

fn get_diffbase_links(pr: &PullRequest) -> Vec<String> {
    pr.diffbase
        .clone()
        .into_iter()
        .map(|d| format!("  {} --> {}", pr.number, d).to_owned())
        .collect()
}

fn get_base_branch_link(
    pr_numbers_by_branch_name: &HashMap<&String, u32>,
    pr: &PullRequest,
) -> Option<String> {
    match pr_numbers_by_branch_name.get(&pr.base_branch_name) {
        Some(base_number) => Some(format!("  {} --> {}", pr.number, base_number).to_owned()),
        None => {
            let base_branch = pr.base_branch_name.clone();
            if base_branch.contains("merge-base") || base_branch.contains("merge_base") {
                return None;
            }
            return Some(
                format!("  {} --> {}[({})]", pr.number, base_branch, base_branch).to_owned(),
            );
        }
    }
}

fn safe_pr_title(title: String) -> String {
    title
        .replace("[", "")
        .replace("]", "")
        .replace("(", "")
        .replace(")", "")
        .replace("{", "")
        .replace("}", "")
        .replace('"', "")
        .replace("'", "")
        .replace("@", "")
        .to_owned()
}
