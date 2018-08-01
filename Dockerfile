# Create build environment
FROM debian:stretch-slim as build

RUN apt-get update \
    && apt-get install -y \
        curl \
        build-essential \
        musl-tools \
    && rm -rf /var/lib/apt/lists/* \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path \
    && ~/.cargo/bin/rustup target add x86_64-unknown-linux-musl \
    && mkdir /build

COPY ./ /build
RUN cd /build \
    && ~/.cargo/bin/cargo build --target x86_64-unknown-linux-musl --release

# Setup service container
FROM scratch
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/otterbot /build/config.json /
EXPOSE 8001
WORKDIR /
CMD [ "./otterbot" ]
