use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn tauri_command_invoke<I, O>(
    cmd: &str,
    args: I
)
    -> std::result::Result<O, serde_wasm_bindgen::Error>
    where I: serde::Serialize, O: serde::de::DeserializeOwned
{
    let args = to_value(&args)?;
    let output = invoke(cmd, args).await;
    serde_wasm_bindgen::from_value(output)
}

#[macro_export]
macro_rules! tauri_command {
    (
        $vis:vis fn $ident:ident < $($lifetime:lifetime),*
        > ($($arg_ident:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)?
    ) => {
        $vis async fn $ident <$($lifetime)+>($($arg_ident: $arg_ty), *) $(-> std::result::Result<$ret_ty, serde_wasm_bindgen::Error>)? {
            #[derive(serde::Serialize)]
            struct TauriCommandArgs<$($lifetime)+> { $($arg_ident: $arg_ty),* }
            $crate::helper::tauri_command_invoke::<&TauriCommandArgs, $($ret_ty)+>(stringify!($ident), &TauriCommandArgs { $($arg_ident),* }).await
        }
    };
    ($vis:vis fn $ident:ident($($arg_ident:ident: $arg_ty:ty),*) $(-> $ret_ty:ty)?) => {
        $vis async fn $ident ($($arg_ident: $arg_ty), *) $(-> std::result::Result<$ret_ty, serde_wasm_bindgen::Error>)? {
            #[derive(serde::Serialize)]
            struct TauriCommandArgs { $($arg_ident: $arg_ty),* }
            $crate::helper::tauri_command_invoke::<&TauriCommandArgs, $($ret_ty)+>(stringify!($ident), &TauriCommandArgs { $($arg_ident),* }).await
        }
    };
}
