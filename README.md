<img src="https://raw.githubusercontent.com/mufeedvh/binserve/master/assets/binserve-logo.png" alt="binserve logo" width="250" align="right">

# `binserve` :rocket::crab:

A [**fast**](#benchmarks) static web server with **TLS** (HTTPS), **Routing**, **Hot Reloading**, **Caching**, **Templating**, and **Security** in a single-binary you can set up with zero code.

Built from the ground up for self-hosters with [performance](#benchmarks), [ease of use](#configuration), and [portability](#portability) in mind. ❤️

<p align="left">
    <img src="https://img.shields.io/badge/version-0.2.0-blue.svg" title="version" alt="version">
    <a href="https://github.com/mufeedvh/binserve/blob/master/LICENSE"><img alt="gitHub license" src="https://img.shields.io/github/license/mufeedvh/binserve.svg"></a>
    <a href="https://twitter.com/intent/tweet?text=Check%20this%20out!%20A%20fast%20static%20web%20server%20in%20a%20single%20binary%20you%20can%20set%20up%20with%20zero%20code.:&url=https%3A%2F%2Fgithub.com%2Fmufeedvh%2Fbinserve"><img alt="twitter share" src="https://img.shields.io/twitter/url/https/github.com/mufeedvh/binserve.svg?style=social"></a>
</p>

## Table of Contents

* [Features](#features)
* [Hello World!](#hello-world)
* [Installation](#installation)
* [Build From Source](#build-from-source)
* [Configuration](#configuration)
* [Templates](#templating)
* [Benchmarks](#benchmarks)
* [FAQ](#faq)
* [Contribution](#contribution)
* [License](#license)
* [Credits](#credits)

**Example:** [Hosting a website produced by a Static Site Generators like Hugo, Zola, Jekyll, Hexo, etc.](#static-site-generators)

# Features

- **Fast**: Binserve is [designed](#caching) to be performant, this is thanks to [**Actix-Web**](https://github.com/actix/actix-web) - one of the [fastest](https://www.techempower.com/benchmarks/) web frameworks out there and [**DashMap**](https://github.com/xacrimon/dashmap) for handling routes and the cache storage. (See [**Benchmarks**](#benchmarks))
- **Portability:** Binserve is cross-platform and portable to any major operating system, like it can run on your [Android](#portability) phone!
- **Routing:** Routing is simply matching a URI path to a file or a directory in a JSON file. (See [**Configuration**](#configuration))
- **Templating:** You can write templates and partials using [Handlebars](https://handlebarsjs.com/guide/). (See [**Templating**](#templating))
- **Hot Reload:** You can reload your configuration (routes) and static files with no downtime.
- **Caching:** Binserve's performance is achieved due to minimization of Disk I/O operations at runtime (with `fast_mem_cache` enabled) and serving static files from memory. On the client-side, `Cache-Control`, `Etag`, and `Last-Modified` are utilized.
- **Security:** Prevents common attack vectors like [Directory Traversal](https://en.wikipedia.org/wiki/Directory_traversal_attack) and [Symlink Attacks](https://capec.mitre.org/data/definitions/132.html).

# 👋 Enterprise?

If you're deplyoing to production or expecting high traffic to your server, get [binserve+](https://mufeedvh.gumroad.com/l/binserveplus) which has **DDoS Protection**, **Rate Limiting**, and **Prometheus Metrics** for monitoring along with all the above features built-in.

Checkout <a href="https://mufeedvh.gumroad.com/l/binserveplus">Binserve Plus</a>!

Read [**FAQ**](#faq) for more details.

## Hello World!

Download the executable for your OS from [**Releases**](https://github.com/mufeedvh/binserve/releases), then just run it:

```sh
mkdir mywebsite/
binserve
```

On the first run, it will create the configuration file and a starter boilerplate for you to get started.

```
 _   _
| |_|_|___ ___ ___ ___ _ _ ___
| . | |   |_ -| -_|  _| | | -_|
|___|_|_|_|___|___|_|  \_/|___| 0.2.0

[INFO] Build finished in 295 μs ⚡
[SUCCESS] Your server is up and running at 127.0.0.1:1337 🚀
```

Go to http://127.0.0.0:1337/ and you will be greeted with the index page of Binserve.

Now all you need to do is to edit the `binserve.json` file. (See [**Configuration**](#configuration)).

## Installation

Download the executable from [**Releases**](https://github.com/mufeedvh/binserve/releases) OR Install with `cargo`:

```sh
cargo install --git https://github.com/mufeedvh/binserve.git
```

[Install Rust/Cargo](https://rust-lang.org/tools/install)

### Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```sh
git clone https://github.com/mufeedvh/binserve.git
cd binserve/
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Configuration

The configuration file is a JSON file called `binserve.json` that's generated automatically by the executable. Configuring binserve is pretty straight-forward because the configuration fields are self-explanatory:

And all of the values here have secure defaults so you don't have to specify the ones you don't need.

💡 **TIP**: Most probably you wouldn't be needing all of the configuration fields, checkout the [Static Site Generator example](#static-site-generators) on how to serve a single directory.

```json
{
    "server": {
        "host": "127.0.0.1:1337",
        "tls": {
            "host": "127.0.0.1:443",
            "enable": false,
            "key": "key.pem",
            "cert": "cert.pem"
        }
    },

    "routes": {
        "/": "public/index.html",
        "/usage": "public/usage.hbs",
        "/blog": "public/blog/"
    },

    "static": {
        "directory": "public/assets",
        "served_from": "/assets",
        "error_pages": {
            "404": "public/404.html"
        }
    },

    "template": {
        "partials": {
            "header": "public/header.hbs"
        },
        "variables": {
            "app_name": "Binserve"
        }
    },

    "config": {
        "enable_hot_reload": true,
        "fast_mem_cache": true,
        "enable_cache_control": true,
        "enable_directory_listing": true,
        "minify_html": false,
        "follow_symlinks": false,
        "enable_logging": false
    },

    "insert_headers": {
        "x-greetings": "hellooo!"
    }
}
```

You can override the configuration with command-line arguments as well:

<ul>
  <li><code>-c/--cert</code> - The path to the TLS certificate for your domain.</li>
  <br>
  <li><code>-k/--key</code> - The path to the TLS key for your domain.</li>
  <br>
  <li><code>-h/--host</code> - The host/domain with the specified port for your webserver to run on.</li>
  <br>
  <ul>
    <li>Example: <code>--host 127.0.0.1:1337</code> OR <code>--host zombo.com</code></li>
  </ul>
</ul>

## TLS

There is built-in support for TLS:

```json
{
    "server": {
        "host": "127.0.0.1:1337",
        "tls": {
            "host": "127.0.0.1:443",
            "enable": true,
            "key": "key.pem",
            "cert": "cert.pem"
        }
    }
}
```

The key and certificate can be generated with `openssl`:

```shell
# generate pkcs#10 key+cert (PEM):
$ openssl req -x509 -newkey rsa:4096 -keyout key_pkcs10.pem -out cert.pem -sha256 -days 36

# convert the private key to PKCS#8 (PEM):
$ openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in key_pkcs10.pem -out key.pem
```

## Templating

Binserve uses [Handlebars](https://github.com/sunng87/handlebars-rust) for templating as it's simple and the most commonly known templating engine.

You can register partial templates and template variables like this in the configuration file:

```json
"template": {
    "partials": {
        "header": "public/header.hbs"
    },
    "variables": {
        "app_name": "Binserve"
    }
}
```

**public/header.hbs**:

```hbs
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ app_name }} v0.2.0 Usage</title>
    <link rel="stylesheet" href="assets/css/styles.css">
</head>
```

And use it like this:

```hbs
<html>
    {{> header}}
    <body>Hello World!</body>
</html>
```

Which would render down to:

```hbs
<html>
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Binserve v0.2.0 Usage</title>
        <link rel="stylesheet" href="assets/css/styles.css">
    </head>
    <body>Hello World!</body>
</html>
```

## Static Site Generators

Every static generator builds your Markdown/Template files into a directory, usually named `public/`, all you have to do is point that directory as the index route:

```json
{
    "server": {
        "host": "127.0.0.1:1337",
        "tls": {
            "host": "127.0.0.1:443",
            "enable": false,
            "key": "key.pem",
            "cert": "cert.pem"
        }
    },
    "routes": {
        "/": "public/"
    }
}
```

That's it!

As mentioned previously, you don't have to specify all the fields, secure defaults will be applied! 🙌

Pointing directories as routes is an intentional design so that you can host multiple Static Site Generator outputs easily. Let's say you have a portfolio as the homepage and your blog is made with a different template. You can just do:

```json
"routes": {
    "/": "my_zola_portfolio/public/",
    "/blog": "my_hugo_blog/public/"
}
```

## Portability

Binserve is cross-platform which means you can run it on any major operating system / devices. It is low on CPU usage and memory footprint so you can run it on your **Raspberry Pi** or even your **Android** Phone:

<div align="center">
    <table>
        <tr><td><img src="https://raw.githubusercontent.com/mufeedvh/binserve/master/assets/binserve-android.gif" width="200"></td></tr>
    </table>
</div>

## Caching

With `fast_mem_cache` enabled, all files are stored in-memory mapped to it's route index and response on initialization and will not perform any disk reads at runtime. It is recommended to keep it enabled even if you have hundreds of files, Binserve automatically sorts large files to be read from disk. Only when you are serving lots of large media content you should disable this mode.

Under the hood, binserve maps the routes and prepares the response with the file's content, `mime` type, `metadata`, and the required fields to derive the `Etag` and `Last-Modified` tags beforehand and will not perform any of these operations at runtime. Upon changes to any of these files, hot reload is executed in a background thread which updates the program state by making changes to the [concurrent hashmap](https://github.com/xacrimon/dashmap) which manages the cache, changes are instant and wouldn't cause any downtime either.

## Benchmarks

<div align="center">
  <table>
    <tr><td><img src="https://raw.githubusercontent.com/mufeedvh/binserve/master/assets/benchmarks.jpeg" width="500"></td></tr>
  </table>
</div>

See [**BENCHMARKS.md**](BENCHMARKS.md)

## FAQ

**Q:** What is binserve+?

> Binserve+ is made for websites meant to run in production and handle high amounts of traffic. It comes with DDoS Protection, Rate Limiting, and Prometheus Metrics out-of-the-box.
>
> You get lifetime license + lifetime bug fixes for $24/once.
>
> This exists as a way to support the project, it does not have any license keys or verification system, you get pre-compiled executables for major operating systems and architectures in a ZIP archive. (MIT License)
>
> [Get binserve+](https://mufeedvh.gumroad.com/l/binserveplus).

## Contribution

Ways to contribute:

- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word

## License

Licensed under the MIT License, see <a href="https://github.com/mufeedvh/binserve/blob/master/LICENSE">LICENSE</a> for more information.

## Credits

Binserve wouldn't exist without these amazing projects:

- [**actix-web**](https://actix.rs/) - Binserve runs on top of actix-web, the performance wouldn't be achievable without it.
    - As well as: [**actix-web-labs**](https://github.com/robjtede/actix-web-lab) by [@robjtede](https://github.com/robjtede).
- [**dashmap**](https://github.com/xacrimon/dashmap) - The in-memory file cache is stored using dashmap for high concurrency reads.
- [**ahash**](https://github.com/tkaitchuck/aHash) - aHash is the hashing algorithm used for the dashmap.
- [**compact_str**](https://github.com/ParkMyCar/compact_str) - A memory efficient string type that can store up to 24* bytes on the stack. Route index keys are stored as [`CompactString`](https://docs.rs/compact_str/0.4.0/compact_str/struct.CompactString.html)s.
- [**handlebars-rust**](https://github.com/sunng87/handlebars-rust) - This library is used for the Handlebars templating and rendering.
- [**jwalk**](https://github.com/jessegrosjean/jwalk) - This library helps binserve to index/walk directories quickly in parallel.
- [**minify-html-one-pass**](https://github.com/wilsonzlin/minify-html/tree/master/rust/onepass) - Fast HTML minification library, helps to drastically reduce rendering times for reloading/saving hundreds of files.
- [**notify**](https://github.com/notify-rs/notify) - Hot reloading depends on this library to watch for filesystem events and update the program state in realtime.
- [**once_cell**](https://github.com/matklad/once_cell) - A lazy static assignment library, this helps in managing the global program state under an "RwLock" implementation of dashmap.
- [**parking_lot**](https://github.com/Amanieu/parking_lot) - Binserve uses parking_lot's [Mutex](https://docs.rs/parking_lot/latest/parking_lot/type.Mutex.html#differences-from-the-standard-library-mutex) implementation for the global configuration state.
- [**rustls**](https://github.com/rustls/rustls) - The TLS implementation used by Binserve is written in pure Rust and this eliminates the need for OpenSSL.
- [**serde**](https://github.com/serde-rs/serde) - The serialization framework used by Binserve for managing the configuration settings, feels like magic and extremely good documentation.

Thank you! :heart:

---
