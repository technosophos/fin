# Finger, v3

In v1, we created a server styled after the old UNIX `finger` server.
In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.

In v3, we're going to drift back a little bit closer to the original spec: We're going to add some structure to the finger data. We're also going to add a new endpoint that delivers serialized data. This sets the foundation for a later version.

## The New Additions

To add support for structured data, and for an eventual syndication method, we're going to encode some of our data as JSON.

For starters, we are going to express our finger data like this:

```
{
    "username": "technosophos", // Required
    "realname": "Matt Butcher", // Required
    "location": "Boulder, CO",
    "description": "I Fermyon all day, everyday.",
    "links": { // A dictionary of site name, URL
        "GitHub": "https://github.com/technosophos",
        "LinkedIn": "https://linkedin.com/in/mattbutcher"
    }
}
```

The above is roughly inspired by the definition of `{C}` in [RFC 1288](https://www.rfc-editor.org/rfc/rfc1288), but updating it to be more like Our Favorite Social Media profiles.

The new endpoint will be named "/uc". This is a little nod to the original finger specification, where a `{U}{C}` query returned both the finger record and the plan. For us, the result will be serialized as JSON data. (And the plan will be delivered as markdown because doing so poses less of a security risk).
