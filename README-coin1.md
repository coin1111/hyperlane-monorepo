# Hyperlane Libra bridge

## Installation for Ubuntu 24.04

```
# repository
sudo add-apt-repository universe
sudo apt update

# install yarn
sudo apt install yarn

# libssl
sudo apt install libssl-dev pkg-config -y

# install clang and build essentials
sudo apt install -y clang
sudo apt install build-essential -y
sudo apt install libudev-dev -y

# install legacy libssl 1.1.1 needed by solana part
wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.22_amd64.deb
sudo dpkg -i libssl1.1_1.1.1f-1ubuntu2.22_amd64.deb.1
```

## Run tests in manual mode
```
# launch local aptos runtime
# also deployes hyperlane contracts
./run-local-aptos.sh 

# init hyperlane states
./init-aptos-states.sh
```

## To run move tests
```
cd move
./run-tests.sh
```