# CRM Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fcrm)](https://examples.yew.rs/crm)

A very shallow customer relationship management tool.
It consists of a list of clients with an associated name and description.
It's possible to create new clients and clear the list entirely.

## Concepts

This example features multiple views ("scenes") which the user can switch between.

For a much more sophisticated approach check out [`yew-router`](https://yew.rs/concepts/router/).
One major flaw with the implementation used by this example is that the scenes aren't tied to the URL.
Reloading the page always brings the user back to the initial scene.

The example also uses the [`gloo::storage`](https://gloo-rs.web.app/docs/storage)
to persist the clients across sessions.

## Improvements

- Improve the presentation of the example with CSS.

### Features

- Edit a client
- Remove individual clients from the list
- Pagination
