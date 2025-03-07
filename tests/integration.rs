#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_aws_sso_list_profiles() {
        let output = Command::new("aws-sso")
            .arg("list-profiles")
            .output();

        assert!(output.is_ok(), "Failed to execute aws-sso list-profiles");
    }
}
