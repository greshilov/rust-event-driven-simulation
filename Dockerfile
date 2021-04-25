# -- WASM Library --
FROM rust:1.51 as builder

ARG SDIR=red-simulation

# Installing wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown

# Instal&cache cargo dependencies
RUN cargo new --lib /app
COPY ./$SDIR/Cargo.toml ./$SDIR/Cargo.lock /app/

WORKDIR /app
RUN cargo fetch && rm -rf src

COPY ./$SDIR/ .
RUN wasm-pack build --release

# -- Frontend --
FROM node:15

ARG DDIR=red-demonstration
ARG SDIR=red-simulation

WORKDIR /app
COPY --from=builder /app/pkg /$SDIR/pkg

# Install&cache npm dependencies
COPY ./$DDIR/package-lock.json ./$DDIR/package.json /app/
RUN npm install

COPY ./$DDIR/ /app/
RUN npm run build
