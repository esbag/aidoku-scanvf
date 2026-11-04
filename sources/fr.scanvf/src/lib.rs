#![no_std]
//! Scan VF (https://www.scan-vf.net), a French site running My Manga Reader CMS.
//!
//! Differences from the stock MMRCMS template, checked against the live HTML
//! (November 2026):
//! - manga pages live at `{BASE_URL}/{slug}` (no `/manga/` prefix);
//! - chapter rows read "One Piece 1197 : <em>Title</em>", so the number is taken
//!   from the URL (`chapitre-1197`, `Chapitre-17`, `episode-247`) and the title
//!   from the `<em>`;
//! - page images are lazy-loaded with a `data-src` padded with spaces;
//! - `/latest-release` lists `.manga-item` blocks, one per update, so the same
//!   manga appears several times.
use aidoku::{
	Chapter, Manga, MangaPageResult, Page, PageContent, Result, Source,
	alloc::{String, Vec, format},
	imports::{
		html::{Document, Element},
		net::Request,
		std::parse_date,
	},
	prelude::*,
};
use mmrcms::{Impl, MMRCMS, Params, helpers};

const BASE_URL: &str = "https://www.scan-vf.net";

struct ScanVf;

/// Chapter number from a chapter key such as `chapitre-1197`, `Chapitre-17`,
/// `episode-247` or `chapitre-139.5`.
fn number_from_key(key: &str) -> Option<f32> {
	key.rsplit('-').next()?.trim().parse().ok()
}

impl Impl for ScanVf {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			item_path: "".into(),
			details_title_selector: "h2.widget-title".into(),
			manga_list_selector: "div.media".into(),
			manga_list_next_page_selector: ".pagination a[rel=next]".into(),
			chapter_list_selector: "ul.chapters > li".into(),
			chapter_string: "Chapitre".into(),
			date_format: "d MMM. yyyy".into(),
			..Default::default()
		}
	}

	fn parse_chapter_element(
		&self,
		params: &Params,
		element: &Element,
		manga: &Manga,
	) -> Option<Chapter> {
		let title_wrapper = element.select_first(".chapter-title-rtl")?;
		let anchor = title_wrapper.select_first("a")?;
		let url = anchor.attr("abs:href")?;
		let key: String = url
			.strip_prefix(&format!("{}/{}/", params.base_url, manga.key))?
			.trim_end_matches('/')
			.into();

		let chapter_number = number_from_key(&key).or_else(|| {
			anchor
				.text()
				.and_then(|text| helpers::chapter_number(text.trim_start_matches(&*manga.title)))
		});
		let title = title_wrapper
			.select_first("em")
			.and_then(|em| em.text())
			.filter(|text| !text.is_empty());

		Some(Chapter {
			key,
			chapter_number,
			title,
			date_uploaded: element
				.select_first(".date-chapter-title-rtl")
				.and_then(|el| el.text())
				.and_then(|date| parse_date(date.trim(), &params.date_format)),
			url: Some(url),
			language: Some("fr".into()),
			..Default::default()
		})
	}

	fn parse_page_list(&self, _params: &Params, html: &Document) -> Vec<Page> {
		html.select("#all img")
			.map(|imgs| {
				imgs.filter_map(|img| {
					let url = img
						.attr("data-src")
						.map(|src| String::from(src.trim()))
						.filter(|src| !src.is_empty())
						.or_else(|| img.attr("abs:src"))
						.filter(|src| !src.starts_with("data:"))?;
					Some(Page {
						content: PageContent::url(url),
						..Default::default()
					})
				})
				.collect()
			})
			.unwrap_or_default()
	}

	fn get_manga_list(
		&self,
		params: &Params,
		listing: aidoku::Listing,
		page: i32,
	) -> Result<MangaPageResult> {
		if listing.id == "popular" {
			let html = Request::get(format!(
				"{}/filterList?page={page}&sortBy=views&asc=false",
				params.base_url
			))?
			.header("Referer", &format!("{}/", params.base_url))
			.html()?;
			return Ok(self.parse_manga_page(params, &html));
		}

		let html = Request::get(format!("{}/latest-release?page={page}", params.base_url))?
			.header("Referer", &format!("{}/", params.base_url))
			.html()?;
		let mut entries: Vec<Manga> = Vec::new();
		if let Some(items) = html.select(".mangalist .manga-item") {
			for item in items {
				let Some(manga) =
					helpers::manga_from_element(&item, &params.base_url, &params.item_path)
				else {
					continue;
				};
				if !entries.iter().any(|existing| existing.key == manga.key) {
					entries.push(manga);
				}
			}
		}
		Ok(MangaPageResult {
			entries,
			has_next_page: html
				.select(&params.manga_list_next_page_selector)
				.is_some(),
		})
	}
}

register_source!(
	MMRCMS<ScanVf>,
	ListingProvider,
	DynamicFilters,
	ImageRequestProvider,
	DeepLinkHandler
);
