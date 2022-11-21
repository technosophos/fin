# Fin, v9

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.
* In v4, we added styling, the static file server, and a profile image.
* In v5, we added friends and the feed
* In v6, we added Redis support and in-browser editing
* In v7, we added output sanitization and HTTP Basic auth for the editor form
* In v8, we renamed the project from Finger to Fin

In this version, we'll add history

## The New Additions

Now that we have Redis, we can keep a history. Right now, there is not support in Spin for some of Redis' richer features, but even so we can easily store some history.

Now, instead of storing just one unstructured plan in Redis, we store a structured set of plans, with the newest one being first.

The new `history.hbs` provides the template for the `/history` endpoint. Much of the Redis internals were rewritten so that we could access the history and then select items out of the history.

Like the Feed feature we added in v5, this is gonna have issues as we scale up. So we'll address this in a future version.
