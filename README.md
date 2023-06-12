# Read me before installing
Follow the instructions carefully. This code corresponds to the rviewer challenge [Beer Tap Dispenser](https://rviewer.stoplight.io/docs/beer-tap-dispenser/juus8uwnzzal5-beer-tap-dispenser) . You can check this URL out to find out more about the specs and requirements for this endpoint.

This endpoint has been developed using RUST and [ROCKET] (https://rocket.rs/) .

## Setting up the development environment

NOTE: Make sure you config the .env file properly in order for this endpoint to run. .env file is not included in the initial commit, so you need to copy the values from env.dist into new .env file which is being ignored by git.

### Requirements

- In order for this environment to work, you need to have [rust](https://www.rust-lang.org/tools/install) installed (the compiler, not the videogame), as well as [docker desktop](https://www.docker.com/products/docker-desktop/)
-Check you have installed them by using the following commands in your terminal
`rustup --version`
`docker --version`

### Env Variables

- In order for this environment to work, you **must** set the .env file. To do this. make a copy of .env.dist file into .env and alter all of the variables you may need to suit your own. **By default, these parameters will work with the system defined in docker-compose.yml** 

### Docker
- You can use docker to spin up the different services neeeded for testing and development
`docker-compose up` will read the docker compose file docker-compose.yml and recreate the environments (mongo, redis).

## Building
- You must be connected to the internet the first time you run the build command.
In order to build the system, run the following command
`cargo build`

This will compile your application in to /target folder

## Running
- In order to run the project, you can use:
`cargo run`
Make sure you have a running redis instance and a running mongodb instance (via docker or by your own local machine). If you are not spinning the development environment using the docker-compose.yml file, **make sure that you have updated the appropiate strings and variables in the .env file**

## Testing
- Application can be tested via the following command.
`cargo test`

Note: Tests also include **integration tests**, hence in order to run all the tests properly, **you need to have the development services up and running. (Mongo and Redis)**. Make sure you run `docker-compose up` before you run the tests, or that your services are active.
