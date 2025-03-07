# Rust K9s Wizard

## Overview
This Rust CLI tool automates the process of selecting an AWS SSO profile, logging in, selecting a Kubernetes context, and automatically accessing Kubernetes pods using `kubectl exec`.

## Features
- Fetches available AWS SSO profiles and allows interactive selection.
- Logs into the selected AWS SSO profile using `aws-sso-profile`.
- Retrieves available Kubernetes contexts and allows interactive selection.
- Retrieves Kubernetes pod names based on a provided `<pod_name>`.
- Automatically selects the first pod that matches `<pod_name>`.
- Enters the shell of the selected pod automatically.

## Prerequisites
- Rust installed (`cargo` should be available).
- `aws-sso` CLI installed and configured.
- `aws-sso-profile` CLI installed and available in the PATH.
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
- Prompts for AWS profile selection.
- Prompts for Kubernetes context selection.
- Automatically finds and connects to the first pod containing `<pod_name>`.
- Directly opens a shell in the selected pod.

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
