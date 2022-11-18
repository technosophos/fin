# Finger, v2

In v1, we created a server styled after the old UNIX `finger` server. Now let's start making some changes to modernize our app.

First up, let's emit HTML instead of markdown text.
To accomplish this, we'll add the following:

- A template library to make it possible to customize the HTML "wrapper"
- A markdown processor to convert the raw text to HTML

## The New Additions

Using `pulldown-cmark`, we'll add markdown processing support. And using `handlebars`, we'll add template support. This also requires a little bit of refactoring so that we can efficiently generate HTML pages instead of plain text.

The `files` directory now has more than just `finger.md` and `plan.md`:

```
files
├── finger.md
├── plan.md
└── templates
    ├── finger.hbs
    ├── index.hbs
    └── plan.hbs
```

Each of the `hbs` files has a template for rendering one of our three HTTP endpoints.
