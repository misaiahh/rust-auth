X Create dummy API endpoint
X Create Dockerfile for API
X Choose between MySQL and Postgres: Postgres due to it being better for enterprise apps. Why learn MySQL if it's not used for production grade business apps
Create docker-compose for Postgres and Auth app
Create volume mounts for both containers
Update Dockerfile to use the container for development instead of local machine. Do we need a nodemon equivelent for Rust?
Implement Rust app code to talk to Postgres
  Create connecion pool for database? Is this necessary as the auth app should not share the db with other apps?
  Create table `Registrations`
  Create seed data for db
Configure network for communication between app and db

Create `Orchistration` project for monorepo
    Move docker-compose here