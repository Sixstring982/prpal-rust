mod dot;
mod mermaid;

use clap::ValueEnum;

use crate::pulls::pull_request::PullRequest;

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Dot,
    Mermaid,
}

pub fn render_graph(output_format: OutputFormat, prs: &Vec<PullRequest>) -> String {
    use OutputFormat::*;

    match output_format {
        Mermaid => mermaid::render_mermaid(prs),
        Dot => dot::render_dot(prs),
    }
}

