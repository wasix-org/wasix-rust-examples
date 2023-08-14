# Independent.ie - News Scrapper

## Project Abstract

This project aims to build a scrapper for the Independent.ie website. The scrapper will be able to extract the following information from the website:

- Top Banner
- Main Article
- Articles in the "Latest News" section
- Top Stories
- Featured Stories
- Most Read Stories
- Life Magazine
- World News
- Etc.

This project will retrieve the information from the website and proxy it using a web server. The information will be returned in plain text format. And reader can use the CLI to read the headlines and articles.

## Project

The project is made in rust and consists of the following components:

- A web scrapper
- A web server for proxying the information

### Web Scrapper

Web scrapping rust can be done using the `scraper` crate. This crate allows us to extract information from the HTML document.

#### Extracting the information

##### Step 1: Get the HTML's main document

The main element of the website given by:

```html
<main role="main"></main>
```

has all the information we need.

##### Step 2: Subdivide the `main` element

The `main` element has the following sub sections:

```html
<section class="grid" data-vr-zone="section-1">...</section>
<section class="grid" data-vr-zone="section-1">...</section>
<section class="grid" data-vr-zone="section-3">...</section>
<section class="grid" data-vr-zone="section-4">...</section>
<section class="grid" data-vr-zone="section-5">...</section>
<section class="grid" data-vr-zone="section-6">...</section>
<section class="grid" data-vr-zone="section-7">...</section>
<section class="grid" data-vr-zone="section-8">...</section>
<section class="grid" data-vr-zone="section-9">...</section>
<section class="grid" data-vr-zone="section-10">...</section>
```

> Note: The `...` represents the content of the section.
> Note: The `data-vr-zone` attribute is used to identify the section.
> Note: The problem with the section and the second section is that they both share the same `data-vr-zone` attribute and `section-2` doesn't exist. Probably a mistake from the website.

##### Step 3: Extract the information from each section

Going further into each section we can find the following information:

```html
<div class="grid__col">...</div>
<div class="grid__col size-2-3--bp4">...</div>
<div class="grid__col size-1-3--bp4">...</div>
<div class="grid__col size-1-2--bp3">...</div>
<div class="grid__col size-1-2--bp3">...</div>
<div class="grid__col size-1-3--bp5">...</div>
<div class="grid__col size-1-3--bp5">...</div>
<div class="grid__col size-1-3--bp5">...</div>
<div class="grid__col size-1-2--bp4 size-1-4--bp5">...</div>
<div class="grid__col size-1-2--bp4 size-1-4--bp5">...</div>
<div class="grid__col size-1-2--bp4 size-1-4--bp5">...</div>
<div class="grid__col size-1-2--bp4 size-1-4--bp5">...</div>
<div class="grid__col">...</div>
```

> Note: The `...` represents the content of the section.

The first element can sometimes have the following structure:

```html
<h3 class="eyebrow">Life Magazine</h3>
```

> But the first `div` element won't have the above `h3` for some sections.

##### Step 4: Extract the information from each `div` element

Each `div` element has a `ul` tag which has a bunch of `li` tags. The `li` tags have the important information we need.
Further up, this `li` tag can also have a `span` with `data-testid` attribute set to **label** which specified the label of the article.

```html
<h3 data-testid="list-header">
  <span class="..."> Latest News</span>
</h3>
<ul class="..." data-testid="article-teaser-list">
  <!-- This is reluctant and would cause more nesting, just get article-teaser-list-item -->
  <li class="..." data-testid="article-teaser-list-item">
    <div class="...">
      <span data-testid="label">(... Label ...)</span>
      <!-- This is important if it does exist -->
    </div>

    <div>
      <h2 data-testid="title">
        <!-- This can also be h3/h4 anything don't rely on the h2 tag -->
        <!-- This contains a sublabel(optional) with the headline -->
        <span data-testid="sublabel"> (... Sublabel ...) </span>
        <span> (... Headline ...) </span>
      </h2>
      <time class="..." data-testid="timestamp">(... Time ...)</time>
      <!-- This is optional, but can be useful so need to show the time of a news it's published -->
    </div>
  </li>
</ul>
```

If the `<ul>` path doesn't go so then there's another form of articles which come under:

```html
<div class="..." data-testid="grid-teaser">
  <div class="...">
    <span data-testid="label">(... Label ...)</span>
    <!-- This is important if it does exist -->
  </div>
  <div>
    <h2 data-testid="title">
      <!-- This can also be h3/h4 anything don't rely on the h2 tag -->
      <!-- This contains a sublabel(optional) with the headline -->
      <span data-testid="sublabel"> (... Sublabel ...) </span>
      <span> (... Headline ...) </span>
    </h2>
    <time class="..." data-testid="timestamp">(... Time ...)</time>
    <!-- This is optional, but can be useful so need to show the time of a news it's published -->
  </div>
</div>
```

So the strategy to get the news is like:

1. Get the `main` element
2. Get the `data-vr-zone` section
3. Go recursively into each element
4. First check that the element has some kind of ``
5. If the element has a `grid-teaser` then go down the second role or `article-teaser-list` then get the information from the `li` element. but as soon as you get the information return from that function.

### Web Server

We use the `axum` web server to proxy the information. The server will be able to return the information in **plain-text** format.

```

```
