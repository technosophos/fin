# Finger, v5

* In v1, we created a server styled after the old UNIX `finger` server.
* In v2, we added support for HTML, rendering our markdown and using handlebars templates for layout.
* In v3, we added some structure to our finger data, and then write a JSON syndication endpoint at `/uc`.
* In v4, we added styling, the static file server, and a profile image.

## A False Start

Sometimes it is good to talk about a mistake. In this case, inspired by the original Finger's multi-user design, I started designing a system that would host multiple finger users on the same finger instance. But then I realized that forcing that design pattern didn't really match with the way I have been working. It would be peculiar to have to coordinate deployments of the same Spin project. So I gave up on that.

For now, and probably onward, our finger implementation will be one user per server.

## A New Hope

But this did get me to a better place. It is time to tackle syndication.

The goal of v5 is to make it possible for one user to "follow" other finger servers. For this first go, the feature will be a little dull. We'll be using a JSON file to track friends' feeds. But in some later version, perhaps we can make this better.

## The New Additions

First, take a look at `files/friends.json`. This is where we will add new friends. Unfortunately, right now any time you add a friend, you have to also edit `spin.toml` to allow the domain for your friend's server. Or if you are particularly daring, you can use `allowed_http_hosts = ["insecure:allow-all"]`.

Right now, the URL in `files/friends.json` must always end in a slash. I will fix this eventually.

The main page now lists friends, and links to the friends' instance. The new `/feed` page lists the latest plans from all your friends. Note that because of the way this URL is constructed, the URL in the `files/friends.json` MUST go to another Finger server.

The easiest way to test is to deploy an instance to Fermyon Cloud with `spin deploy`, and then set up that URL as a friend in your `files/friends.json`. Then you can locally `spin up` and have at least one friend.
