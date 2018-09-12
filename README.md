# Crypto Watch
#### Manage cryptocurrency market data locally

### Required!
* Linux only (at this moment...)
* Curl
* R and Rscript

### Setup
Download the utility:

Download the R script to plot the graphs:

Go to https://pro.coinmarketcap.com/ and create an account. Save the API key.

Make an `config.json` file with all the cryptocurrencies that you want to track. Follow this example:
```
{
  "api_key": "asudai-sugdasb-daisdbaosidb",
  "ref_slug": "bitcoin",
  "data": [
  {
    "website_slug": "bitcoin",
    "wallet": 20
  },
  {
    "website_slug": "ripple",
    "wallet": 12
  },
  {
    "website_slug": "dash",
    "wallet": 154
  },
  {
    "website_slug": "monero",
    "wallet": 60
  }

  ]
}
```
`ref_slug` will be used as a secondary form of tracking. Can be used to compare more closely the price of two cryptocurrencies.

### Use
`./crypto_watch -h`

Displays the help menu.

`./crypto_watch -A`

Gets from the API all the market data needed and stores it in `history.csv`

`./crypto_watch -G <slug>`

Generates a line graph with all the records of certain cryptocurrency in USD price. The .jpg file will be created in your current directory.

>To find the slugs for your cryptocurrency of choice go to https://coinmarketcap.com/, click to enter its page. The slug will be on the right of the name.

#### TODO
* Use the hyper framework instead of Curl
* Other types of graphs
