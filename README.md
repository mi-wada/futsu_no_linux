# For ふつうのLinuxプログラミング

Reading log: <https://zenn.dev/mitsuaki/scraps/34131a819624ac>

## Run and enter Container

```zsh
docker image build -t futsu-no-linux .
docker run -v ./:/app --rm -it futsu-no-linux bash
```

## Run

```zsh
# You can find COMMAND_NAME from Cargo.toml [[bin]].name
cargo make run COMMAND_NAME
```

## Run test

```zsh
cargo make test
```
