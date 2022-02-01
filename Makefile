# include .env file and export its env vars
# (-include to ignore error if it does not exist)
-include .env

all: solc compile

# Install proper solc version.
solc:; nix-env -f https://github.com/dapphub/dapptools/archive/master.tar.gz -iA solc-static-versions.solc_0_8_11

# Compile the Sample Smart Contract
compile  :; solc --bin-runtime --optimize -o . SampleContract.sol
