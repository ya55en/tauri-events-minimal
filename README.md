# tauri-events-minimal

A Minimal Tauri events example to showcase an issue with pub/sub.

Created following [the official guide].

## Download

Simply clone this repository:

```
$ git clone https://github.com/yassen-damyanov/tauri-events-minimal.git
```

## Prerequisites

- POSIX OS (Won't run on Windows because of a symlink used.)
- rust + cargo
- node + npm
- yarn
- make

For rust + cargo and node + npm, see [Tauri's official installation docs],
(see at the bottom for OS-specific instructions).


## Initializaton

Enter the repository root and do:

```
$ make init
```

## Running in dev mode

Enter the repository root and do:

```
$ make dev
```

For a while you should see something like

```
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
   Compiling app v0.1.0 (/home/yassen/Work/public/tauri-events-minimal/src-tauri)
    Building [=======================> ] 379/380: app(bin)                                                         

```

 and then the app window should appear.

## Check how it works

- Open the dev tools of the app window (e.g. via right-click + 'Inspect Element')
- Choose the 'Console' tab on bottom
- Click "Send Event"

What appears on the console:

```
sendEvent() called
onUserClick() called
user-click
{"message": "Mmmm ... Tauri looks good!"}
```

which means that the front-end listener has intercepted the front-end `emit`
call, not the back-end one. The rust back-end (in `src-tauri/src/main.rs`)
does not seem to be involved at all.



[the official guide]: https://tauri.studio/en/docs/usage/guides/events
[Tauri's official installation docs]: https://tauri.studio/en/docs/getting-started/intro
