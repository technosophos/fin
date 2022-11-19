Long ago, in a galaxy far, far away, it was 1977. _Star Wars: A New Hope_ was in theaters. And in the UNIX world, a new protocol had been standardized. Finger ([RFC 742](https://www.rfc-editor.org/rfc/rfc742)) described what I like to think of as the "Internet's first social media."

Finger allowed users to use a command line program, `finger`, to look up information about other users on their network. Along with seeing basic information like username, real name, and phone number, one could also get a glimpse into a user's _plan_. A plan (or `.plan`) was a free-form smallish chunk of text (like the one you are currently reading) that a user could plunk down in their `$HOME` directory. The `finger` server would then add that text to the user's finger information. Here's an example:

```
$ finger technosophos
Login: technosophos                     Name: Matthew Butcher
Directory: /Users/technosophos          Shell: /bin/zsh
On since Mon Sep 19 14:17 (MDT) on console, idle 60 days 22:29 (messages off)
On since Tue Oct 18 20:03 (MDT) on ttys001, idle 1:25
No Mail.
Plan:
Every day I'm shufflin'

This is a longer plan. Plans can be multi-line. Though it's frowned upon to be too verbose.
```

Now, these days most of that info is considered either sensitive or pointless. But the idea is cool. So I started writing a Spin app called `finger` that pays homage to this original form of social media.

---

I'm currently working on styling this page with [Bootstrap](https://getbootstrap.com/), a style framework created by Twitter long ago. I like it because I can hack together a decent looking site without much work.

I'm also working on templates using hte [Handlebars](https://handlebarsjs.com/) language. There is a great Rust library that supports Handlebars syntax, so I have been using that.
