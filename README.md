# Finger, v7

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.
* In v4, we added styling, the static file server, and a profile image.
* In v5, we added friends and the feed
* In v6, we added Redis support and in-browser editing

But our handling of things in v6 was NOT secure. In this release, we will start to clean up our security.

## The New Additions

The following additions are needed for security:

* We need to sanitize all Markdown
* We need to protect the plan form from unauthorized access. We do that with HTTP Basic auth right now
* Currently, all of the properties in the finger do not allow HTML, and we rely on handlebars to escape.

At this point, there are two env vars you will need to set:

So your spin commandline will look something like this:

```
$ spin build --up -e REDIS_HOST='redis://redis.example.com:14908' -e ADMIN_PASSWORD=password
```

> Since this is a single user system right now, any username is allowed when prompted for HTTP basic auth. Only password is checked.
