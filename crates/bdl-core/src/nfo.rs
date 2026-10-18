//! Portable movie NFO metadata. Configuration and source facts are task snapshots.
use crate::{BdlError, BdlResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NfoMetadata {
    pub title: String,
    pub part_title: String,
    pub description: String,
    pub genre: String,
    pub uploader: String,
    pub publish_date: String,
    pub duration_seconds: Option<u64>,
    pub bvid: String,
    pub cid: Option<u64>,
    pub aid: Option<u64>,
    pub owner_mid: Option<u64>,
    pub part_index: usize,
    pub index: usize,
    pub source_title: String,
    pub date: String,
    pub quality: String,
    pub codec: String,
    pub source_url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NfoExtraFields {
    pub sort_title: String,
    pub original_title: String,
    pub tagline: String,
    pub outline: String,
    pub genres: String,
    pub tags: String,
    pub collection: String,
    pub collection_overview: String,
    pub directors: String,
    pub writers: String,
    pub studios: String,
    pub countries: String,
}

impl NfoExtraFields {
    fn templates(&self) -> [&str; 12] {
        [
            &self.sort_title,
            &self.original_title,
            &self.tagline,
            &self.outline,
            &self.genres,
            &self.tags,
            &self.collection,
            &self.collection_overview,
            &self.directors,
            &self.writers,
            &self.studios,
            &self.countries,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NfoOptions {
    pub title_template: String,
    pub plot_template: String,
    pub include_plot: bool,
    pub include_date: bool,
    pub include_runtime: bool,
    pub include_genre: bool,
    pub include_source: bool,
    pub include_cover: bool,
    pub uploader_as_actor: bool,
    pub custom_tags: String,
    pub extra: NfoExtraFields,
}

impl Default for NfoOptions {
    fn default() -> Self {
        Self {
            title_template: "{title}".into(),
            plot_template: "{description}\nUP 主：{uploader}\n{source_url}".into(),
            include_plot: true,
            include_date: true,
            include_runtime: true,
            include_genre: true,
            include_source: true,
            include_cover: true,
            uploader_as_actor: false,
            custom_tags: String::new(),
            extra: Default::default(),
        }
    }
}

pub const VARIABLES: &[&str] = &[
    "title",
    "video_title",
    "part_title",
    "description",
    "uploader",
    "publish_date",
    "bvid",
    "cid",
    "source_url",
    "aid",
    "owner_name",
    "owner_mid",
    "part_index",
    "index",
    "source_title",
    "collection_title",
    "date",
    "quality",
    "quality_label",
    "codec",
    "genre",
    "duration_seconds",
];

fn expand(template: &str, value: impl Fn(&str) -> String) -> BdlResult<String> {
    let mut result = String::new();
    let mut chars = template.chars();
    while let Some(ch) = chars.next() {
        if ch == '}' {
            return Err(invalid("NFO 模板存在多余的 }"));
        }
        if ch != '{' {
            result.push(ch);
            continue;
        }
        let mut name = String::new();
        let mut closed = false;
        for ch in chars.by_ref() {
            if ch == '}' {
                closed = true;
                break;
            }
            name.push(ch);
        }
        if !closed || !valid_variable(name.trim()) {
            return Err(invalid("NFO 模板变量未知或未闭合"));
        }
        result.push_str(&value(name.trim()));
    }
    Ok(result)
}

fn valid_variable(name: &str) -> bool {
    if VARIABLES.contains(&name) {
        return true;
    }
    matches!(
        name.split_once(':'),
        Some(("date" | "publish_date", "%Y-%m-%d" | "%Y.%m.%d" | "%Y%m%d"))
            | Some(("part_index" | "index", "02" | "03" | "04"))
    )
}

fn invalid(message: &str) -> BdlError {
    BdlError::Planning {
        message: message.into(),
    }
}

impl NfoOptions {
    pub fn validate(&self) -> BdlResult<()> {
        if self.title_template.trim().is_empty() {
            return Err(invalid("NFO 标题模板不能为空"));
        }
        expand(&self.title_template, |_| String::new())?;
        if self.include_plot {
            expand(&self.plot_template, |_| String::new())?;
        }
        for template in self.extra.templates() {
            expand(template, |_| String::new())?;
        }
        Ok(())
    }
}

pub fn render(
    options: &NfoOptions,
    metadata: &NfoMetadata,
    task_title: &str,
    cover: Option<&str>,
) -> BdlResult<String> {
    options.validate()?;
    let source_url = if !metadata.source_url.is_empty() {
        metadata.source_url.clone()
    } else if metadata.bvid.is_empty() {
        String::new()
    } else {
        format!("https://www.bilibili.com/video/{}", metadata.bvid)
    };
    let raw_value = |name: &str| match name {
        "title" => task_title.to_owned(),
        "video_title" => metadata.title.clone(),
        "part_title" => metadata.part_title.clone(),
        "description" => metadata.description.clone(),
        "uploader" | "owner_name" => metadata.uploader.clone(),
        "publish_date" => metadata.publish_date.clone(),
        "bvid" => metadata.bvid.clone(),
        "cid" => metadata.cid.map(|v| v.to_string()).unwrap_or_default(),
        "source_url" => source_url.clone(),
        "aid" => metadata.aid.map(|v| v.to_string()).unwrap_or_default(),
        "owner_mid" => metadata
            .owner_mid
            .map(|v| v.to_string())
            .unwrap_or_default(),
        "part_index" => {
            if metadata.part_index > 0 {
                metadata.part_index.to_string()
            } else {
                String::new()
            }
        }
        "index" => {
            if metadata.index > 0 {
                metadata.index.to_string()
            } else {
                String::new()
            }
        }
        "source_title" | "collection_title" => metadata.source_title.clone(),
        "date" => metadata.date.clone(),
        "quality" => metadata.quality.clone(),
        "quality_label" => crate::naming::quality_label(&metadata.quality).to_owned(),
        "codec" => metadata.codec.clone(),
        "genre" => metadata.genre.clone(),
        "duration_seconds" => metadata
            .duration_seconds
            .map(|v| v.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    };
    let value = |name: &str| {
        let (key, format) = name.split_once(':').unwrap_or((name, ""));
        let raw = raw_value(key);
        if raw.is_empty() || format.is_empty() {
            return raw;
        }
        if matches!(key, "date" | "publish_date") {
            return chrono::NaiveDate::parse_from_str(&raw, "%Y-%m-%d")
                .map(|d| d.format(format).to_string())
                .unwrap_or_default();
        }
        format!(
            "{:0>width$}",
            raw,
            width = format.parse::<usize>().unwrap_or(0)
        )
    };
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<movie>\n");
    let mut tag = |name: &str, text: &str| {
        if !text.trim().is_empty() {
            xml.push_str(&format!("  <{name}>{}</{name}>\n", escape_xml(text.trim())));
        }
    };
    let title = expand(&options.title_template, value)?;
    tag(
        "title",
        if title.trim().is_empty() {
            task_title
        } else {
            &title
        },
    );
    for (name, template) in [
        ("originaltitle", &options.extra.original_title),
        ("sorttitle", &options.extra.sort_title),
        ("tagline", &options.extra.tagline),
        ("outline", &options.extra.outline),
    ] {
        tag(name, &expand(template, value)?);
    }
    if options.include_plot {
        tag("plot", &expand(&options.plot_template, value)?);
    }

    if options.include_date
        && chrono::NaiveDate::parse_from_str(&metadata.publish_date, "%Y-%m-%d").is_ok()
    {
        tag("premiered", &metadata.publish_date);
        tag("year", &metadata.publish_date[..4]);
    }
    if options.include_runtime
        && let Some(seconds) = metadata.duration_seconds.filter(|seconds| *seconds > 0)
    {
        tag("runtime", &seconds.div_ceil(60).to_string());
    }
    for (name, source, template) in [
        (
            "genre",
            if options.include_genre {
                metadata.genre.as_str()
            } else {
                ""
            },
            &options.extra.genres,
        ),
        ("tag", options.custom_tags.as_str(), &options.extra.tags),
        ("director", "", &options.extra.directors),
        ("credits", "", &options.extra.writers),
        ("studio", "", &options.extra.studios),
        ("country", "", &options.extra.countries),
    ] {
        let expanded = expand(template, value)?;
        let mut seen = std::collections::HashSet::new();
        for text in source
            .split([',', '，', '\n'])
            .chain(expanded.split([',', '，', '\n']))
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            if seen.insert(text) {
                tag(name, text);
            }
        }
    }
    let collection = expand(&options.extra.collection, value)?;
    if !collection.trim().is_empty() {
        xml.push_str(&format!(
            "  <set><name>{}</name>",
            escape_xml(collection.trim())
        ));
        let overview = expand(&options.extra.collection_overview, value)?;
        if !overview.trim().is_empty() {
            xml.push_str(&format!(
                "<overview>{}</overview>",
                escape_xml(overview.trim())
            ));
        }
        xml.push_str("</set>\n");
    }
    if options.include_source && !metadata.bvid.is_empty() {
        let id = metadata
            .cid
            .map(|cid| format!("{}-{cid}", metadata.bvid))
            .unwrap_or_else(|| metadata.bvid.clone());
        xml.push_str(&format!(
            "  <uniqueid type=\"bilibili\" default=\"true\">{}</uniqueid>\n",
            escape_xml(&id)
        ));
    }
    if options.include_cover
        && let Some(cover) = cover
    {
        xml.push_str(&format!(
            "  <thumb aspect=\"poster\">{}</thumb>\n",
            escape_xml(cover)
        ));
    }
    if options.uploader_as_actor && !metadata.uploader.is_empty() {
        xml.push_str(&format!(
            "  <actor><name>{}</name><role>UP 主</role></actor>\n",
            escape_xml(&metadata.uploader)
        ));
    }
    xml.push_str("</movie>\n");
    Ok(xml)
}

fn escape_xml(text: &str) -> String {
    text.chars()
        .filter(|c| {
            matches!(c, '\t' | '\n' | '\r')
                || (*c >= '\u{20}' && !matches!(c, '\u{fffe}' | '\u{ffff}'))
        })
        .collect::<String>()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn advanced_fields_support_grouping_sorting_lists_and_safe_formats() {
        let metadata = NfoMetadata {
            title: "课程 & 教程".into(),
            uploader: "老师".into(),
            genre: "知识".into(),
            part_index: 2,
            index: 7,
            publish_date: "2026-10-18".into(),
            quality: "80".into(),
            source_url: "https://example.test/watch?p=2&x=1".into(),
            ..Default::default()
        };
        let options = NfoOptions {
            extra: NfoExtraFields {
                sort_title: "{publish_date:%Y.%m.%d}-{part_index:03}".into(),
                original_title: "{video_title}".into(),
                tagline: "{quality_label}".into(),
                outline: "来自 {uploader}".into(),
                collection: "{video_title}".into(),
                collection_overview: "简介 & 资料".into(),
                genres: "知识,教程，知识".into(),
                tags: "{uploader}, {quality_label}".into(),
                directors: "甲,乙".into(),
                writers: "丙".into(),
                studios: "真实机构".into(),
                countries: "中国".into(),
            },
            ..Default::default()
        };
        let xml = render(&options, &metadata, "title", None).unwrap();
        assert!(xml.contains("<sorttitle>2026.10.18-002</sorttitle>"));
        assert!(xml.contains("<originaltitle>课程 &amp; 教程</originaltitle>"));
        assert!(xml.contains(
            "<set><name>课程 &amp; 教程</name><overview>简介 &amp; 资料</overview></set>"
        ));
        assert_eq!(xml.matches("<genre>知识</genre>").count(), 1);
        assert!(xml.contains("<tag>1080P</tag>"));
        for tag in [
            "tagline", "outline", "director", "credits", "studio", "country",
        ] {
            assert!(xml.contains(&format!("<{tag}>")));
        }
        assert!(xml.contains("?p=2&amp;x=1"));
        for template in [
            "{part_index:999999}",
            "{publish_date:%Q}",
            "{missing}",
            "{title}}",
        ] {
            let mut invalid = options.clone();
            invalid.extra.sort_title = template.into();
            assert!(invalid.validate().is_err());
        }
        let old: NfoOptions =
            serde_json::from_str(r#"{"extra":{"sort_title":"{title}"}}"#).unwrap();
        assert!(old.extra.directors.is_empty());
        assert!(old.validate().is_ok());
    }

    #[test]
    fn portable_fields_escape_values_and_preserve_machine_dates() {
        let m = NfoMetadata {
            description: "<test>&\u{1}".into(),
            publish_date: "2026-10-18".into(),
            duration_seconds: Some(61),
            bvid: "BV123".into(),
            cid: Some(42),
            ..Default::default()
        };
        let o = NfoOptions {
            custom_tags: "教程,教程，收藏".into(),
            ..Default::default()
        };
        let xml = render(&o, &m, "A&B", Some("A&B.cover.jpg")).unwrap();
        assert!(xml.contains("<title>A&amp;B</title>"));
        assert!(xml.contains("&lt;test&gt;&amp;"));
        assert!(!xml.contains('\u{1}'));
        assert!(xml.contains("<premiered>2026-10-18</premiered>"));
        assert!(xml.contains("<runtime>2</runtime>"));
        assert!(xml.contains(">BV123-42</uniqueid>"));
        assert_eq!(xml.matches("<tag>教程</tag>").count(), 1);
        assert!(!xml.contains("<actor>"));
        #[derive(Deserialize)]
        struct Movie {
            title: String,
            plot: String,
            premiered: String,
            runtime: u64,
        }
        let parsed: Movie = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(parsed.title, "A&B");
        assert!(parsed.plot.starts_with("<test>&"));
        assert_eq!(parsed.premiered, "2026-10-18");
        assert_eq!(parsed.runtime, 2);

        let disabled = NfoOptions {
            include_plot: false,
            include_date: false,
            include_runtime: false,
            include_genre: false,
            include_source: false,
            include_cover: false,
            ..Default::default()
        };
        let xml = render(&disabled, &m, "title", Some("cover.jpg")).unwrap();
        for tag in [
            "plot",
            "premiered",
            "year",
            "runtime",
            "genre",
            "uniqueid",
            "thumb",
        ] {
            assert!(!xml.contains(&format!("<{tag}")));
        }
    }
    #[test]
    fn old_defaults_and_disabled_fields() {
        let o: NfoOptions = serde_json::from_str("{}").unwrap();
        assert_eq!(o, NfoOptions::default());
        let xml = render(&o, &NfoMetadata::default(), "old task", None).unwrap();
        assert!(!xml.contains("<premiered>"));
        assert!(!xml.contains("<runtime>"));
        assert!(!xml.contains("<uniqueid"));
        assert!(
            render(
                &NfoOptions {
                    title_template: "{unknown}".into(),
                    ..o
                },
                &NfoMetadata::default(),
                "x",
                None
            )
            .is_err()
        );
    }
}
