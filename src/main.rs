use ruststartup::comment;
use ruststartup::enumeration;
use ruststartup::formatter;
use ruststartup::generics;
use ruststartup::hashmap_usage;
use ruststartup::life_cycle;
use ruststartup::match_keyword;
use ruststartup::trait_debug_display;
use ruststartup::trait_x;
use ruststartup::trait_object;
use ruststartup::vector_usage;

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

    life_cycle::trigger_dangling_pointer();
    let [xs1, xs2] = [vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let xs = life_cycle::test_which_bigger_vector(&xs1, &xs2);
    println!("{:?}", xs);

    generics::test_add();
    generics::test_generic_struct();
    generics::test_const_generic();

    trait_x::test_auto_derive_trait();
    trait_x::test_custom_fly_trait();
    trait_x::test_trait_as_parameter();
    trait_x::test_multiple_trait_bounds();
    trait_x::test_multiple_trait_bounds_where();

    trait_object::test_zoo();
    trait_object::test_dog_swin_and_run();

    vector_usage::create_vec();
    vector_usage::add_and_remove_elements();
    vector_usage::iter_vec();

    hashmap_usage::create_hashmap_example();
    hashmap_usage::query_hashmap_example();
}
