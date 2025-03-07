use std::process::{Command};
use dialoguer::Select;
use std::env;

fn main() {
    // Step 1: AWS SSO Profile Selection
    println!("Fetching AWS SSO profiles...");
    let output = Command::new("aws-sso")
        .arg("list-profiles")
        .output()
        .expect("Failed to list AWS profiles");
    let profiles = String::from_utf8_lossy(&output.stdout);
    let profile_list: Vec<&str> = profiles.lines().collect();

    let selection = Select::new()
        .with_prompt("Select AWS profile")
        .items(&profile_list)
        .default(0)
        .interact()
        .expect("Selection failed");

    let selected_profile = profile_list[selection];
    println!("Using profile: {}", selected_profile);

    // Step 2: Set AWS Profile
    let _ = Command::new("aws-sso-profile")
        .arg(selected_profile)
        .status()
        .expect("Failed to set AWS profile");

    // Step 3: Get pod name from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rshell <pod_name>");
        return;
    }
    let pod_name = &args[1];

    // Step 4: Fetch Pod Variants (event/http)
    println!("Fetching pod instances for: {}", pod_name);
    let output = Command::new("kubectl")
        .args(["get", "pods", "-o", "jsonpath={.items[*].metadata.name}"])
        .output()
        .expect("Failed to list pods");
    let pods = String::from_utf8_lossy(&output.stdout);
    let filtered_pods: Vec<&str> = pods
        .split_whitespace()
        .filter(|p| p.starts_with(&format!("{}_event_", pod_name)) || p.starts_with(&format!("{}_http_", pod_name)))
        .collect();

    if filtered_pods.is_empty() {
        eprintln!("No matching pods found for {}", pod_name);
        return;
    }

    let pod_selection = Select::new()
        .with_prompt("Select pod type")
        .items(&filtered_pods)
        .default(0)
        .interact()
        .expect("Selection failed");
    let selected_pod = filtered_pods[pod_selection];
    println!("Selected pod: {}", selected_pod);

    // Step 5: Enter the pod's shell
    let _ = Command::new("kubectl")
        .args(["exec", "-it", selected_pod, "--", "sh"])
        .status()
        .expect("Failed to enter pod shell");
}
