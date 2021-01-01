# Monkey programming language [![CircleCI](https://circleci.com/gh/danimal141/monkey-rs.svg?style=svg&circle-token=1a711b6ee9e8f4f0c2372cc6d55bb9ec44b6c9f8)](https://circleci.com/gh/danimal141/monkey-rs)

This is my monkey programming language project inspired by [Writing An Interpreter In Go](https://interpreterbook.com/).

```
　　　　＿ツ)_
　　 _／　　　＼_
　　(/ ／⌒⌒＼∧)
　　｜((●　●))｜
　　人∧　||　/ 人
　 /　 (　‥　)　∧
　｜　 ヽ二二ノ　 ｜
　∧　＼　　　／　∧
 (　＼　＼　／　／　)
　＼へ＼ミ)(ミ／へ／
　 (ミ　￣)(￣　ミ)
```

## Dependencies
- Docker

## How to develop
### Build / Run
- `bin/run-container cargo build`
- `bin/run-container cargo run`

### Lint
- `bin/run-container cargo clippy --`

If you want the build to fail when encountering warnings, please use `bin/run-container cargo clippy -- -D warnings`.

### Format

- `bin/run-container cargo fmt -- --check`

### Test
- `bin/run-container cargo test`
