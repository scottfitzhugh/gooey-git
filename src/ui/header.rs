use libadwaita as adw;
use adw::HeaderBar;
use gtk4::Button;

pub struct Header {
	header_bar: HeaderBar,
}

impl Header {
	pub fn new() -> Self {
		// Create a header bar
		let header_bar = HeaderBar::new();
		
		// Create open repository button
		let open_btn = Button::builder()
			.icon_name("folder-open-symbolic")
			.tooltip_text("Open Repository")
			.build();
		
		// Add open button to the start of the header
		header_bar.pack_start(&open_btn);
		
		// Create repository menu button
		let menu_btn = Button::builder()
			.icon_name("view-more-symbolic")
			.tooltip_text("Repository Actions")
			.build();
		
		// Add menu button to the end of the header
		header_bar.pack_end(&menu_btn);
		
		Self { header_bar }
	}
	
	pub fn widget(&self) -> HeaderBar {
		self.header_bar.clone()
	}
} 