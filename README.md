![Rust](https://github.com/Elpulgo/smhi-cli/workflows/Rust/badge.svg)

# SMHI CLI
Show weather forecast from SMHI in terminal.

## Features
 + Show weather forecast for locations within Swedish territory.
 + Can use default location.
 + Shows
    + Time
    + Temperature
    + Windspeed
    + Rain 
    + Visibility
    + Symbol
 + Display a range between 1 - 10 days from now.

## How to use
```
$ smhi -l "" -r 1 -d
```
+ ` -l` / `--location`
    + Optional
    + Location for forecast, e.g "Stockholm", "Storgatan 1, Stockholm"
    + Mandatory if setting default location
+ ` -r` / `--range`
    + Optional, defaults to 1 day
    + The number of days from now, the forecast should show.
+ `-d` / `--default`
    + Optional
    + Use this flag combined with `--location` to set the location as default. When a default is set, no location option need to be passed.
+ `-s` / `--show-default`
    + Use this flag to show if any default location is set.

## Install
+ [Download](https://github.com/Elpulgo/smhi-cli/releases/download/v.1.0.4/smhi)
+ `mv smhi /usr/local/bin/smhi`
+ `chmod +x smhi` 

## Screen
<img src="https://github.com/Elpulgo/smhi-cli/blob/master/screen/screenshot.png" width="640">

