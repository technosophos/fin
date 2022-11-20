# Fin, v8

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.
* In v4, we added styling, the static file server, and a profile image.
* In v5, we added friends and the feed
* In v6, we added Redis support and in-browser editing
* In v8, we added output sanitization and HTTP Basic auth for the editor form

For this version, we're only going to do one thing: Rename the project from `Finger` to `Fin`.
This is a pun. "Fin" refers to an appendage on aquatic animals like fish. But "Fin" is both half of "Finger", and the Latin word for "The End" (not that there's any social commentary implied there on the implosion of a certain bird-themed social media platform).

## The New Additions

Renamed the top-level name to Fin
