use std::fmt;

use colored::*;
use scraper::{ElementRef, Html};

// Scraper for Hacker News
#[derive(Debug)]
struct NewsHeadline {
    headline: String,
    link: String,
    time: String,
    num_points: Option<u32>,
    num_comments: Option<String>,
    author: Option<String>,
}

impl fmt::Display for NewsHeadline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", self.headline.bold(),))?;
        f.write_str(&format!(
            "\n\t{} {} {} | {}",
            self.num_points
                .map(|num_points| format!("{} points", num_points))
                .map(|num_points| num_points.magenta())
                .unwrap_or_default(),
            self.author
                .clone()
                .map(|author| format!("by {}", author))
                .map(|author| author.bright_black())
                .unwrap_or_default(),
            self.time.dimmed(),
            self.num_comments
                .clone()
                .map(|num_comments| num_comments.to_string())
                .map(|num_comments| num_comments.green())
                .unwrap_or_default()
        ))?;
        f.write_str(&format!("\n\t{}", self.link.underline().blue()))?;
        Ok(())
    }
}

pub struct NewsScraper {
    news: Vec<NewsHeadline>,
}

impl NewsScraper {
    pub fn new() -> Self {
        NewsScraper { news: Vec::new() }
    }

    pub fn scrape(&mut self, page: String) {
        // Parse the page into a DOM tree
        let document = Html::parse_document(&page);

        // Get all the elements with class "athing"
        let athing_selector = scraper::Selector::parse(".athing").unwrap();
        let athing_elements = document.select(&athing_selector);

        // traverse the elements
        for athing_element in athing_elements {
            // on the athing element get the span with `titleline`
            let titleline_selector = scraper::Selector::parse(".titleline").unwrap();
            let titleline_element = athing_element.select(&titleline_selector).next().unwrap();
            // get the first anchor element
            let anchor_selector = scraper::Selector::parse("a").unwrap();
            let anchor_element = titleline_element.select(&anchor_selector).next().unwrap();

            // get the text of the anchor element
            let headline = anchor_element.text().collect::<Vec<_>>().join("");
            // get the href attribute of the anchor element
            let link = anchor_element.value().attr("href").unwrap().to_string();

            // get the next sibling of the athing element
            let athing_next_sibling = athing_element.next_sibling().unwrap();
            // convert the node to an element
            let athing_next_sibling_element = ElementRef::wrap(athing_next_sibling).unwrap();
            // get the span with class "age"
            let age_selector = scraper::Selector::parse(".age").unwrap();
            let time = athing_next_sibling_element
                .select(&age_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // get the text of the age element

            // get the score element
            let score_selector = scraper::Selector::parse(".score").unwrap();
            let num_points = athing_next_sibling_element
                .select(&score_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());
            // get the text of the score element

            // get the anchor element with class "hnuser"
            let hnuser_selector = scraper::Selector::parse(".hnuser").unwrap();
            let author = athing_next_sibling_element
                .select(&hnuser_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());

            // get the last child of subline element to get the number of comments
            let subline_selector = scraper::Selector::parse(".subline > a:last-child").unwrap();
            let num_comments = athing_next_sibling_element
                .select(&subline_selector)
                .next()
                .map(|elem| elem.text().collect::<String>());

            self.news.push(NewsHeadline {
                headline,
                link,
                time: time.unwrap(),
                num_points: num_points.map(|num_points| {
                    num_points
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                }),
                num_comments,
                author,
            });
        }
    }

    pub fn get_news(&self) -> String {
        let mut news = String::new();
        news.push_str(&format!(
            "\n{}\n",
            " Hacker News ".bold().on_bright_green().black()
        ));
        for (i, news_headline) in self.news.iter().enumerate() {
            news.push_str(&format!("\n{}. {}", i + 1, news_headline));
        }
        news
    }
}
