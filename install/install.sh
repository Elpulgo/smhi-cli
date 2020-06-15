#!/usr/bin/env bash

install() {
    export FILE="smhi"
    curl "https://raw.githubusercontent.com/Elpulgo/smhi-cli/master/dist/smhi.tar.gz" | tar xvz
    chmod +x smhi
    sudo mv smhi /usr/local/bin/smhi
}

install