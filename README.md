<h1 align="center">Git Controller</h1>

<p align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Git-logo.svg/1280px-Git-logo.svg.png" alt="git" width=300 />
</p>

## Dependencies: _dev on your local machine_

- Others:

```bash
# sudo apt install -y protobuf-compiler libp
```

- ssh-keys-fuse:

```
sudo apt-get install fuse3
sudo apt-get install libfuse-dev pkg-config
```

### :warning: Fuse :warning:: DON'T forgot to unmount dir on your local machine

Use the following command to unmount the directory.

```bash
umount <PATH_TO_DIR>
```

## Getting Started

To deploy locally `Git Controller`, follow these steps:

1. Make sure you have [docker](https://docs.docker.com/engine/install/) and [docker compose](https://docs.docker.com/compose/install/) plugin installed on your machine.

2. Clone this repository to your local machine and navigate to the project directory.

```bash
git clone https://github.com/paastech-cloud/git-controller.git
cd git-controller
```

1. Build and start project.

```bash
docker compose up --build
```

## Usage

To enter the client container and access the command-line interface, use the following command:

```bash
docker exec -it git-controller-client-1 /bin/bash
```

From there, you can start using Git Controller's CLI to manage your Git repositories efficiently.