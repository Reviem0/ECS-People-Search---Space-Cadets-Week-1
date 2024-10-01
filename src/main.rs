use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    title: String,
    description: String,
    faculty: String,
    school: String,
}

impl Person {
    fn from_html(html: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let document = Html::parse_document(html);

        let name = Person::extract_meta_content(&document, "og:title")?;
        let description = Person::extract_meta_content(&document, "og:description")?;
        let faculty = Person::extract_meta_content(&document, "faculty_metatag")?;
        let school = Person::extract_meta_content(&document, "school_metatag")?;

        // Extracting title from h1
        let title_selector = Selector::parse("h1").unwrap();
        let title = document.select(&title_selector)
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("").to_string();

        Ok(Person {
            name,
            title,
            description,
            faculty,
            school,
        })
    }

    fn extract_meta_content(document: &Html, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let selector = Selector::parse(&format!("meta[property='{}'], meta[name='{}']", name, name)).unwrap();
        let content = document.select(&selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .ok_or_else(|| format!("Meta tag '{}' not found", name))?
            .to_string();
        Ok(content)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the email id of the person you want to search: ");
    // get the email id from the user input
    let mut email_id = String::new();
    std::io::stdin().read_line(&mut email_id).unwrap();

    let url = "https://www.ecs.soton.ac.uk/people/".to_owned() + &email_id;
    println!("{}",url);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    let person = match Person::from_html(&response) {
        Ok(person) => person,
        Err(e) => {
            eprintln!("Error: Invalid ID: \n {}", e);
            return Ok(());
        }
    };

    let serialized = serde_json::to_string_pretty(&person)?;
    println!("{}", serialized);

    Ok(())
}