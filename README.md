# K8S

Config for utilitaries I run on Kubernetes

## Motivation

This repository stores configuration files for Kubernetes which I use on my homelab
server. Normally they have some degree of utility, enough that I don't want to throw
it away, but they don't necessarily fit in any repository.

Notice also that *I use a k3s installation on Raspberry Pi 4*, so these files are
optimized and tested on an on-premise k3s instance with ARM64 processor.

## Contents

### dev

Development tools. Normally part of the devtools namespace.

- `devtools.yml`: Namespace definition.
- `interlisp.yml`: Medley Interlisp-D.
  - `30200`: VNC connection port.
- `sonarqube.yml`: SonarQube for my projects. Normally is fed by GitHub Actions joining
  my Tailscale net as ephemeral nodes.
  - `30100`: HTTP/REST port.
- `surrealdb.yml`: SurrealDB NewSQL database.
  - `30101`: Server port. Uses `root` user/pass.
- `mariadb.yml`: MariaDB database.
  - `30400`: Database port. User: `minerva`, pass: `mysql`.
- `phpmyadmin.yml`: phpMyAdmin tool.
  - `30401`: Port for accessing phpMyAdmin.
  - *NOTE:* When attempting to access MariaDB, use the IP of the cluster
	as reference, from your computer's standpoint.
- `postgresql.yml`: PostgreSQL database for testing.
  - `30402`: Database port. User/pass: `postgres`.


### misc

Miscellaneous. Normally part of the misctools namespace.

- `misctools.yml`: Namespace definition.
- `minecraft-server.yml`: Minecraft Server. Created as a Spigot server with Geyser and
  Floodgate services to allow both Java and Bedrock clients.
  - `30201`: Java Server port.
  - `30202`: RCON port, connect to it with a tool such as mcrcon to enter server commands.
  - `30203` (UDP only): Bedrock Server port through Geyser plugin.


### Examples

Code examples for some applications.

- `newsqldb`: Example queries and Rust project for using SurrealDB.

