# wgc2: Wireguard Command and Control

## About

wgc2 Is a web service that provides command and control of wireguard on a host. wgc2 provides REST API and Web UI access to the wg tool as well as some elements of iproute2 and iptables.

### Built With

* Rust

## Getting Started

### Prerequisites

* Rust > 1.47
* Wireguard

### Installation

1. Clone the repo

2. cd into the source directory

3. build the project with `cargo build`

4. (optional) copy the wgc2 binary from the `target` directory to somewhere else.

## Usage

* Execute the wgc2 binary
* Optionally set command-line parameters

```
USAGE:
    wgc2 [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <BIND ADDRESS>    address to bind to
    -p, --port <PORT NUMBER>        port to listen on
```

## Roadmap

1. Web UI
2. full wg control
3. replacement of wg-quick
4. partial iproute2 control
5. partial iptables/ufw control
6. import/export of config data via web API/UI
7. storage/tracking of configuration data outside of configuration files.

## Contributing

1. Fork the project
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Open a pull request

## License

Copyright Josh M. 2020

## Contact

Josh M. - cyrex562@gmail.com

Project Link: <https://github.com/cyrex562/wgc2>

## Acknowledgements

* The Wireguard team for making an amazing system.