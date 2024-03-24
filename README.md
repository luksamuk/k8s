# K8S

Config for utilitaries I run on Kubernetes

## Motivation

This repository stores configuration files for Kubernetes which I use on my homelab
server. Normally they have some degree of utility, enough that I don't want to throw
it away, but they don't necessarily fit in any repository.

Notice also that *I use a k3s installation on Raspberry Pi 4*, so these files are
optimized and tested on an on-premise k3s instance with ARM64 processor.

## Contents

### rpi-config

Configuration for installing K3s on a cluster of Raspberry Pi boards.

I prefer using Raspberry Pi 4 Model B, with 8GB of RAM, and at least a 64GB SD Card (though external SSD storage would be far more trustworthy). Currently I have two of those, and a single one is a master of the network. They also communicate through my Tailnet on Tailscale, since my router sucks and tends to mess with my local connections.

Configuration is largely based on this one: https://dev.to/hatati/cook-up-a-k3s-cluster-on-raspberry-pies-with-ansible-4bb4

**You will also need to ensure that the user described on the inventory can use passwordless `sudo`.** Please notice that **you need to be wary of the security implications of this.** I recommend using the separate sudoer file approach:

```
# On a file such as /etc/sudoers.d/username:
username ALL=(ALL) NOPASSWD: ALL
```

> Remember that these are the files for MY local setup. You'll need to tweak the `inventory.yml` file if you want to spin your own.

To install K3s on all nodes:

```bash
ansible-playbook -i inventory.yml install.yml
```

To uninstall:

```bash
ansible-playbook -i inventory.yml uninstall.yml
```

This will create a `kubeconfig.yml` file which you should use for remote management from your own computer.

I also recommend installing [k9s](https://k9scli.io/), and maybe [Lens](https://k8slens.dev/).

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
- `postgres9.yml`: PostgreSQL database for testing, version 9.4.
  - `5433` (ClusterIP): Database port. Can only be accessed within the cluster. Use port forwarding to access locally.
- `rabbitmq.yml`: RabbitMQ message broker.
  - `30403`: AMQP protocol port.
  - `30404`: Management port.
  - `30405`: VHost port.
- `pgadmin.yml`: PgAdmin4 for accessing PostgreSQL databases within the cluster. Credentials are `admin@admin.com`, password `admin`.
  - `30406`: HTTP port for browser access.
- `ichef-api`: iChef API for chefs to share recipes with amateur cooks.
  - `30411`: HTTP port for API access.

### misc

Miscellaneous. Normally part of the misctools namespace.

- `misctools.yml`: Namespace definition.
- `minecraft-server.yml`: Minecraft Server. Created as a Spigot server with Geyser and
  Floodgate services to allow both Java and Bedrock clients.
  - `30201/TCP`: Java Server port.
  - `30201/TCP`: Query port.
  - `30202`: RCON port, connect to it with a tool such as mcrcon to enter server commands.
  - `30203/UDP`: Bedrock Server port through Geyser plugin.
- `browservice.yml`: Service for accessing the web from outdated browsers and OSes (such
  as Windows 98 with IE6). Please refer to the [project page](https://github.com/ttalvitie/browservice/) and [this other repository](https://github.com/vivlim/browservice-docker)
  for the Dockerfile. Notice that this .yml uses an ARM64 image!
  - `30406`: Connect to this port from your browser.
- `transmission`: uTorrent Server with GUI. I usually deploy this when I need to download a torrent to an external device.
  - `30500`: GUI, accessible from web browser.
  - `30501`: Actual service port.

### Examples

Code examples for some applications.

- `newsqldb`: Example queries and Rust project for using SurrealDB.

### iot

Stuff for IoT devices.

- `mosquitto`: Mosquitto MQTT message broker. Exposed on route `/mqtt` through an ingress controller (Traefik on K3s).
