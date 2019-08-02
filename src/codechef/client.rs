use crate::util::HtmlClient;
use crate::Result;
use reqwest::Client;

use super::*;

const CODECHEF_PREFIX: &str = "https://www.codechef.com/problems/";

pub enum CodeChefProblemPage {
    Beginner,
    Easy,
    Medium,
    Hard,
    Challenge,
    Peer,
}

impl CodeChefProblemPage {
    fn value(&self) -> &str {
        match self {
            CodeChefProblemPage::Beginner => "school",
            CodeChefProblemPage::Easy => "easy",
            CodeChefProblemPage::Medium => "medium",
            CodeChefProblemPage::Hard => "hard",
            CodeChefProblemPage::Challenge => "challenge",
            CodeChefProblemPage::Peer => "extcontest",
        }
    }
}

pub struct CodeChefClient {
    client: Client,
}

impl Default for CodeChefClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl CodeChefClient {
    pub fn fetch_problem_list(&self, page: CodeChefProblemPage) -> Result<Vec<CodeChefProblem>> {
        let url = format!("{}{}", CODECHEF_PREFIX, page.value());
        let html = self.client.get_html(&url)?;
        problem::scrape(&html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_problem_list() {
        let client = CodeChefClient::default();
        assert!(client
            .fetch_problem_list(CodeChefProblemPage::Beginner)
            .is_ok());
        assert!(client.fetch_problem_list(CodeChefProblemPage::Easy).is_ok());
        assert!(client
            .fetch_problem_list(CodeChefProblemPage::Medium)
            .is_ok());
        assert!(client.fetch_problem_list(CodeChefProblemPage::Hard).is_ok());
        assert!(client
            .fetch_problem_list(CodeChefProblemPage::Challenge)
            .is_ok());
        assert!(client.fetch_problem_list(CodeChefProblemPage::Peer).is_ok());
    }
}
