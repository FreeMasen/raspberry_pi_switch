FROM rust:1.31

WORKDIR /usr/src/light-switch
COPY . .

RUN cargo install --path .

CMD ["light_switch"]