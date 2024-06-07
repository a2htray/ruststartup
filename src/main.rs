use ruststartup::comment;
use ruststartup::enumeration;
use ruststartup::formatter;
use ruststartup::trait_debug_display;
use ruststartup::match_keyword;

fn main() {
    comment::line_comment();
    comment::block_comment();
    comment::doc_comment();

    trait_debug_display::derive_debug_trait();
    trait_debug_display::custom_debug_trait();
    trait_debug_display::custom_display_trait();

    formatter::formatter_examples();

    enumeration::print_car_level();
    enumeration::print_car_level_with_brand();
    enumeration::print_car_level_with_brand_or_price();

    match_keyword::simple_match();
    match_keyword::use_or_operation_in_match();
    match_keyword::binding_in_match();
    match_keyword::matches_macro();
}
