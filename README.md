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

(The first run is going to take time, compiling some 380 crates.)

For a while you should see something like

```
.
.
Starting up http-server, serving www/
Available on:
  http://127.0.0.1:8000
  http://192.168.34.128:8000
Hit CTRL-C to stop the server
   Compiling proc-macro2 v1.0.27
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.73
   .
   . [compiling lots of modules]
   . 
   Compiling tauri-runtime v0.1.4
   Compiling tauri-runtime-wry v0.1.4
    Finished dev [unoptimized + debuginfo] target(s) in 2m 23s
     Running `target/debug/app`
** Tauri app about to start
[2021-07-20T16:58:14.504Z]  "GET /" "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15"
```

and then the app window should appear.

## Check how it works

- Open the dev tools of the app window (e.g. via right-click + 'Inspect Element')
- Choose the 'Console' tab on bottom to reveal the JS console
- Click "Send Event" in the application pane above

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
