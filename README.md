Local Development

`cargo watch -x run`

Dockerfile commands

`docker build -t IMAGE_NAME .`

`docker run -p 8080:8080 --name CONTAINER_NAME -t IMAGE_NAME`

`docker start CONTAINER_NAME`

docker-compose command(s) *Updated to only launch postgres for local development*

`docker-compose up --build`