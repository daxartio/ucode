use chrono::{DateTime, Datelike, Duration, Local};
use lsp_types::CompletionItemKind;

pub fn get_items() -> Vec<lsp_types::CompletionItem> {
    let date: DateTime<Local> = Local::now();
    let mut items = Vec::new();

    items.push(lsp_types::CompletionItem {
        label: "today".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some(date.format("%Y-%m-%d").to_string()),
        ..Default::default()
    });

    items.push(lsp_types::CompletionItem {
        label: "tomorrow".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some((date + Duration::days(1)).format("%Y-%m-%d").to_string()),
        ..Default::default()
    });

    items.push(lsp_types::CompletionItem {
        label: "yesterday".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text: Some((date - Duration::days(1)).format("%Y-%m-%d").to_string()),
        ..Default::default()
    });

    for day in &[
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ] {
        items.push(lsp_types::CompletionItem {
            label: format!("this-{}", day),
            kind: Some(CompletionItemKind::SNIPPET),
            insert_text: Some(
                day_of_week(date, day.parse().unwrap())
                    .format("%Y-%m-%d")
                    .to_string(),
            ),
            ..Default::default()
        });
        items.push(lsp_types::CompletionItem {
            label: format!("next-{}", day),
            kind: Some(CompletionItemKind::SNIPPET),
            insert_text: Some(
                next_day_of_week(date, day.parse().unwrap())
                    .format("%Y-%m-%d")
                    .to_string(),
            ),
            ..Default::default()
        });
    }

    for i in 1..=7 {
        items.push(lsp_types::CompletionItem {
            label: format!("{}days-ago", i),
            kind: Some(CompletionItemKind::SNIPPET),
            insert_text: Some((date - Duration::days(i)).format("%Y-%m-%d").to_string()),
            ..Default::default()
        });
        items.push(lsp_types::CompletionItem {
            label: format!("{}days-from-now", i),
            kind: Some(CompletionItemKind::SNIPPET),
            insert_text: Some((date + Duration::days(i)).format("%Y-%m-%d").to_string()),
            ..Default::default()
        });
    }

    items
}

fn day_of_week(date: DateTime<Local>, weekday: chrono::Weekday) -> DateTime<Local> {
    let mut date = date;
    while date.weekday() != chrono::Weekday::Mon {
        date = date - Duration::days(1);
    }
    while date.weekday() != weekday {
        date = date + Duration::days(1);
    }
    date
}

fn next_day_of_week(date: DateTime<Local>, weekday: chrono::Weekday) -> DateTime<Local> {
    let mut date = date;
    date = date + Duration::days(7);
    while date.weekday() != chrono::Weekday::Mon {
        date = date - Duration::days(1);
    }
    while date.weekday() != weekday {
        date = date + Duration::days(1);
    }
    date
}
