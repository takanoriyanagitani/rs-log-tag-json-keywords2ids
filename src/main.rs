use std::process::ExitCode;

use std::io;

use rs_log_tag_json_keywords2ids::words2ids::aho::corasick::Config;
use rs_log_tag_json_keywords2ids::words2ids::aho::corasick::BODY_KEY_DEFAULT;
use rs_log_tag_json_keywords2ids::words2ids::aho::corasick::MATCH_KIND_DEFAULT;
use rs_log_tag_json_keywords2ids::words2ids::aho::corasick::TAGS_KEY_DEFAULT;

fn patterns_from_args() -> impl Iterator<Item = String> {
    std::env::args().skip(1)
}

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn match_kind() -> String {
    env_val_by_key("ENV_MATCH_KIND")
        .ok()
        .unwrap_or_else(|| MATCH_KIND_DEFAULT.into())
}

fn body_key() -> String {
    env_val_by_key("ENV_BODY_KEY")
        .ok()
        .unwrap_or_else(|| BODY_KEY_DEFAULT.into())
}

fn tags_key() -> String {
    env_val_by_key("ENV_TAGS_KEY")
        .ok()
        .unwrap_or_else(|| TAGS_KEY_DEFAULT.into())
}

fn config() -> Config {
    Config {
        body_key: body_key(),
        tags_key: tags_key(),
    }
}

fn stdin2stdout() -> Result<(), io::Error> {
    let patterns = patterns_from_args();
    let kind: String = match_kind();
    let cfg: Config = config();
    cfg.stdin2ids2maps2stdout(patterns, &kind)
}

fn sub() -> Result<(), io::Error> {
    stdin2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
