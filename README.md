# Finger, v4

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.

In v4, we're going to focus on the UI. Namely, we're going to use [Bootstrap](https://getbootstrap.com/docs/5.2/getting-started/introduction/)
to style things up a bit.

## The New Additions

Styling is mainly done in the templates, which we added in v2. So we'll spend more time in HTML and CSS than we will in Rust code. Though in the name of ease, we will make a few changes to our template renderer.

### Changes to the Rust Code

To make things more flexible and pretty, there are a few changes to the code:

* I added an `image` field to the finger
* Instead of loading individual templates, we now load the entire `/files/templates` directory

### Adding a new Wasm module

To serve static files, the `modules/spin_static_fs.wasm` static file server has been added, and static files are found in the `static/` directory

The static file server comes from the [Spin Fileserver](https://github.com/fermyon/spin-fileserver) project on GitHub.

### Styling Away!

The rest of the work in this project has been editing the `hbs` files in `files/templates`.

* Larger templates are broken into partials for reuse
* Lots of HTML has been added
* Mostly, I used Bootstrap styles, but custom styles are in `static/style/style.css` (served by the Spin Fileserver).
