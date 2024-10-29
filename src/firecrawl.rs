use regex::Regex;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod, HttpRequest, RedirectPolicy},
    serde_json::{self, json},
    Range, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

struct SlashCommandFireCrawlExtension;

impl zed::Extension for SlashCommandFireCrawlExtension {
    fn new() -> Self {
        SlashCommandFireCrawlExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "firecrawl" => Ok(vec![]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if command.name != "firecrawl" {
            return Err("Invalid command. Expected 'firecrawl'.".into());
        }
        let worktree = worktree.ok_or("Worktree is required")?;

        let (verb, url) = match args.len() {
            0 => return Err("Specify [parse|crawl] and where to scrape.".to_string()),
            // 1 if args[0] == "scrape" => return Err("Specify a URL to scrape".to_string()),
            1 if args[0].starts_with("http") => ("scrape", args[0].clone()),
            1 => return Err("Invalid verb/URL; only http/https are supported.".to_string()),
            // 2 if args[0] == "crawl" => return Err("Crawl not yet supported.".to_string()),
            // 2 if args[0] == "scrape" && !args[1].starts_with("http") => ("scrape", args[1].clone()),
            _ => return Err("Unexpected arguments. Expected 'URL'".to_string()),
        };

        // Get the API key from the environment
        let env_vars = worktree.shell_env();
        let api_key = env_vars
            .iter()
            .find(|(key, _)| key == "FIRECRAWL_API_KEY")
            .map(|(_, value)| value.clone())
            .ok_or("FIRECRAWL_API_KEY not found in environment")?;

        let base_url = format!("https://api.firecrawl.dev");
        let request_url = match verb {
            "scrape" => format!("{base_url}/v1/scrape"),
            "crawl" => format!("{base_url}/v1/crawl"),
            _ => return Err("Invalid verb.".to_string()),
        };

        let headers = vec![
            ("Authorization".to_string(), format!("Bearer {}", api_key)),
            ("Content-Type".to_string(), "application/json".to_string()),
        ];
        let body = Some(
            serde_json::to_vec(&json!({
                "url": url.clone(),
                "formats": ["markdown"],
            }))
            .unwrap(),
        );

        let json_request = HttpRequest {
            method: HttpMethod::Post,
            redirect_policy: RedirectPolicy::FollowAll,
            url: request_url,
            headers,
            body,
        };

        let response_json = zed::http_client::fetch(&json_request);
        // TODO: Maybe handle response codes better
        let resp: ScrapeResponse = match response_json {
            Ok(response) => match serde_json::from_slice(&response.body) {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to deserialize response. Error: {}", e)),
            },
            Err(e) => return Err(format!("Failed to fetch: {}", e)),
        };

        let blanklines_regex = Regex::new("(?m) +\n").unwrap();
        let data = resp.data;
        let text = data.markdown;
        let text = blanklines_regex.replace_all(&text, "");
        let text = format!("URL: {}\n{}", url.clone(), text);
        let text = text.to_string();
        let label = format!("{} ( {} )", data.metadata.title, url);

        let sections = vec![SlashCommandOutputSection {
            range: Range {
                start: 0,
                end: text.len() as u32,
            },
            label,
        }];
        Ok(SlashCommandOutput { text, sections })
    }
}

#[derive(serde::Deserialize)]
struct ScrapeResponse {
    // success: bool,
    data: ScrapeResponseData,
}

#[derive(serde::Deserialize)]
struct ScrapeResponseData {
    markdown: String,
    metadata: ScrapeResponseMetaData,
    // html: String,
}

#[derive(serde::Deserialize)]
struct ScrapeResponseMetaData {
    title: String,
    // description: String,
    // language: String,
    // keywords: String,
    // robots: String,
    // #[serde(rename = "ogTitle")]
    // og_title: String,
    // #[serde(rename = "ogDescription")]
    // og_description: String,
    // #[serde(rename = "ogUrl")]
    // og_url: String,
    // #[serde(rename = "ogImage")]
    // og_image: String,
    // #[serde(rename = "ogLocaleAlternate")]
    // og_locale_alternate: Vec<String>,
    // #[serde(rename = "ogSiteName")]
    // og_site_name: String,
    // #[serde(rename = "sourceUrl")]
    // source_url: String,
    // #[serde(rename = "statusCode")]
    // status_code: u32,
}

zed::register_extension!(SlashCommandFireCrawlExtension);
