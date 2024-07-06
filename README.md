# CoinGecko CLI
In this project, I will be implementing a CLI that maps to CoingGecko API 
https://www.coingecko.com/en/api/documentation#

I will be implementing the following commands:
* ping
* simple
* coins
* contract
* asset_platforms
* categories
* exchanges
* indexes
* derivatives
* nfts
* exchange_rates
* search
* trending
* global
* companies

Each of them will have more subcommands, you may find more detail in the API documentation.
Initially, I will be supporting the Free API which currently supports **10-**30 **calls**/minute****
and doesn't come with an API key.

The CLI will be implemented using rust
