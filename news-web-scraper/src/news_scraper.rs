use std::vec;

use colored::*;
use scraper::{ElementRef, Html};

#[derive(Debug)]
struct NewsHeadline {
    label: Option<String>,
    sublabel: Option<String>,
    headline: String,
}

#[derive(Debug)]
struct NewsSection {
    section_title: Option<String>,
    headlines: Vec<NewsHeadline>,
}

pub struct NewsScraper {
    news: Vec<NewsSection>,
}

impl NewsScraper {
    pub fn new() -> Self {
        NewsScraper { news: Vec::new() }
    }

    fn fetch_news_page(url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::blocking::get(url)?;
        response.text().map_err(Into::into)
    }

    fn scrape_headline(element: ElementRef) -> NewsHeadline {
        let label_selector = &scraper::Selector::parse(r#"span[data-testid='label']"#).unwrap();
        let labels = element.select(label_selector);
        let labels = labels.fold(Vec::<&str>::new(), |mut acc, label| {
            let label_text = label.text().collect::<Vec<&str>>();
            // add label_text to acc
            acc.extend(label_text);
            acc
        });

        let label = labels.first().map(|&label| label.into());

        let title_selector = &scraper::Selector::parse(r#"[data-testid='title']"#).unwrap();
        let titles = element.select(title_selector);
        let titles = titles.fold(Vec::<&str>::new(), |mut acc, title| {
            let title_text = title.text().collect::<Vec<&str>>();
            // add title_text to acc
            acc.extend(title_text);
            acc
        });
        let (sublabel, headline): (Option<String>, String) = if titles.len() > 1 {
            let (first, rest) = titles.split_first().unwrap();
            let sublabel = first.to_string();
            (Some(sublabel), rest.join(""))
        } else {
            (None, titles.join(""))
        };
        NewsHeadline {
            label,
            sublabel,
            headline,
        }
    }

    pub fn scrape(&mut self) -> Result<(), reqwest::Error> {
        let url = "https://www.independent.ie/";
        let page = Self::fetch_news_page(url)?;

        // Parse the page into a DOM tree
        let document = Html::parse_document(&page);

        // Get the main element
        let main_selector = scraper::Selector::parse("main").unwrap();
        let main_element = document.select(&main_selector).next().unwrap();

        // From the main element get the section elements which contain the attribute data-vr-zone *= "section"
        let section_selector =
            scraper::Selector::parse(r#"section[data-vr-zone*='section']"#).unwrap();
        let section_elements = main_element.select(&section_selector);

        let mut news_sections: Vec<NewsSection> = vec![];
        for section_element in section_elements {
            // Get the direct child nodes of this section_element
            let child_nodes = section_element.children();
            // for all the child nodes print the outer tag

            for child_node in child_nodes {
                // match if the child_node is a div
                if child_node.value().is_text() {
                    continue;
                }
                // assuming each child_node only has one label
                let mut news_section: Option<NewsSection> = None;
                let mut news_div_label: Option<String> = None;
                // the child_node is a div
                if let Some(div) = ElementRef::wrap(child_node) {
                    // From this div get the elements with attribute data-testid = "list-header"
                    let list_header_selector =
                        scraper::Selector::parse(r#"[data-testid='list-header']"#).unwrap();
                    let list_header_elements = div.select(&list_header_selector);
                    list_header_elements.for_each(|list_header_element| {
                        let headers = list_header_element.text().collect::<Vec<&str>>();
                        news_div_label = headers.first().map(|&list_header| list_header.into());
                    });

                    // From this div get the elements with attribute class = "eyebrow"
                    let eyebrow_selector = scraper::Selector::parse(r#".eyebrow"#).unwrap();
                    let eyebrow_elements = div.select(&eyebrow_selector);

                    eyebrow_elements.for_each(|eyebrow_element| {
                        let eyebrow = eyebrow_element.text().collect::<Vec<&str>>();
                        news_div_label = eyebrow.first().map(|&eyebrow| eyebrow.into());
                    });

                    let mut news_headlines: Vec<NewsHeadline> = vec![];

                    // From this div get the element ul with attribute data-testid = "article-teaser-list"
                    let article_teaser_list_selector =
                        scraper::Selector::parse(r#"ul[data-testid='article-teaser-list']"#)
                            .unwrap();
                    let article_teaser_list_elements = div.select(&article_teaser_list_selector);
                    article_teaser_list_elements.for_each(|article_teaser_list_element| {
                        let article_teaser_list_elements_selector = &scraper::Selector::parse(
                            r#"li[data-testid='article-teaser-list-item']"#,
                        )
                        .unwrap();
                        let article_teaser_list_item_elements = article_teaser_list_element
                            .select(article_teaser_list_elements_selector);
                        for li_item in article_teaser_list_item_elements {
                            news_headlines.push(Self::scrape_headline(li_item));
                        }
                    });

                    // From this div get the element div with attribute data-testid = "grid-teaser"
                    let grid_teaser_selector =
                        scraper::Selector::parse(r#"[data-testid='grid-teaser']"#).unwrap();
                    let grid_teaser_elements = div.select(&grid_teaser_selector);
                    grid_teaser_elements.for_each(|grid_teaser_element| {
                        news_headlines.push(Self::scrape_headline(grid_teaser_element));
                    });

                    if !news_headlines.is_empty() {
                        news_section = Some(NewsSection {
                            section_title: news_div_label,
                            headlines: news_headlines,
                        });
                    }
                }

                if let Some(news_section) = news_section {
                    news_sections.push(news_section);
                }
            }
        }
        if !news_sections.is_empty() {
            self.news = news_sections;
        };
        Ok(())
    }

    pub fn get_news(&self) -> String {
        let mut news = String::new();
        for news_section in &self.news {
            if let Some(section_title) = &news_section.section_title {
                news.push_str(&format!("{}\n", section_title.black().on_white().bold()));
            }
            for headline in &news_section.headlines {
                if let Some(label) = &headline.label {
                    news.push_str(&format!(" {} ", label.blue().bold()));
                }
                if let Some(sublabel) = &headline.sublabel {
                    news.push_str(&format!(" {} ", sublabel.white().on_red().bold()));
                }
                news.push_str(&format!("{}\n", headline.headline.cyan()));
            }
        }
        news
    }
}
