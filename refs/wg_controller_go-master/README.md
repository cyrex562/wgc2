# wg_controller_go

A Wireguard Controller implemented in Go. Provides a HTTP REST Interface/Web UI for managing a WireGuard server.

## Pre-requisites

1. Only tested on Ubuntu Server LTS 18.04 and 20.04
2. Must have go installed and available in the path to build

## Installation

1. Extract the directory on a host with Wireguard installed
2. Build by running the included build.py

## TODO List

1. Group routes a la : <https://gin-gonic.com/docs/examples/grouping-routes/>
2. Start interface
3. Stop interface
4. Add peer to interface
5. Delete peer from interface
6. Delete interface
7. Backup interface files
8. Gen peer config
9. Download peer config
10. Gen qr code for peer config
11. Configure a base_path for the URL to support proxying via NGINX
12. Provide configuration for basic auth from NGINX
13. Auth tokens
