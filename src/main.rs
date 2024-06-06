use ruststartup::comment;
use ruststartup::trait_debug_display;
use ruststartup::formatter;

fn main() {
    comment::line_comment();
    comment::block_comment();
    comment::doc_comment();

    trait_debug_display::derive_debug_trait();
    trait_debug_display::custom_debug_trait();
    trait_debug_display::custom_display_trait();

    formatter::formatter_examples();
}
