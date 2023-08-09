X Create dummy API endpoint

X Create Dockerfile for API

X Choose between MySQL and Postgres: Postgres due to it being better for enterprise apps. Why learn MySQL if it's not used for production grade business apps

X Create docker-compose for Postgres and Auth app

X Create volume mounts for both containers

X Update Dockerfile to use the container for development instead of local machine. Do we need a nodemon equivelent for Rust?

Implement Rust app code to talk to Postgres
  - X Create connection pool for database? Is this necessary as the auth app should not share the db with other apps?
  Create 
  - X table `RegistrationKeys`
  - table `Users`
  - X seed data for db

X Configure network for communication between app and db

X Create `Orchistration` project for monorepo
    X Move docker-compose here