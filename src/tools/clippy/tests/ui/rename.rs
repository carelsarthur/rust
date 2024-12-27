// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

#![allow(clippy::duplicated_attributes)]
#![allow(clippy::almost_complete_range)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::blocks_in_conditions)]
#![allow(clippy::box_collection)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::disallowed_types)]
#![allow(clippy::mixed_read_write_in_expression)]
#![allow(clippy::manual_find_map)]
#![allow(clippy::manual_filter_map)]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::match_result_ok)]
#![allow(clippy::non_canonical_clone_impl)]
#![allow(clippy::non_canonical_partial_ord_impl)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::overly_complex_bool_expr)]
#![allow(clippy::new_without_default)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::expect_used)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::panicking_overflow_checks)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::single_char_add_str)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_const_for_thread_local)]
#![allow(clippy::recursive_format_impl)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::invisible_characters)]
#![allow(invalid_reference_casting)]
#![allow(suspicious_double_ref_op)]
#![allow(invalid_nan_comparisons)]
#![allow(drop_bounds)]
#![allow(dropping_copy_types)]
#![allow(dropping_references)]
#![allow(useless_ptr_null_checks)]
#![allow(for_loops_over_fallibles)]
#![allow(forgetting_copy_types)]
#![allow(forgetting_references)]
#![allow(array_into_iter)]
#![allow(invalid_atomic_ordering)]
#![allow(invalid_value)]
#![allow(invalid_from_utf8_unchecked)]
#![allow(let_underscore_drop)]
#![allow(unexpected_cfgs)]
#![allow(enum_intrinsics_non_enums)]
#![allow(non_fmt_panics)]
#![allow(named_arguments_used_positionally)]
#![allow(dangling_pointers_from_temporaries)]
#![allow(undropped_manually_drops)]
#![allow(unknown_lints)]
#![allow(unused_labels)]
#![allow(ambiguous_wide_pointer_comparisons)]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(clippy::reversed_empty_ranges)]
#![warn(clippy::almost_complete_letter_range)] //~ ERROR: lint `clippy::almost_complete_letter_range`
#![warn(clippy::blacklisted_name)] //~ ERROR: lint `clippy::blacklisted_name`
#![warn(clippy::block_in_if_condition_expr)] //~ ERROR: lint `clippy::block_in_if_condition_expr`
#![warn(clippy::block_in_if_condition_stmt)] //~ ERROR: lint `clippy::block_in_if_condition_stmt`
#![warn(clippy::blocks_in_if_conditions)] //~ ERROR: lint `clippy::blocks_in_if_conditions`
#![warn(clippy::box_vec)] //~ ERROR: lint `clippy::box_vec`
#![warn(clippy::const_static_lifetime)] //~ ERROR: lint `clippy::const_static_lifetime`
#![warn(clippy::cyclomatic_complexity)] //~ ERROR: lint `clippy::cyclomatic_complexity`
#![warn(clippy::derive_hash_xor_eq)] //~ ERROR: lint `clippy::derive_hash_xor_eq`
#![warn(clippy::disallowed_method)] //~ ERROR: lint `clippy::disallowed_method`
#![warn(clippy::disallowed_type)] //~ ERROR: lint `clippy::disallowed_type`
#![warn(clippy::eval_order_dependence)] //~ ERROR: lint `clippy::eval_order_dependence`
#![warn(clippy::find_map)] //~ ERROR: lint `clippy::find_map`
#![warn(clippy::filter_map)] //~ ERROR: lint `clippy::filter_map`
#![warn(clippy::fn_address_comparisons)] //~ ERROR: lint `clippy::fn_address_comparisons`
#![warn(clippy::identity_conversion)] //~ ERROR: lint `clippy::identity_conversion`
#![warn(clippy::if_let_redundant_pattern_matching)] //~ ERROR: lint `clippy::if_let_redundant_pattern_matching`
#![warn(clippy::if_let_some_result)] //~ ERROR: lint `clippy::if_let_some_result`
#![warn(clippy::incorrect_clone_impl_on_copy_type)] //~ ERROR: lint `clippy::incorrect_clone_impl_on_copy_type`
#![warn(clippy::incorrect_partial_ord_impl_on_ord_type)] //~ ERROR: lint `clippy::incorrect_partial_ord_impl_on_ord_type`
#![warn(clippy::integer_arithmetic)] //~ ERROR: lint `clippy::integer_arithmetic`
#![warn(clippy::logic_bug)] //~ ERROR: lint `clippy::logic_bug`
#![warn(clippy::new_without_default_derive)] //~ ERROR: lint `clippy::new_without_default_derive`
#![warn(clippy::option_and_then_some)] //~ ERROR: lint `clippy::option_and_then_some`
#![warn(clippy::option_expect_used)] //~ ERROR: lint `clippy::option_expect_used`
#![warn(clippy::option_map_unwrap_or)] //~ ERROR: lint `clippy::option_map_unwrap_or`
#![warn(clippy::option_map_unwrap_or_else)] //~ ERROR: lint `clippy::option_map_unwrap_or_else`
#![warn(clippy::option_unwrap_used)] //~ ERROR: lint `clippy::option_unwrap_used`
#![warn(clippy::overflow_check_conditional)] //~ ERROR: lint `clippy::overflow_check_conditional`
#![warn(clippy::ref_in_deref)] //~ ERROR: lint `clippy::ref_in_deref`
#![warn(clippy::result_expect_used)] //~ ERROR: lint `clippy::result_expect_used`
#![warn(clippy::result_map_unwrap_or_else)] //~ ERROR: lint `clippy::result_map_unwrap_or_else`
#![warn(clippy::result_unwrap_used)] //~ ERROR: lint `clippy::result_unwrap_used`
#![warn(clippy::single_char_push_str)] //~ ERROR: lint `clippy::single_char_push_str`
#![warn(clippy::stutter)] //~ ERROR: lint `clippy::stutter`
#![warn(clippy::thread_local_initializer_can_be_made_const)] //~ ERROR: lint `clippy::thread_local_initializer_can_be_made_const`
#![warn(clippy::to_string_in_display)] //~ ERROR: lint `clippy::to_string_in_display`
#![warn(clippy::unwrap_or_else_default)] //~ ERROR: lint `clippy::unwrap_or_else_default`
#![warn(clippy::zero_width_space)] //~ ERROR: lint `clippy::zero_width_space`
#![warn(clippy::cast_ref_to_mut)] //~ ERROR: lint `clippy::cast_ref_to_mut`
#![warn(clippy::clone_double_ref)] //~ ERROR: lint `clippy::clone_double_ref`
#![warn(clippy::cmp_nan)] //~ ERROR: lint `clippy::cmp_nan`
#![warn(clippy::drop_bounds)] //~ ERROR: lint `clippy::drop_bounds`
#![warn(clippy::drop_copy)] //~ ERROR: lint `clippy::drop_copy`
#![warn(clippy::drop_ref)] //~ ERROR: lint `clippy::drop_ref`
#![warn(clippy::fn_null_check)] //~ ERROR: lint `clippy::fn_null_check`
#![warn(clippy::for_loop_over_option)] //~ ERROR: lint `clippy::for_loop_over_option`
#![warn(clippy::for_loop_over_result)] //~ ERROR: lint `clippy::for_loop_over_result`
#![warn(clippy::for_loops_over_fallibles)] //~ ERROR: lint `clippy::for_loops_over_fallibles`
#![warn(clippy::forget_copy)] //~ ERROR: lint `clippy::forget_copy`
#![warn(clippy::forget_ref)] //~ ERROR: lint `clippy::forget_ref`
#![warn(clippy::into_iter_on_array)] //~ ERROR: lint `clippy::into_iter_on_array`
#![warn(clippy::invalid_atomic_ordering)] //~ ERROR: lint `clippy::invalid_atomic_ordering`
#![warn(clippy::invalid_ref)] //~ ERROR: lint `clippy::invalid_ref`
#![warn(clippy::invalid_utf8_in_unchecked)] //~ ERROR: lint `clippy::invalid_utf8_in_unchecked`
#![warn(clippy::let_underscore_drop)] //~ ERROR: lint `clippy::let_underscore_drop`
#![warn(clippy::maybe_misused_cfg)] //~ ERROR: lint `clippy::maybe_misused_cfg`
#![warn(clippy::mem_discriminant_non_enum)] //~ ERROR: lint `clippy::mem_discriminant_non_enum`
#![warn(clippy::mismatched_target_os)] //~ ERROR: lint `clippy::mismatched_target_os`
#![warn(clippy::panic_params)] //~ ERROR: lint `clippy::panic_params`
#![warn(clippy::positional_named_format_parameters)] //~ ERROR: lint `clippy::positional_named_format_parameters`
#![warn(clippy::temporary_cstring_as_ptr)] //~ ERROR: lint `clippy::temporary_cstring_as_ptr`
#![warn(clippy::undropped_manually_drops)] //~ ERROR: lint `clippy::undropped_manually_drops`
#![warn(clippy::unknown_clippy_lints)] //~ ERROR: lint `clippy::unknown_clippy_lints`
#![warn(clippy::unused_label)] //~ ERROR: lint `clippy::unused_label`
#![warn(clippy::vtable_address_comparisons)] //~ ERROR: lint `clippy::vtable_address_comparisons`
#![warn(clippy::reverse_range_loop)] //~ ERROR: lint `clippy::reverse_range_loop`

fn main() {}
