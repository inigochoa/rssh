# rssh

**rssh** exists to list, ping and connect servers listed inside the SSH config
file.

## Installation

```shell
cargo install rssh
```

## Configuration

**rssh** requires a valid [.ssh/config] file:

```config
Host host01
    HostName 127.0.0.1
    User root
    Port 22
    IdentityFile ~/.ssh/host01.key
```

## Usage

To list available hosts:

```shell
# Long version
rssh --list

# Short version
rssh -l
```

To ssh into a host:

```shell
rssh
```

To ping a host:

```shell
# Long version
rssh --ping

# Short version
rssh -p
```

[.ssh/config]: https://linux.die.net/man/5/ssh_config
