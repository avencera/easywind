# EasyWind [![CI](https://github.com/avencera/easywind/workflows/Mean%20Bean%20CI/badge.svg)](https://github.com/avencera/easywind/actions?query=workflow%3A%22Mean+Bean+CI%22)

## Install

Install using homebrew (mac and linux):

`brew install avencera/taps/easywind`

or

Install from a github release:

`curl -LSfs https://avencera.github.io/easywind/install.sh | sh -s -- --git avencera/easywind`

or 

Install from crates.io (if you already have Rust installed)

`cargo install easywind_cli`

or

Download a release directly from github: github.com/avencera/easywind/releases

## Usage

You only need to commands to get started (no dependencies not even node):

`easywind init mywebsite` will create a project in a new directory, with a `tailwind.config.js` file and an `index.html` file.
***NOTE:** This command will also download the [standalone tailwindcss cli](https://github.com/tailwindlabs/tailwindcss/releases) if you don't have node on your system.*

`easywind start mywebsite --open` will start the tailwind watcher and a live reloading server

https://github.com/avencera/easywind/assets/1775346/adf34522-404f-49d0-8fa3-2ac0d4731e05

### easywind init

```bash
Initialize a new project

Usage: easywind init <PROJECT_NAME>

Arguments:
  <PROJECT_NAME>
          Name of the project to initialize
          
          This will be used to create a directory with the same name (usage: easywind init portfolio)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### easywind start
```shell
Start the server and tailwind watcher

Usage: easywind start [OPTIONS] [ROOT_DIR]

Arguments:
  [ROOT_DIR]  [default: .]

Options:
  -p, --port <PORT>      Port the server shoud use, defaults to 3500 [default: 3500]
  -O, --open             Open in your browser
  -i, --input <INPUT>    Input css file to process
  -o, --output <OUTPUT>  Where you want the final CSS file to be written
  -h, --help             Print help
```

### easywind serve

```shell
Run a live reloading server to serve content

Usage: easywind serve [OPTIONS] [ROOT_DIR]

Arguments:
  [ROOT_DIR]  [default: .]

Options:
  -p, --port <PORT>  Port the server shoud use, defaults to 3500 [default: 3500]
  -o, --open         Open in your browser
  -h, --help         Print help
```
<img width="1680" alt="easywind_serve" src="https://github.com/avencera/easywind/assets/1775346/fa1f947a-480a-4991-a583-4d252f766769">

### easywind tailwind
```shell
Run the tailwind watcher that generates the CSS

Usage: easywind tailwind [OPTIONS] [ROOT_DIR]

Arguments:
  [ROOT_DIR]
          Path to the root directory of the project. This is where the `tailwind.config.js` file is located.
          
          Defaults to the current directory
          
          [default: .]

Options:
  -i, --input <INPUT>
          Input css file to process
          
          [default: src/app.css]

  -o, --output <OUTPUT>
          Where you want the final CSS file to be written
          
          [default: dist/app.css]

  -w, --watch
          Watch for changes in input CSS and recompile the output CSS

  -h, --help
          Print help (see a summary with '-h')

```
