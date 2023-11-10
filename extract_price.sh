#!/bin/bash

extract_bitcoin_price() {
    curl_output=$(curl -L --compressed -H "Accept-Encoding: gzip, deflate, br" -H "Accept-Language: en-US,en;q=0.9" -H "Accept-Charset: utf-8" "https://finance.yahoo.com/quote/BTC-USD?p=BTC-USD&.tsrc=fin-srch")
    price=$(echo "$curl_output" | grep -o '<fin-streamer class="Fw(b) Fz(36px) Mb(-4px) D(ib)" data-symbol="BTC-USD" data-test="qsp-price" data-field="regularMarketPrice" data-trend="none" data-pricehint="2" value="[^"]*" active="">' | awk -F 'value="' '{print $2}' | awk -F '"' '{print $1}')
    echo "Bitcoin Price: $price"
}

extract_bitcoin_price

