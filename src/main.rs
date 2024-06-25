use ruststartup::async_await;
use ruststartup::atomic_usage;
use ruststartup::cir_ref;
use ruststartup::closure;
use ruststartup::closure2;
use ruststartup::comment;
use ruststartup::enumeration;
use ruststartup::error_handle;
use ruststartup::formatter;
use ruststartup::generics;
use ruststartup::global_variables;
use ruststartup::hashmap_usage;
use ruststartup::life_cycle;
use ruststartup::macros;
use ruststartup::match_keyword;
use ruststartup::smart_pointer;
use ruststartup::trait_debug_display;
use ruststartup::trait_object;
use ruststartup::trait_x;
use ruststartup::types;
use ruststartup::unsafe_usage;
use ruststartup::variables;
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
    life_cycle::test_longest();
    life_cycle::test_longest2();
    life_cycle::use_life_circle_in_struct();

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

    closure::simple_closure();
    closure::fn_as_parameter();
    closure::fn_as_return();
    closure::test_owner();
    closure::fn_as_parameter2();
    closure::fn_variable_declare();
    closure::capture_outer_variable();
    closure::rust_by_example_capturing();

    closure2::test_fn();
    closure2::test_fn_mut();
    closure2::test_fn_once();

    error_handle::panic_active_trigger();
    error_handle::panic_passive_triger();
    error_handle::ip_parse();
    error_handle::test_result_enum();
    error_handle::error_propagation();

    types::test_as();
    types::as_examples();
    types::mem_addr_to_pointer();
    types::passing_as();
    types::try_into();

    macros::test_do_something_macro();

    async_await::run_future();
    async_await::run_future2();
    // async_await::run_future3();

    unsafe_usage::raw_pointer();
    unsafe_usage::vec_pointer();
    unsafe_usage::create_raw_pointer();
    unsafe_usage::create_raw_pointer_from_address();
    unsafe_usage::use_raw_pointer_from_address();
    unsafe_usage::create_raw_pointer_from_box();
    unsafe_usage::test_split_at_mut();
    unsafe_usage::unsafe_trait_use_raw_pointer();

    variables::use_static_mut_variable();

    atomic_usage::test_start_atomicu8();

    cir_ref::use_list();

    println!("输出定义的常量 {}", global_variables::MAX_ID);
    global_variables::add_counter();

    smart_pointer::use_box_integer();
    smart_pointer::large_scale_datastructure();
    smart_pointer::normal_deref();
    smart_pointer::use_deref();
    smart_pointer::test_drops();
}
