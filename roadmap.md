# Dots

## Interface

- `dots install`
    - should install & link all dependencies
- `dots uninstall`
    - should cleanly remove all dotfiles
- `dots doctor`
    - should check to see if all files & links are in place without taking any action
- `dots outdated`
    - should check to see if there are any upstream changes to any dots 
- `dots list`
    - should list all dot files by name
- `dots prefix <DOT>`
    - should return the folder of the specified dotfile

## Responsibilities

- should link `.bash_profile` & `.bashrc`
- should pull down velvet, link it, and install its plugins
- should pull down homebrew and run `brew Bundle` across all dots

## Install Process

1. download to `~/.dots/.tmp/`
2. read `Dot.json` package name
3. rename to `./.dots/{package.name}/`
4. link everything

## handling XDG

- Store source config in `~/dots` so links don't break when xdg changes
- store xdg info in `~/.dots/.dotsinfo`
- when an xdg variable changes, check to see if we had anything linked there before and relink.

## Inspiration

- http://thoughtbot.github.io/rcm/rcm.7.html

