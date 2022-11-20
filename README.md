# Finger, v6

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.
* In v4, we added styling, the static file server, and a profile image.
* In v5, we added friends and the feed

Now it's time to really go off the rails!

To this point, all of our data has been stored in flat files within the app. That means updating the plan requires you to redeploy the app. What if we stored the plan in Redis, though? Then we could edit the plan from anywhere!

> The main feature is the ability to edit your plan file in-browser, with the results stored in (and fetched from) Redis. Note, however, that this version (v6) should NOT be used in production because there is no concept of a user account or login. Anyone can edit the plan.

## The New Additions

To get started, I created a free Redis account at Redis Labs' website. That way, I can use `spin deploy` to deploy my app, but I can also test it locally. Setting the `REDIS_HOST` env var to the `redis://` URL will get you going.

> NOTE: Make sure you do not embed your username and password into a `spin.toml` and then check that file into VCS.

This new change adds the `redis.rs` library.

The new `/plan/edit` page adds a form for editing your plan. It accepts markdown.

To edit your plan, go to the `/plan` endpoint (`Plan` in the menu) and click `edit` or go straight to `/plan/edit`.

Once more... DO NOT USE THIS IN PRODUCTION! In v7, we will start adding security.
