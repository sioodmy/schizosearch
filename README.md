# Schizosearch

<p align="center">
  <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sioodmy/schizosearch/rust.yml">
  <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sioodmy/schizosearch/check.yml?label=nix flake">
  <img alt="GitHub Sponsors" src="https://img.shields.io/github/sponsors/sioodmy">
</p>
  
Minimal, privacy friendly meta search engine

# Features 
  - ❔ Your IP remains a secret to big tech search engines
  - 👥 Results mixed with many others, making it harder to deanonymize you
  - 💾 Zero logs
  - ⛔ Zero caching
  - ♻️ Results from some of the most popular indexes
  - 📓 Special results
  - 🐦 Zero Javascript policy, super lightweight frontend
  - 🦀 Written in Rust
  - ❄️ NixOS module for ease of deployment

# Do you keep logs or cache?
**No**

Yeah, thats it. I don't log any of your information. Many others, self-proclaimed "privacy respecting" search engines
have a statement like "uhmm, actually we keep a temporary cache to split results into pages 🤓".

Do you really need to split 3kB page into 10 different html files? If your browser can handle rendering super complex and muh bloated sites like [literally anything from archwiki or sth](https://wiki.archlinux.org/title/GRUB), then it can probably handle schizosearch. Why do we always have to either pretend we're in the 90s or make a whole operating system out of a simple page?

# Instances 
- [search.sioodmy.dev (testing)](https://search.sioodmy.dev)

# Credits
- [hnhx](github.com/hnhx)

# TODO
- [x] image results
- [ ] tor
- [x] nixos module
- [x] unit testing
- [ ] remove all placeholders
- [x] handle duplicate results
- [x] use hashmap for results
- [x] convert unfree garbage to libre alternatives

engines
  - [x] duckduckgo
  - [x] brave
  - [ ] yandex
  - [x] qwant
  - [ ] 1337x.to
  - [ ] sourcegraph

special results:
  - [x] Calculator
  - [x] Dictionary
  - [ ] Wikipedia (wikiless)
  - [ ] Brave snippets
  - [ ] Stackoverflow 
