# Finger

When the world was new, dinosaurs roamed the earth, and Mosaic was not yet a thing, UNIX folks had a nifty little social media tool called `finger`. The `finger` RFC 742 was written in 1977, the year Star Wars came out (and the year I was born). Later, a more complete [RFC 1288](https://www.rfc-editor.org/rfc/rfc1288) replaced the original spec.

On any given server, one could type the command `finger username@servername` (or just `finger username`) and get back something that looked like this:

```console
$ finger -ls
Login: technosophos   			Name: Matthew Butcher
Directory: /Users/technosophos      	Shell: /bin/zsh
On since Mon Sep 19 14:17 (MDT) on console, idle 59 days 4:27 (messages off)
Last login Tue Oct 18 20:03 (MDT) on ttys001
No Mail.
Plan: Every day I'm shufflin'
```

(Believe it or not, the above is from my current mac, which still has `finger` installed by default!)

Finger messages are broken into two main parts:

1. The information about the user. Mostly, this was system-derived.
2. The user's "plan" from their `.plan` file. In my experience, the `.plan` was used more like a tweet than an actual expression of a plan.

With the uncertainty brewing at the Large Bird Social Media Factory, I thought it might be a fun time to riff on the Finger protocol and build a simple little social media program.

This is not at all an implementation of the protocol, just a cute tool inspired by the experience of using finger and `.plan` files.

## Writing This App

To get started, I did:

```
$ spin new http-rust finger
$ cd finger
```

I decided that initially I would take the easiest possible approach:

* The route `/finger` would get the user information
* The route `/plan` would get just the plan
* The default `/` route would get the finger and plan info

While I am not trying to follow the finger protocol, format, or even the list of recommended fields, I do like the idea of having the `finger` part return personal info and the `plan` part returning a little user-supplied thought nugget.

In this initial implementation, I also figured I'd just start with a file-based version that reads a `finger.md` file for the user info and a `plan.md` for the plan. Why `.md`? Because it seems like a plan file should allow some minimal markup, and Markdown is the most popular these days.

## Running the App

If you're ready to try things out, edit the `files/finter.md` and `files/plan.md` to say whatever you want, then start up the server:

```console
$ spin build --up
Executing the build command for component finger: cargo build --target wasm32-wasi --release
    Finished release [optimized] target(s) in 0.12s
Successfully ran the build command for the Spin components.
Serving http://127.0.0.1:3000
Available Routes:
  finger: http://127.0.0.1:3000 (wildcard)
```

You should be able to test out all three of the routes by pointing your browser of choice to:
- http://127.0.0.1:3000
- http://127.0.0.1:3000/finger
- http://127.0.0.1:3000/plan
