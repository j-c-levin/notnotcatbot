FROM rust:1

# set CROSS_CONTAINER_IN_CONTAINER to inform `cross` that it is executed from within a container
ENV CROSS_CONTAINER_IN_CONTAINER=true

# install `cross`
RUN cargo install cross
