FROM rust:latest

# Cargo init requires this.
ENV USER aioescrow_payment_server

# Set default to nightly
RUN rustup default nightly

# Copy the sources, fetch and build all dependencies
RUN mkdir -p /opt/server
RUN /bin/bash -c "cd /opt/server && cargo init"
COPY ./Cargo.toml /opt/server/Cargo.toml
RUN /bin/bash -c "cd /opt/server && \
                  cargo build --release"

# Copy over the actual code.
COPY ./entrypoint.sh /entrypoint.sh
COPY ./ /opt/server/
COPY ./Rocket.toml /opt/server/Rocket.toml

# Explicitly modify main.rs so it has to re-build main.rs
RUN echo " " >> /opt/server/src/main.rs

# Build it.
RUN /bin/bash -c "cd /opt/server && cargo build --release"

# Run entrypoint.sh
CMD ["/entrypoint.sh"]
