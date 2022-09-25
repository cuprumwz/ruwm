#![cfg_attr(not(feature = "std"), no_std)]
#![feature(cfg_version)]
#![cfg_attr(not(version("1.65")), feature(generic_associated_types))]
#![cfg_attr(not(version("1.64")), feature(future_poll_fn))]
#![feature(type_alias_impl_trait)]

pub mod battery;
pub mod button;
pub mod channel;
pub mod emergency;
pub mod error;
pub mod keepalive;
pub mod mqtt;
pub mod notification;
pub mod pulse_counter;
pub mod screen;
pub mod state;
pub mod system;
pub mod valve;
pub mod water_meter;
pub mod water_meter_stats;
pub mod web;
pub mod web_dto;
pub mod wifi;
