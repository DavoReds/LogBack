FROM rust:1-alpine AS builder

WORKDIR /app
ENV SQLX_OFFLINE=true
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static-debian12 AS runner

WORKDIR /app
ADD public /app/public
COPY --from=builder /app/target/release/logback .
ENV LOGBACK_LOG=info
EXPOSE 80
CMD ["./logback"]
