use regex::Regex;
use serde::Deserialize;
use serde_yaml::from_reader;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};


#[derive(Deserialize, Debug)]
struct AdapterCondition {
    adapter_type: String,
    keywords: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct AdapterConditions {
    conditions: Vec<AdapterCondition>,
}

fn read_adapter_conditions(
    filename: &str,
) -> Result<AdapterConditions, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(from_reader(contents.as_bytes())?)
}

pub fn get_mac_addresses() -> (String, String, String) {
    let output = Command::new("wmic")
        .args(&[
            "path",
            "win32_networkadapter",
            "get",
            "name,macaddress,physicaladapter",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute wmic command")
        .stdout
        .unwrap();
        
    let exe_path = env::current_exe().expect("Unable to get the current executable path");
    let dir = exe_path
        .parent()
        .expect("Unable to get the parent directory of the executable");
    let yaml_path = dir.join("conditions.yaml");
    let yaml_path_str = yaml_path
        .to_str()
        .expect("Unable to convert PathBuf to str");
    let conditions_from_yaml = read_adapter_conditions(yaml_path_str).unwrap_or_else(|err| {
        eprintln!("Error reading YAML file: {}", err);
        std::process::exit(1);
    });
    let conditions = conditions_from_yaml.conditions;

    let reader = BufReader::new(output);
    let mut mac_addresses = ["未采集"; 3].map(String::from);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line_lower = line.to_lowercase();

        for (index, adapter_condition) in conditions.iter().enumerate() {
            let _adapter_type = &adapter_condition.adapter_type; // 如果你需要这个值的话
            let keywords_groups = &adapter_condition.keywords;

            let mut matched = false;
            for keywords in keywords_groups {
                if keywords.iter().all(|kw| line_lower.contains(kw)) {
                    matched = true;
                    break;
                }
            }
            if matched {
                if let Some(mac) = extract_mac_address(&line) {
                    mac_addresses[index] = mac;
                    break; // No need to check further conditions for this line
                }
            }
        }
    }

    (
        mac_addresses[0].clone(),
        mac_addresses[1].clone(),
        mac_addresses[2].clone(),
    )
}

fn extract_mac_address(line: &str) -> Option<String> {
    let mac_regex = Regex::new(r"([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})").unwrap();
    mac_regex
        .captures(line)?
        .get(0)?
        .as_str()
        .to_string()
        .into()
}
