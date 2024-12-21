<h1 align="center">glazy</h1>

<p align="center">
  GitLab CLI for the lazy ones.
  <br><br>
  <a href="https://github.com/dj95/glazy/actions/workflows/lint.yml">
    <img alt="clippy check" src="https://github.com/dj95/glazy/actions/workflows/lint.yml/badge.svg" />
  </a>
  <a href="https://github.com/dj95/glazy/releases">
    <img alt="latest version" src="https://img.shields.io/github/v/tag/dj95/glazy.svg?sort=semver" />
  </a>

  <br><br>
  The goal of this CLI is to speed up the interaction with GitLab and your local terminal environment.
  It provides ways to easily find and clone remote repositories to the local machine with some extra
  actions.
</p>

## 📦 Requirements

- cargo (for building glazy)
- git (for cloning the repositories)

## 🚀 Installation

Clone this repository and run `cargo build -r` within it. Then copy the binary from `./target/release/glazy` into one of the directories of your `$PATH` (like `/usr/loca/bin`).

After copying the binary, create a config file at `$HOME/.config/glazy/config.kdl`.

```javascript
// Configuration related to the GitLab instance
gitlab {
    // URL to the GitLab instance
    host "url_val"

    // Personal Access Token with at least api & write_repository permissions
    token "token_val"
}

// Configuration for the local environment
local {
    // Directory, where the project tree should start
    project_dir "/Users/daniel/Developer"
}
```

The `url` must point to your GitLab instance without the protocol! `token` must be a personal access token with `api` &
`read_repository` permissions.

Then go ahead and run `glazy help` for getting familiar with the CLI interface.

## 🔨 Usage

### Find and clone repository

```bash
glazy open

# for starting at a certain group with all subgroups
glazy open mygroup
```

## 🔮 Future Features

- [ ] refresh/pull all local repository
- [ ] initial bootstrap of multiple repositories via a layout file

## 🤝 Contributing

If you are missing features or find some annoying bugs please feel free to submit an issue or a bugfix within a pull request :)

## 📝 License

© 2024 Daniel Jankowski

This project is licensed under the MIT license.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
