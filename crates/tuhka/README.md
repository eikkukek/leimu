# tuhka
New up-to-date Vulkan bindings used by [leimu].

Follows mostly the same structure as [ash], but differs in some areas.

## Usage

``` rust
use tuhka::{vk, Library, Instance};
let library = unsafe {
      Library::load()
}.unwrap();
let app_name = c"Test";
let engine_name = c"Engine";
let application_info = vk::ApplicationInfo {
    p_application_name: app_name.as_ptr(),
    application_version: vk::make_api_version(0, 1, 0, 0),
    p_engine_name: engine_name.as_ptr(),
    engine_version: vk::make_api_version(0, 1, 0, 0),
    api_version: vk::API_VERSION_1_4,
    ..Default::default()
};
let create_info = vk::InstanceCreateInfo {
    p_application_info: &application_info,
    ..Default::default()
};
let instance: Instance = unsafe {
    library.create_instance(&create_info, None)
}.unwrap().value;
unsafe {
    library.destroy_instance(&instance, None);
}
```
[leimu]: https://crates.io/crates/leimu
[ash]: https://crates.io/crates/ash
