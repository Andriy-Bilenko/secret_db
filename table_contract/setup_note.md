# start working
```bash
npm install
git init

# revise Makefile, .env's, scripts/

# setup localsecret, secretcli and this project net config
rm -rf ~/.secretd

docker run -it -p 9091:9091 -p 26657:26657 -p 1317:1317 -p 5000:5000 \
--name localsecret -v ~/.secretd:/root/.secretd ghcr.io/scrtlabs/localsecret:v1.15.0-beta.7

secretcli config set client chain-id secretdev-1
secretcli config set client keyring-backend test
secretcli config set client output json
secretcli config set client node http://localhost:26657
sudo chown -R $(whoami):$(whoami) ~/.secretd

make network-localnet
# build smart contract, optimise it and build .ts scripts
make build # if not successful try cleanbuilding
# store and instantiate
make store
make instantiate
```
# explanations
`schema/` contains json files of state, queries and responces
`scripts/` contain .ts scripts for storing, instantiating and executing smart contract

