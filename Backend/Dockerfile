# create a docker file for rust application that communicate with postgresql database
# build the docker image with the following command
FROM rust:latest

# create a new empty shell project
RUN apt-get update && apt-get install libpq-dev musl-tools -y

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

EXPOSE 32112

CMD ["cargo", "run"]