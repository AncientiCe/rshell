use std::process::Command;
use dialoguer::Select;
use std::os::unix::process::CommandExt;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <pod_name>");
        return;
    }
    let pod_name_filter = &args[1];

    // Step 1: Login AWS SSO
    println!("Starting AWS SSO login...");
    Command::new("aws-sso")
        .status()
        .expect("AWS SSO login failed");

    // Step 2: Select AWS Profile
    let aws_profiles_output = Command::new("aws-sso")
        .output()
        .expect("Failed to fetch AWS profiles");
    let profiles_str = String::from_utf8_lossy(&aws_profiles_output.stdout);
    let profile_names: Vec<&str> = profiles_str
        .lines()
        .skip(2)
        .filter_map(|line| line.split('|').nth(3).map(|s| s.trim()))
        .collect();

    let selected_profile = Select::new()
        .with_prompt("Select AWS profile")
        .items(&profile_names)
        .default(0)
        .interact()
        .expect("Profile selection failed");
    let profile = profile_names[selected_profile];

    // Step 3: Select Kubernetes Context
    let contexts_output = Command::new("kubectl")
        .args(["config", "get-contexts", "--output", "name"])
        .output()
        .expect("Failed to fetch Kubernetes contexts");
    let contexts_str = String::from_utf8_lossy(&contexts_output.stdout);
    let contexts: Vec<&str> = contexts_str.lines().collect();

    let selected_context = Select::new()
        .with_prompt("Select Kubernetes context")
        .items(&contexts)
        .default(0)
        .interact()
        .expect("Context selection failed");
    let context = contexts[selected_context];

    // Final Step: Execute kubectl in interactive shell
    let shell_cmd = format!(
        "aws-sso-profile {} && \
        POD=$(kubectl --context {} get pods -o jsonpath='{{.items[*].metadata.name}}' | tr ' ' '\\n' | grep '{}' | head -n 1) && \
        echo 'Entering shell for pod: '$POD && \
        kubectl --context {} exec -it $POD -- sh",
        profile, context, pod_name_filter, context
    );

    let error = Command::new("zsh")
        .arg("-i")
        .arg("-c")
        .arg(shell_cmd)
        .exec();

    eprintln!("Failed to execute command: {:?}", error);
}