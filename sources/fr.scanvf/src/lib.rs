#![no_std]
//! Scan VF (https://www.scan-vf.net), a French site running My Manga Reader CMS.
//!
//! Manga pages live at `{BASE_URL}/{slug}` (no `/manga/` prefix), so `item_path`
//! is empty. Selectors are the MMRCMS defaults and still need checking against
//! the live HTML of the site.
use aidoku::{Source, prelude::*};
use mmrcms::{Impl, MMRCMS, Params};

const BASE_URL: &str = "https://www.scan-vf.net";

struct ScanVf;

impl Impl for ScanVf {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			item_path: "".into(),
			chapter_string: "Chapitre".into(),
			date_format: "d MMM. yyyy".into(),
			..Default::default()
		}
	}
}

register_source!(
	MMRCMS<ScanVf>,
	ListingProvider,
	DynamicFilters,
	ImageRequestProvider,
	DeepLinkHandler
);
