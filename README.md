<h1 align="center">Git Controller</h1>

<p align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Git-logo.svg/1280px-Git-logo.svg.png" alt="git" width=300 />
</p>

## Dependencies

```
sudo apt update && sudo apt upgrade -y
# sudo apt install -y protobuf-compiler libp
sudo apt-get install fuse3
sudo apt-get install libfuse-dev pkg-config
```

## Getting Started

To run Git Controller locally, follow these steps:

1. Make sure you have Docker installed on your machine.

2. Clone this repository to your local machine.

```bash
git clone https://github.com/your-username/git-controller.git
```

3. Navigate to the project directory.

```bash
cd git-controller
```

4. Build and start the Docker containers.

```bash
docker compose up --build
```

## Usage

To enter the client container and access the command-line interface, use the following command:

```bash
docker exec -it git-controller-client-1 /bin/bash
```

From there, you can start using Git Controller's CLI to manage your Git repositories efficiently.

## License

This project is licensed under the MIT License.