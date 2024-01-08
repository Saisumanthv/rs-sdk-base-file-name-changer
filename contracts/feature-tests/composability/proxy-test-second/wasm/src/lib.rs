// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   6

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!(static64k);
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    proxy_test_second
    (
        init => init
        upgrade => upgrade
        payMe => pay_me
        payMeWithResult => pay_me_with_result_endpoint
        messageMe => message_me
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}