// A press on a container screen, where Horizon's own inventory tweaks look at
// it: Mouse Tweaks notes it (the press goes on to the screen), and while it is
// on Horizon's own tweaks stay out.
fn mousetweaks_press(
    game: &mut crate::game::Game<'_>,
    slot: Option<usize>,
    right: bool,
    shift: bool,
) -> ::core::ops::ControlFlow<bool> {
    if crate::game::native_hooks::base::mousetweaks_glue::press(game, slot, right, shift) {
        return ::core::ops::ControlFlow::Break(false);
    }
    ::core::ops::ControlFlow::Continue(())
}
