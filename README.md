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

## Installation (Unix/Linux)
Run the following to install `rshell` globally:
```sh
curl -fsSL https://raw.githubusercontent.com/AncientiCe/rshell/main/install.sh | bash
```

Alternatively, clone and install manually:
```sh
git clone https://github.com/AncientiCe/rshell.git
cd rshell
bash install.sh
```

## Usage
```sh
rshell <pod_name>
```
- Prompts for AWS profile selection (if multiple exist).
- Finds pods that contain `<pod_name>` in their name.
- Prompts for selection if multiple pods exist.
- Enters the selected pod's shell.

## Uninstallation
To remove `rshell`, run:
```sh
curl -fsSL https://raw.githubusercontent.com/AncientiCe/rshell/main/uninstall.sh | bash
```

Or manually:
```sh
bash uninstall.sh
```

## Testing
Run tests with:
```sh
cargo test
```

## License
MIT
