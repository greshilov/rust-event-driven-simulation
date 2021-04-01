FROM rust:1.50 as builder

# Installing wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown

# Instal&cache cargo dependencies
RUN cargo new --lib /app
COPY Cargo.toml Cargo.lock /app/

WORKDIR /app
RUN cargo fetch && rm -rf src

COPY src src
RUN wasm-pack build

FROM node:15
WORKDIR /app
COPY --from=builder /app/pkg /pkg

# Install&cache npm dependencies
COPY ./demo/package-lock.json ./demo/package.json /app/
RUN npm install

COPY ./demo/ /app/
RUN npm run build
