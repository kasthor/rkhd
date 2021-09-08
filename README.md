# rkhd

## Goals

- Optimize the keyboard experience
- Faster switching back and forth from aplications and to the right application

## Current state

Hardcoded keyboard shortcuts that use yabai for some operations
it also throws some debugging information to the console

## Window Slots

It works like radio stations in old car radios, you set a window to a number, then
you get back to that window using the same number

It works with the numbers on the keyboard, fn + shift + N to set, fn + N to get to that window

## Shortcuts

```
shift + hyper + l : swap window to the righ
shift + hyper + h : swap window to the left

hyper + 0 : balance window sizes
hyper + f : toggle full screen

hyper + 1 : focus desktop 1
hyper + 2 : focus desktop 2
hyper + 3 : focus desktop 3

shift + fn + 1 : set window to slot 1
shift + fn + 2 : set window to slot 2
.
.
.
shift + fn + 9 : set window to slot 9

fn + 1 : focus window 1
fn + 2 : focus window 2
.
.
.
fn + 9 : focus window 9

```

## Disclaimer

The source code at this point is mostly for experimentation

## Prerequisites

- yabai
- I have Capslock mapped to F18 and that's my hyper key

## Motivation

I spend too much time in a computer for the reason that I do development mostly all day
I often try to bring better tools to my life so i can be more efficient during that time
Most of my frustration comes when I have to reach the mouse, the more I can do with the keyboard the better.
For this reason I use tools like Neovim and tmux for my development environment, it was good, the problem comes when I have to interact with the rest of the environment
On a Mac I know a few shortcuts to be moving around thru some of the windows, and switching spaces, which mostly helps, but I've been found frustrated from time to time trying to reach the window I need
So in order to fix this problem I've reached tools such as [yabai](https://github.com/koekeishiya/yabai) and [skhd](https://github.com/koekeishiya/skhd), also there is a tool called 'Snap' and 'Amethyst'
During my research I've previously found a guy on youtube that was sharing his setup and shared one interesting idea.
As stated before searching for windows is a mess, and the thing is that moslty all window managers are navigated in a "relative" manner, it would be better to have an absolute way to find the window with just a keyboard shortcut
For my last setup I was thinking about trying yabai with skhd, but well, I have no more modifier keys, and I'm affraid I will be having conflicts with other keyboard shortcuts, so I've seen that there is the "Hyper" key, but to set it up, skhd requires additional tools, and I just don't want to go that path, plus those tools appear not to be working in the latest versions of OSX
And well, I wanted to know if there was a better way to do it from skhd, but then I thought it could be an opportunity to learn rust, so here I am.
I'll attempt to create this project to help me finetune my multi-monitor desktop experience and my head is quite exploding, at first I will try to get something similar to skhd for the keyboard shortcuts, but really don't know where this is going to end, or else I might end up getting to a Linux machine instead
