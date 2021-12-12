# Prerequisites

The complete setup is containerized and needs the following prerequisites to run

* Linux Server
* Installed Docker and Docker Compose (there is also the possibility to use Podman)
* Installed OpenSSL

# Getting started

Clone the SkyRocket GitHub repo by running

`git clone https://github.com/ProjectExchange/SkyRocket.git`

You need the `docker` and `scripts` folder on your host.

# Generate the certificates

Since we want to encrypt our data in transit as well as at rest as we further explained [here](DbEncryption.md), we start bei generating our TLS certificates by executing the following command

```sh
cd scripts && chmod +x *.sh && ./install.sh
```

This will generate all your certificates and secrets.

## Permission issues

The container mapping might cause permission issues with the private key files. Those permission issues must be resolved manually. For quick development setup the permission conflicts can be resolved by running `chmod 644 $(find certs -name *-key.pem)`.

**Note that this is not allowed in production use!**

# Pulling a container image

When using the GitHub registry within a private repository, the container images can only be accessed using a personal access token. To generate such a token , go to `Settings > Developer Settings > Personal access tokens` and create a new one with the scope `read:packages`.

You can simply execute `docker login ghcr.io`, specify your GitHub email address and use the generated token as password and docker should return a success message. You can now easily pull the container images provided by the SkyRocket repository.

Run `docker-compose up -d` to start the containers.
