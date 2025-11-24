# Multi-stage build: compile Yew app to WASM with Trunk, then serve static files via nginx
FROM rust:1.81 as builder

WORKDIR /app

# Install Trunk and WebAssembly target
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch && rm -rf src

# Copy sources
COPY . ./

# Build optimized static bundle
RUN trunk build --release

FROM nginx:alpine as runtime
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/dist .

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
