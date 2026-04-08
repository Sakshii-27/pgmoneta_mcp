# pgmoneta-mcp-admin user guide

The **pgmoneta-mcp-admin** command line interface manages your users for [**pgmoneta-mcp**](https://github.com/pgmoneta/pgmoneta-mcp).

```
pgmoneta-mcp-admin
  Administration utility for pgmoneta-mcp

Usage:
  pgmoneta-mcp-admin [OPTIONS] [COMMAND]

Options:
  -f, --file <FILE>          The user configuration file
  -U, --user <USER>          The user name
  -P, --password <PASSWORD>  The password for the user
  -g, --generate             Generate a password
  -l, --length <LENGTH>      Password length (default: 64, ignored when --generate is not set) [default: 64]
  -F, --format <FORMAT>      Output format [default: text] [possible values: text, json]
  -h, --help                 Print help
  -V, --version              Print version

Commands:
  user        Manage a specific user
```

Before you add or edit users, copy the pgmoneta master key into the MCP home
directory:

```sh
mkdir -p ~/.pgmoneta-mcp
cp ~/.pgmoneta/master.key ~/.pgmoneta-mcp/master.key
chmod 600 ~/.pgmoneta-mcp/master.key
```

`pgmoneta-mcp-admin` uses `~/.pgmoneta-mcp/master.key` to encrypt passwords in
`pgmoneta-mcp-users.conf`, and the running `pgmoneta-mcp-server` must be able to
read that same file.

## user add

Add a user

Command

```sh
pgmoneta-mcp-admin user add
```

Example

```sh
pgmoneta-mcp-admin -f /etc/pgmoneta-mcp/pgmoneta-mcp-users.conf -U admin user add
```

## user edit

Update a user's password

Command

```sh
pgmoneta-mcp-admin user edit
```

Example

```sh
pgmoneta-mcp-admin -f /etc/pgmoneta-mcp/pgmoneta-mcp-users.conf -U admin user edit
```

## user del

Remove a user

Command

```sh
pgmoneta-mcp-admin user del
```

Example

```sh
pgmoneta-mcp-admin -f /etc/pgmoneta-mcp/pgmoneta-mcp-users.conf -U admin user del
```

## user ls

List all users

Command

```sh
pgmoneta-mcp-admin user ls
```

Example

```sh
pgmoneta-mcp-admin -f /etc/pgmoneta-mcp/pgmoneta-mcp-users.conf user ls
```
