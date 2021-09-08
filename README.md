# SimpleSSG - A Simple Static Site Generator

SimpleSSG aims to bridge the gap between raw HTML and larger products like
Gatsby, Vue, etc. It aims to be as minimalistic and unobtrusive as possible,
feeling more like a wrapper around copy/paste than a heavy application. The
end-goal is to provide an easy way to create modular sites (eg. blogs, personal
webpages, etc.) in a way that is more minimal than Jekyll, but less effort than
Make.

One of the key features of this particular site generator is that it has no
system requirements. No additional python, javascript, perl, etc. is required.

This project is in early development and is not yet feature-complete.

### Installation

In the future there will be a variety of different installation methods, but as
of right now you will have to compile from source using `cargo build --release`.
This makes [Rust](https://www.rust-lang.org/) a temporary dependency (at least
for the build step).

### Usage

- `simple-ssg new $PROJECT_NAME` or `simple-ssg init` will set up a new project
  using the default template. You don't have to use this, but it might be
  helpful if setting things up for the first time.
- `simple-ssg build` will compile the project into a static collection of HTML
  files. - UNIMPLEMENTED

#### Project Structure

Each layout in `./layouts/` is a raw HTML file, with comments to tell the site
generator where you want your page content to go. Any pages using the layout
will be inserted directly into the HTML.

Each page in `./pages/` can either be a Markdown file (unimplemented) *or* an
HTML file, with optional metadata to describe which layout should be used, the
page title, etc. If unspecified, the generator will try to create the page
using `./layouts/default.html`, and will name the page according to the
filename.

Snippets in `./snippets/` are similar to pages, but contain no metadata. They
can be formatted as either HTML or Markdown, but unlike pages they can also be
inserted into any page *or* layout, and won't be turned into new pages when
building the site. The generator can optionally produce dynamic snippets with
information about your site, contents pages, etc. Note that as of right now
snippets can *not* contain other snippets. This is to prevent recursive calls.
