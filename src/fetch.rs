use crate::database_conn::*;
use crate::diesel::ExpressionMethods;
use crate::models;
use std::iter::Iterator;

const ASCII_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn get_url(term: &str) -> String {
    format!(
        "https://www.syriacdictionary.net/index.cgi?langsel=en&word={}",
        term
    )
}

fn sanitize_suryoyo(suryoyo: &mut String) {
    for each in ASCII_LETTERS.chars() {
        *suryoyo = suryoyo.replace(each, "");
    }
}

pub fn fetch_term_results(conn: SuryoyoTranslateDb, term: &str) -> serde_json::Value {
    let res = translations
        .filter(translation_keyword.eq(term))
        .get_result::<models::TranslationEntry>(&*conn);

    match res {
        Ok(value) => serde_json::to_value(value).unwrap(),
        Err(_) => serde_json::to_value(
            diesel::insert_into(translations)
                .values(NewTranslation {
                    translation_keyword: term.to_owned(),
                    translation_result: Some(fetch_term_results_http(term).unwrap()),
                })
                .get_result::<models::TranslationEntry>(&*conn)
                .unwrap(),
        )
        .unwrap(),
    }
}

fn fetch_term_results_http(term: &str) -> Result<serde_json::Value, ureq::Error> {
    let res = ureq::get(&get_url(term)).call()?.into_string()?;
    let selector = scraper::Selector::parse(r#"div[class="message"]"#).unwrap();
    let soup = scraper::Html::parse_document(&res);
    let message = soup.select(&selector).next();
    if let Some(_) = message {
        // The tag was found, probably Result not found?
        let error = format!("No match found for {}", term);
        return Ok(serde_json::json!({
            "error": error,
            "result": {}
        }));
    }
    let mut output = serde_json::json!({
        "error": null,
        "results": {}
    });
    let records_selector = scraper::Selector::parse(r#"div[id="recordContainer"]"#).unwrap();
    let records = soup.select(&records_selector);
    for record in records {
        let number_selector = scraper::Selector::parse(r#"div[id="recordnr"]"#).unwrap();
        let number_b_selector = scraper::Selector::parse(r#"b"#).unwrap();

        let num = record
            .select(&number_selector)
            .next()
            .unwrap()
            .select(&number_b_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let table = record
            .select(&scraper::Selector::parse(r#"table[class="bbttaabbllee"]"#).unwrap())
            .next()
            .unwrap();
        let mut current_num = serde_json::json! {{}};
        let mut english_last = String::new();
        for rule in table.select(&scraper::Selector::parse(r#"tr"#).unwrap()) {
            let english = rule
                .select(&scraper::Selector::parse(r#"td"#).unwrap())
                .next()
                .unwrap();
            let suryoyo = rule
                .select(&scraper::Selector::parse(r#"td[class="sy"]"#).unwrap())
                .next();

            if let None = suryoyo {
                continue;
            }
            let suryoyo = suryoyo.unwrap();
            let english_content = english.text().collect::<String>();
            let english_content = english_content.trim();
            let mut suryoyo_contents = suryoyo.text().collect::<String>();
            sanitize_suryoyo(&mut suryoyo_contents);
            let suryoyo_contents = suryoyo_contents.trim();

            if suryoyo_contents.is_empty() {
                continue;
            }

            if !english_content.is_empty() {
                english_last = english_content.to_owned();
            }
            if current_num.get(english_last.clone()).is_none() {
                current_num
                    .as_object_mut()
                    .unwrap()
                    .insert(english_last.clone(), serde_json::json! {[]});
            }

            current_num[english_last.clone()]
                .as_array_mut()
                .unwrap()
                .push(serde_json::Value::String(String::from(suryoyo_contents)));
        }
        output
            .get_mut("results")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(num, current_num);
    }
    Ok(output)
}
