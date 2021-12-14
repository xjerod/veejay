# veejay

[![build](https://github.com/jrbury/veejay/actions/workflows/build.yml/badge.svg)](https://github.com/jrbury/veejay/actions)

Veejay (/ˈvējā/) is a toy library for editing videos

### Inspiration

Right now it's mostly just an attempt to port Devon Crawford's [library](https://github.com/DevonCrawford/Video-Editing-Automation) he made for a [video](https://www.youtube.com/watch?v=0ZeO0IQaJ-A).

## Phases

### One

The first phase was just hooking up ffmpeg and opening the file and dumping metadata on it.

### Two

The second phase was to add a clip object that let's you modify the state of, such as start/end points.
