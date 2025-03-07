# Rust K9s Wizard

## Overview
This Rust CLI tool automates the process of selecting an AWS SSO profile, logging in, and accessing Kubernetes pods using `kubectl exec`. 

## Features
- Fetches available AWS SSO profiles and allows selection.
- Logs into the selected AWS SSO profile using `aws-sso-profile`.
- Retrieves Kubernetes pod names based on a provided `<pod_name>`.
- Filters pod names that contain `<pod_name>` (not restricted to specific patterns).
- Prompts for selection if multiple matching pods exist.
- Enters the selected pod's shell.

## Prerequisites
- Rust installed (`cargo` should be available).
- `aws-sso` CLI installed and configured.
- `kubectl` installed and configured to access your Kubernetes cluster.

## Installation
1. Clone this repository:
   ```sh
   git clone <repository-url>
   cd rust_k9s_wizard
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the tool:
   ```sh
   ./target/release/rust_k9s_wizard <pod_name>
   ```

## Usage
```sh
rshell <pod_name>
```
- Prompts for AWS profile selection (if multiple exist).
- Finds pods that contain `<pod_name>` in their name.
- Prompts for selection if multiple pods exist.
- Enters the selected pod's shell.

## Testing
Run tests with:
```sh
cargo test
```

## License
MIT
