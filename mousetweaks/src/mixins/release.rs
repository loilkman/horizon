// A button let go over a container screen.
fn mousetweaks_release(right: bool) {
    crate::game::native_hooks::base::mousetweaks_glue::release(right);
}
