#!/bin/bash
##
## Installation script for fan-control service
##
function abort() {
	# shellcheck disable=SC2145
	echo "Abort: $@" >&2
	exit 1
}
dir="$(pwd)"
answer="none"
ok="true"

## Check to see if we're running as root

if [[  $(id -u) != 0 ]] ; then
	abort "This script must be run as root" >&1
fi

##
## Ask to proceed. Accept only yes nor no for an answer
## 

while [[ ${answer} != "yes" ]] ; do
	echo -n "This script will install build dependencies. Proceed? [yes|no]: "
	read -r answer
	case $answer in
		yes ) break ;; 
		no ) exit 0 ;;
	esac
done

##
## Install dependencies
##
echo "Installing dependencies..."
apt-get update
apt-get install -y build-essential curl

# Check for cargo
if ! command -v cargo &> /dev/null; then
    if [ ! -f "$HOME/.cargo/bin/cargo" ]; then
        echo "Rust not found. Installing..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    fi
	
	source "$HOME/.cargo/env"
	export RUSTUP_HOME=$HOME/.rustup
    export CARGO_HOME=$HOME/.cargo
fi

echo "Dependencies installed."
echo "- `make build` to build the fan-control service."
echo "- `make install` to install the fan-control service." 
echo "- `make install-service` to install the fan-control service."
echo "- `fan-control` to get the current fan speed and temperature."
echo "- `fan-control --help` for usage information."
echo "Setup complete."
