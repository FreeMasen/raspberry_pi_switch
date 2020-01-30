docker build -t light-switch
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/light-switch -w /usr/src/ligth-switch rust:1.23.0 cargo build