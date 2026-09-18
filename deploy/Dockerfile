# Two stages: build dist/ with the Rust toolchain, then serve it from nginx.
# Build context is the repository root:  docker build -f deploy/Dockerfile .

FROM rust:1-slim AS build

# The site URL is baked in at compile time (canonical link + Open Graph).
ARG DEMO_SITE_URL=https://winged-demo.fly.dev
ENV DEMO_SITE_URL=${DEMO_SITE_URL}

WORKDIR /src

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl git \
    && rm -rf /var/lib/apt/lists/*

RUN curl -sSf https://rustwasm.github.io/wasm-pack/installer/init.sh | sh

COPY . .

# rust-toolchain.toml pulls the wasm32 target in on this first cargo invocation.
RUN ./scripts/build.sh

FROM nginx:alpine
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/dist /usr/share/nginx/html
EXPOSE 80
