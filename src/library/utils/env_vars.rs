use std::collections::HashMap;

use regex::Regex;

use crate::library::utils::logging;
use std::collections::BTreeMap;

pub async fn print_variables_box(env_vars_map: &BTreeMap<String, (String, String)>) {
    let env_vars_map = env_vars_map.to_owned();

    // If there are no environment variables, don't print anything
    if env_vars_map.is_empty() {
        return;
    }

    logging::nl().await;
    logging::print_color(logging::BG_BLUE, " Environment variables ").await;

    // We need to find the longest key so we can align the table
    let longest_key = env_vars_map.iter().max_by_key(|(key, _)| key.len());
    let longest_key_len = longest_key.map_or(0, |(key, _)| key.len());

    let longest_source_len = env_vars_map
        .iter()
        .max_by_key(|(_, (_, source))| source.len());
    let longest_source_len = longest_source_len.map_or(0, |(_, (_, source))| source.len());

    let key_margin = "─".to_string().repeat(longest_key_len);
    let source_margin = "─".to_string().repeat(longest_source_len);

    // Using longer | character for sides: │
    logging::print_color(logging::NC, &format!("┌─{key_margin}─┬─{source_margin}─┐")).await;
    logging::print_color(
        logging::NC,
        &format!(
            "│ {:<key_width$} │ {:<source_width$} │",
            "Key",
            "Source",
            key_width = longest_key_len,
            source_width = longest_source_len
        ),
    )
    .await;
    logging::print_color(logging::NC, &format!("├─{key_margin}─┼─{source_margin}─┤")).await;

    for (key, (_, source)) in env_vars_map {
        let parsed_key = format!(
            "{:<width$}",
            key.replace(' ', "\u{00A0}"),
            width = longest_key_len
        );
        let parsed_source = format!(
            "{:<width$}",
            source.replace(' ', "\u{00A0}"),
            width = longest_source_len
        );

        logging::print_color(logging::NC, &format!("│ {parsed_key} │ {parsed_source} │")).await;
    }

    logging::print_color(logging::NC, &format!("└─{key_margin}─┴─{source_margin}─┘")).await;
}

pub async fn verify_name(name: String) {
    let regex_result = Regex::new(r"^[a-zA-Z0-9_]+$");
    let regex = match regex_result {
        Ok(regex) => regex,
        Err(e) => {
            logging::error(&format!(
                "Failed to create regex while verifying variable name: {e}"
            ))
            .await;
            std::process::exit(1);
        }
    };

    if !regex.is_match(&name) {
        logging::error(&format!("Invalid variable name: {name}")).await;
        std::process::exit(1);
    }
}

/// Get an environment variable from all sources, in the right order.
#[must_use]
pub fn get_from_all(
    name: &str,
    aliases_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    local_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    process_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    keyring_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    sources_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
) -> Option<String> {
    if let Some(value) = aliases_env_vars.get(name) {
        return Some(value.0.clone());
    }

    if let Some(value) = local_env_vars.get(name) {
        return Some(value.0.clone());
    }

    if let Some(value) = process_env_vars.get(name) {
        return Some(value.0.clone());
    }

    if let Some(value) = sources_env_vars.get(name) {
        return Some(value.0.clone());
    }

    if let Some(value) = keyring_env_vars.get(name) {
        return Some(value.0.clone());
    }

    None
}

pub async fn get_or_exit(
    name: &str,
    aliases_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    local_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    process_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    keyring_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
    sources_env_vars: &HashMap<String, (String, String), std::hash::RandomState>,
) -> String {
    let Some(value) = get_from_all(
        name,
        aliases_env_vars,
        local_env_vars,
        process_env_vars,
        keyring_env_vars,
        sources_env_vars,
    ) else {
        logging::error(&format!(
            "Secret {name} is not set, please set it with `sm secret add {name}`. You can also set it in a .env file or in your environment variables.",
        ))
        .await;
        std::process::exit(1);
    };

    value
}

/// Replaces the environment variables in the string with the actual values
pub async fn replace(
    env_vars_map: &BTreeMap<String, (String, String)>,
    text: &str,
    redact: bool,
) -> String {
    // Match $VAR_NAME or ${VAR_NAME}
    let regex_result = Regex::new(r"\$\{?(\w+)\}?");
    let Ok(re) = regex_result else {
        logging::error("Failed to create regex while replacing environment variables").await;
        std::process::exit(1);
    };

    // Initialize a new String to build the result
    let mut result = String::new();
    let mut last_end = 0;

    // Collect all matches and their positions
    let matches: Vec<_> = re.captures_iter(text).collect();

    for caps in matches {
        // The full match (e.g., ${VAR_NAME})
        let Some(m) = caps.get(0) else {
            logging::error("Failed to get match while replacing environment variables").await;
            std::process::exit(1);
        };

        let var_name = &caps[1]; // The captured variable name

        let value = env_vars_map.get(var_name).map(|(value, _)| value.clone());

        // Append the text before the current match
        result.push_str(&text[last_end..m.start()]);

        // Asynchronously get the environment variable's value
        let value = if redact {
            if let Some(_value) = value {
                format!("[value of {var_name}]")
            } else {
                format!("[unavailable value of {var_name}]")
            }
        } else {
            if let Some(value) = value {
                value
            } else {
                format!("[unavailable value of {var_name}]")
            }
        };

        // Append the retrieved value (or a default if None)
        result.push_str(&value);

        last_end = m.end();
    }

    // Append any remaining text after the last match
    result.push_str(&text[last_end..]);

    result
}
