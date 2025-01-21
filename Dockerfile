FROM rustlang/rust:nightly-alpine AS builder
WORKDIR /usr/src/sixdegreesofapi
COPY . .
RUN apk update && apk add musl-dev
RUN cargo install --path .

FROM alpine:3
COPY --from=builder /usr/local/cargo/bin/sixdegreesofapi /usr/local/bin/sixdegreesofapi
CMD ["sixdegreesofapi"]
