use dialoguer::Select;
use std::env;
use std::process::Command;

fn main() {
    // Step 1: Run AWS SSO to log in
    println!("Starting AWS SSO login...");
    let login_status = Command::new("aws-sso")
        .status()
        .expect("Failed to execute AWS SSO");

    if !login_status.success() {
        eprintln!("AWS SSO login failed. Please check your configuration.");
        return;
    }

    // Step 2: Fetch AWS SSO Profiles
    println!("Fetching AWS SSO profiles...");
    let output = Command::new("aws-sso")
        .output()
        .expect("Failed to get AWS SSO profiles");
    let profiles = String::from_utf8_lossy(&output.stdout);
    let profile_list: Vec<&str> = profiles.lines().collect();

    if profile_list.is_empty() {
        eprintln!("No AWS SSO profiles found. Please configure AWS SSO first.");
        return;
    }

    let selection = Select::new()
        .with_prompt("Select AWS profile")
        .items(&profile_list)
        .default(0)
        .interact()
        .expect("Selection failed");

    let selected_profile = profile_list[selection];
    println!("Using profile: {}", selected_profile);

    // Step 3: Set AWS Profile using aws-sso-profile and wait for completion
    let profile_status = Command::new("aws-sso-profile")
        .arg(selected_profile)
        .status()
        .expect("Failed to set AWS profile");

    if !profile_status.success() {
        eprintln!("Failed to set AWS profile. Please complete login.");
        return;
    }

    // Step 4: Get pod name from args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rshell <pod_name>");
        return;
    }
    let pod_name = &args[1];

    // Step 5: Fetch Matching Pods
    println!("Fetching pod instances for: {}", pod_name);
    let output = Command::new("kubectl")
        .args(["get", "pods", "-o", "jsonpath={.items[*].metadata.name}"])
        .output()
        .expect("Failed to list pods");
    let pods = String::from_utf8_lossy(&output.stdout);
    let filtered_pods: Vec<&str> = pods
        .split_whitespace()
        .filter(|p| p.contains(pod_name))
        .collect();

    if filtered_pods.is_empty() {
        eprintln!("No matching pods found for {}", pod_name);
        return;
    }

    let pod_selection = Select::new()
        .with_prompt("Select a pod")
        .items(&filtered_pods)
        .default(0)
        .interact()
        .expect("Selection failed");
    let selected_pod = filtered_pods[pod_selection];
    println!("Selected pod: {}", selected_pod);

    // Step 6: Enter the pod's shell
    let _ = Command::new("kubectl")
        .args(["exec", "-it", selected_pod, "--", "sh"])
        .status()
        .expect("Failed to enter pod shell");
}
