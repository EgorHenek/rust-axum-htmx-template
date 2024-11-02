FROM rust:1 as build-env

WORKDIR /app

RUN curl -fsSL https://bun.sh/install | BUN_INSTALL="/usr/local" bash
ENV PATH="/usr/local/bin:$PATH"

COPY package.json bun.lockb /app/
RUN bun install

COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/bingo /

EXPOSE 3000
EXPOSE 3001

CMD ["./bingo"]
