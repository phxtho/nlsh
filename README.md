# nlsh
Natural Language Shell leveraging GPT-3, inspired by [this demo](https://vimeo.com/427943407/98fe5258a7)

## Setup

Add your OpenAI Api Key to the dotenv file
```sh
cp .env.example .env
```


## Build
Run

```sh
cargo build --release
```
Binary can be found in the `target/release` folder

## Usage
```
nlsh display my current directory
# Outputs
# " ls -l"
```