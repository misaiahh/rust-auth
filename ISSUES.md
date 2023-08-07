Docker Compose
 - Docker has limitied if no support for waiting for databases to be ready for connections by other containers.
 - Due to this I am removing the dev container and moving back to local development
 - This is mainly due to migrations needing to run while the db is ready to accept connections