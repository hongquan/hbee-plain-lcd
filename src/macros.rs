/// Macro to quickly create EspError from an ESP_ERR_ constant.
#[macro_export]
macro_rules! esp_err {
    ($x:ident) => {
        EspError::from_infallible::<{ $x }>()
    };
}
