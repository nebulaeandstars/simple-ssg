# SimpleSSG - A Simple Static Site Generator

### What is this?

SimpleSSG is a cross-platform utility that aims to bridge the gap between raw HTML and heavier web solutions like Gatsby, React, Vue, and the like. It also aims to be as minimalistic as possible, by allowing users to use as much or as little HTML as they would like. The end-goal is to provide an easy way to create modular sites (ie. blogs, personal webpages, etc.) in a way that is more minimal than Jekyll, but more abstract than Make.

Most importantly, all of this will work without any JavaScript!

### Install

We're currently not in any linux distribution, so you will have to compile from source using `cargo build --release`. You will need to have Rust installed on your system for this.

### How does it work?

SimpleSSG doesn't do much. If it helps, think of it as a wrapper around copy/paste. This gives *you* control over how you want *your* site to work and look.

First, run `simple-ssg init` (in an empty directory) or `simple-ssg new $YOUR_PROJECT_NAME` to create a new website, then look to the newly-created template files for examples.

#### Project Structure

Each layout in `./layouts/` is nothing more than a raw HTML file, with markers as comments to signify where you want your content to go. `simple-ssg` will not try to reference css or scripts for you, so do it here.

Each page in `./pages/` can either be a Markdown file *or* an HTML file, with optional metadata describing which layout should be used, the page title, etc. If there isn't any metadata available, SimpleSSG will try to create the page using `./layouts/default.html`.

Snippets in `./snippets/` work in exactly the same way as pages, in that they can be formatted as either HTML or markdown. In the future, you will be able to use snippets inside other snippets, but currently they can only be used inside layouts or pages. SimpleSSG can optionally generate information about your site, contents pages, etc. as snippets if you want them.
