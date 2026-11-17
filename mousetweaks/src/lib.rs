//! Mouse Tweaks for Horizon: the mod's settings, on its page in Options > Mods,
//! and the code the game runs at the press, drag, release and wheel of a
//! container screen (`src/glue.rs`, `src/runtime.rs`).
//!
//! A port of Mouse Tweaks by Ivan Molodetskikh (YaLTeR), BSD-3-Clause,
//! <https://github.com/YaLTeR/MouseTweaks>: `Main.java`'s RMB, LMB and wheel
//! tweaks and `Config.java`'s options.
use mod_api::{Hooks, Registrar, RegistrationContext, SettingSpec};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The seven options `Config.java` keeps, on the mod's page in
    /// Options > Mods. The game draws them with its own widgets and keeps the
    /// player's values; `src/glue.rs` reads them back once a press.
    fn register(&self, r: &mut dyn Registrar, _cx: &RegistrationContext) {
        for spec in settings() {
            // A spec the game refuses (a bad key, an empty label) is its to
            // report; the others still register.
            let _ = r.register_setting(spec);
        }
    }
}

/// The settings, in the order the page shows them, with `Config`'s defaults.
/// The keys are the ones `Settings::read` in `src/runtime.rs` reads, and a
/// choice's index is the one it maps back to its option.
fn settings() -> [SettingSpec; 7] {
    [
        SettingSpec::toggle("rmb_tweak", "mousetweaks.setting.rmb_tweak", true),
        SettingSpec::toggle(
            "lmb_tweak_with_item",
            "mousetweaks.setting.lmb_tweak_with_item",
            true,
        ),
        SettingSpec::toggle(
            "lmb_tweak_without_item",
            "mousetweaks.setting.lmb_tweak_without_item",
            true,
        ),
        SettingSpec::toggle("wheel_tweak", "mousetweaks.setting.wheel_tweak", true),
        SettingSpec::choice(
            "wheel_search_order",
            "mousetweaks.setting.wheel_search_order",
            [
                "mousetweaks.value.first_to_last",
                "mousetweaks.value.last_to_first",
            ],
            1,
        ),
        SettingSpec::choice(
            "wheel_scroll_direction",
            "mousetweaks.setting.wheel_scroll_direction",
            [
                "mousetweaks.value.down_to_push",
                "mousetweaks.value.up_to_push",
                "mousetweaks.value.position_aware",
                "mousetweaks.value.position_aware_inverted",
            ],
            0,
        ),
        SettingSpec::choice(
            "scroll_item_scaling",
            "mousetweaks.setting.scroll_item_scaling",
            [
                "mousetweaks.value.proportional",
                "mousetweaks.value.always_one",
            ],
            0,
        ),
    ]
}
