# `binserve` :zap::crab:

A blazingly fast static web server with **routing**, **templating**, and **security** in a single binary you can set up with zero code. :fire:

<p align="left">
    <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" title="version" alt="version">
    <a href="https://github.com/mufeedvh/binserve/blob/master/LICENSE"><img alt="GitHub license" src="https://img.shields.io/github/license/mufeedvh/binserve.svg"></a>
    <a href="https://twitter.com/intent/tweet?text=Check%20this%20out!%20A%20blazingly%20fast%20static%20web%20server%20in%20a%20single%20binary%20you%20can%20set%20up%20with%20zero%20code.:&url=https%3A%2F%2Fgithub.com%2Fmufeedvh%2Fbinserve"><img alt="Twitter" src="https://img.shields.io/twitter/url/https/github.com/mufeedvh/binserve.svg?style=social"></a>
</p>

> **Update:** [v0.2.0 Draft](https://github.com/mufeedvh/binserve/issues/29)

## Table of Contents

* [Features](#features)
* [Quick Start](#hello-world)
* [Configuration](#%EF%B8%8F-configuration-file)
* [Templates](#-templates)
* [Build From Source](#build-from-source)
* [Security](#security)
* [Contribution](#contribution)
* [License](#license)
* [Support The Author](#liked-the-project)

----

## Features

- **Single binary with no dependencies and everything built-in.**
- **Blazingly fast!** :zap: - it's built on top of [**Actix**](https://actix.rs/), one of the [**fastest web frameworks**](https://www.techempower.com/benchmarks/) in the world and of course, written in **Rust**.
- **Everything in a single config file** - everything you need to setup is in the `binserve.json`, just change it, run it!
- **Epic Portability** - just carry the binary around and all you have to change is the `host` and `port` in the `binserve.json`.
- **Easiest Routing** - you just have to enter the `route` and the `static file` to respond in the `binserve.json` to add a route entry!
- **[Handlebars](https://github.com/sunng87/handlebars-rust) template engine** - renders every static file with `Handlebars` on the first run improving performance!
- **Template Variables in one place** - that too in the one and only config file!
- **Secure by design** - runs security validation checks on the first run and will only run the server once configuration is confirmed secure! (See [Security](https://github.com/mufeedvh/binserve#security))
- **Config & Static File Assistance** - just running it will create the configuration file and the static directory boilerplate for you!
- **Supports Any Static Files** - just give any static file of your choice to routes and response will match it's `Content-Type`.
- **Straightforward Directory Structure** - static files falls under the `static` directory, you can even change that! And `images`, `css`, and `js` falls under the `assets` directory which should all be created for you in the first run itself!
- **Custom Error Page Support** - you can design your own fancy error pages!
- **Actix Logging Middleware** - Logging is powered directly from **Actix**.

## Hello World!

Download the binary for your OS from [**Releases**](https://github.com/mufeedvh/binserve/releases), then just run it:

    $ binserve

That's it. Done! You should see the following output:

```                        
         _   _                         
        | |_|_|___ ___ ___ ___ _ _ ___ 
        | . | |   |_ -| -_|  _| | | -_|
        |___|_|_|_|___|___|_|  \_/|___| v0.1.0
    

Your server is up and running at http://example.com:80/
```

Here is how the directory structure will look like:

```
├── binserve
├── binserve.json
├── rendered_templates
│   ├── 404.html
│   └── index.html
└── static
    ├── 404.html
    ├── assets
    │   ├── css
    │   ├── images
    │   └── js
    └── index.html
```

### ⚙️ Configuration File:

📄 **File:** `binserve.json`

```json
{
  "directory_listing": false,
  "enable_logging": true,
  "error_pages": {
    "404": "404.html"
  },
  "follow_symlinks": false,
  "routes": {
    "/": "index.html",
    "/example": "example.html"
  },
  "server": {
    "host": "127.0.0.1",
    "port": 1337
  },
  "static_directory": "static",
  "template_variables": {
    "load_static": "/static/",
    "name": "Binserve"
  }
}
```

The whole thing revolves around this configuration file, whatever changes you want to make, just edit the config and run it!

### 🎨 Templates:

`binserve` uses [Handlebars](https://github.com/sunng87/handlebars-rust) as the template engine as it perfectly fits our use case.

**Here is an example:**

```html
<html>
    <head>
        <title>Example</title>
    </head>
    <body>
        <h1>My name is {{name}}</h1>
    </body>
</html>
```

Now add your name in the config file (`binserve.json`) as a `template variable`:

```json
"template_variables": {
    "load_static": "/static/",
    "name": "Keanu Reeves"
}
```

Now run the server!

    $ binserve

This would render down to:

```html
<html>
    <head>
        <title>Example</title>
    </head>
    <body>
        <h1>My name is Keanu Reeves</h1>
    </body>
</html>
```

To load static files such as `images`, `css`, and `javascript`, just use `{{load_static}}`:

`load_static` is specified in the `binserve.json` itself.

```html
<img src="{{load_static}}images/rick_roll.gif">
<link rel="stylesheet" href="{{load_static}}css/main.css">
<script src="{{load_static}}js/script.js">
```

`binserve` renders all your template at once on the first run itself to improve performance as it wouldn't have to render the template on each request.

## Build From Source

For building binserve from source, you need to have these tools installed

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/mufeedvh/binserve.git
$ cd binserve/
$ cargo build --release
```

The first command clones the binserve repository in your local machine. The next two commands changes into the binserve directory and builds it in release mode

## Security

Security is one of the most crucial elements in a web server, `binserve` is secure by design. **Here is how it's secure:**

- Routes are specified in the configuration file not directly accepted from the user.
- Runs a check for **Path Traversal** attempts in routes in the configuration file on each run.
- Runs a check for **Symlink Files** which might point to sensitive files on each run.
- Only **Follows Symlinks** when explicitly allowed in the configuration file which is disabled by default.
- Only enables **Directory Listing** when explicitly allowed in the configuration file which is disabled by default.

## Contribution
Ways to contribute
- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word
- Create an example with binserve and you will be featured here!

## License
Licensed under the MIT License, see <a href="https://github.com/mufeedvh/binserve/blob/master/LICENSE">LICENSE</a> for more information.
