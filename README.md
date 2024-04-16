[![CI](https://github.com/gngpp/vproxy/actions/workflows/ci.yml/badge.svg)](https://github.com/gngpp/vproxy/actions/workflows/ci.yml)
[![CI](https://github.com/gngpp/vproxy/actions/workflows/release.yml/badge.svg)](https://github.com/gngpp/vproxy/actions/workflows/release.yml)
 <a target="_blank" href="https://github.com/gngpp/vproxy/blob/main/LICENSE">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg"/>
 </a>
  <a href="https://github.com/gngpp/vproxy/releases">
    <img src="https://img.shields.io/github/release/gngpp/vproxy.svg?style=flat">
  </a>
  </a><a href="https://github.com/gngpp/vproxy/releases">
    <img src="https://img.shields.io/github/downloads/gngpp/vproxy/total?style=flat">
  </a>


# vproxy

Make the request using a random IPv6 address within the IPv6 subnet

### Usage

Taking [tunnelbroker](https://tunnelbroker.net/) / `Debian10` as an example, make sure your server supports `IPv6` and has configured the tunnel

> If you run the program with sudo, it will automatically configure sysctl net.ipv6.ip_nonlocal_bind=1 and ip route add local 2001:470:e953::/48 dev lo for you. If you do not run it with sudo, you will need to configure these manually.

```shell
sysctl net.ipv6.ip_nonlocal_bind=1

# Replace your IPv6 subnet
ip route add local 2001:470:e953::/48 dev lo

# Run
vproxy run -i 2001:470:e953::/48

# Start Daemon (Run in the background), must use sudo
vproxy start -i 2001:470:e953::/48

# Restart Daemon, must use sudo
vproxy restart

# Stop Daemon, must use sudo
vproxy stop

# Show Daemon log
vproxy log

# Show Daemon status
vproxy status

# Online Update
vproxy update

while true; do curl -x http://127.0.0.1:8100 -s https://api.ip.sb/ip -A Mozilla; done
...
2001:470:e953:5b75:c862:3328:3e8f:f4d1
2001:470:e953:b84d:ad7d:7399:ade5:4c1c
2001:470:e953:4f88:d5ca:84:83fd:6faa
2001:470:e953:29f3:41e2:d3f2:4a49:1f22
2001:470:e953:98f6:cb40:9dfd:c7ab:18c4
2001:470:e953:f1d7:eb68:cc59:b2d0:2c6f
```

### Command Manual

##### Description

> If no subnet is configured, the local default network proxy request will be used. When the local machine sets the priority `Ipv4`/`Ipv6` and the priority is `Ipv4`, it will always use `Ipv4` to make requests (if any).

- `--bind`, Http service listening address, default 0.0.0.0:8100
- `--fallback`, The binding address used when IPv6 access is unreachable
- `--ipv6-subnet`, IPv6 subnet
- `--typed`, Proxy type, e.g. http, https, socks5
- `--auth-user`, Basic auth username
- `--auth-pass`, Basic auth password
- `--tls-cert`, TLS certificate file
- `--tls-key`, TLS private key file

> The functionalities of the parameters --typed, --tls-cert, --tls-key have not been implemented yet.

```shell
$ vproxy -h
Random IPv6 request proxy

Usage: vproxy
       vproxy <COMMAND>

Commands:
  run     Run server
  start   Start server daemon
  restart Restart server daemon
  stop    Stop server daemon
  status  Show the server daemon process
  log     Show the server daemon log
  update  Update the application
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ vproxy run -h
Run server

Usage: vproxy run [OPTIONS]

Options:
  -L, --debug                      Debug mode [env: VPROXY_DEBUG=]
  -B, --bind <BIND>                Bind address [default: 0.0.0.0:8100]
  -u, --auth-user <AUTH_USER>      Basic auth username
  -p, --auth-pass <AUTH_PASS>      Basic auth password
  -C, --tls-cert <TLS_CERT>        TLS certificate file
  -K, --tls-key <TLS_KEY>          TLS private key file
  -i, --ipv6-subnet <IPV6_SUBNET>  Ipv6 subnet, e.g. 2001:db8::/32
  -f, --fallback <FALLBACK>        Fallback address
  -t, --typed <TYPED>              Proxy type, e.g. http, https, socks5 [default: http]
  -h, --help                       Print help
```

### Compile

- Linux compile, Ubuntu machine for example:

```shell
git clone https://github.com/gngpp/vproxy.git && cd vproxy
cargo build --release
```

### Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/gngpp/vproxy/pulls).

### Getting help

Your question might already be answered on the [issues](https://github.com/gngpp/vproxy/issues)

### License

**vproxy** © [gngpp](https://github.com/gngpp), Released under the [MIT](./LICENSE) License.
