==================
pgmoneta-mcp-admin
==================

---------------------------------------
Administration utility for pgmoneta-mcp
---------------------------------------

:Manual section: 1

SYNOPSIS
========

pgmoneta-mcp-admin [ OPTIONS ] <COMMAND>

DESCRIPTION
===========

pgmoneta-mcp-admin is an administration utility for pgmoneta-mcp.

OPTIONS
=======

-f, --file FILE
  The user configuration file

-U, --user USER
  The user name

-P, --password PASSWORD
  The password for the user

-g, --generate
  Generate a password

-l, --length LENGTH
  Password length (default: 64)

-F, --format FORMAT
  Output format (text, json)

-h, --help
  Print help

-V, --version
  Print version

COMMANDS
========

master-key
  Create or update the master key

user add
  Add a new user to configuration file

user del
  Remove an existing user

user edit
  Change the password for an existing user

user ls
  List all available users

REPORTING BUGS
==============

pgmoneta-mcp is maintained on GitHub at https://github.com/pgmoneta/pgmoneta-mcp

COPYRIGHT
=========

pgmoneta-mcp is licensed under the GNU General Public License v3.0

SEE ALSO
========

pgmoneta-mcp-server(1)
