# Keycursieve

_Keycursieve_ is an experiment in building a flexible keystroke library for keyboard firmware and other usages. The goal is to create a powerful, compositional framework to allow complicated use-cases to be created quickly.

As a contributor to QMK, one of the difficulties I found was replicating a lot of the same functionality for every feature I created such as one-shot and mod-tap. In addition, custom functions were incompatible with sophisticated features like chording and leaders.

_Keycursieve_ aims to make creating custom functionality easier by composing complicated functionality and allowing for reprocessing of modified keystrokes in a new context. It's name is a portmanteau of _key_, _recursive_, and _sieve_.

## Theory

To see how _Keycursieve_ works it helps to look at an example, _mod-tap_. The concept behind _mod-tap_ is that if you hold down a key it performs one action such as `<Ctrl>` and a different action if you tap it, such as `<Esc>`.

In QMK _mod-tap_ is limited to modifiers and standard key codes. If you want to add _mod-tap_ functionality to your new action you have to implement it yourself.

How this might be implemented in _Keycursieve_ is to have a handler for the key press. When pressed down, the handler sets a timeout--the delay before the mod goes active--and `yields` back to the main loop. If the timeout occurs, the handler will be invoked and then returns a data structure to set the modifier and `resume` processing of the pipeline. On key-up the handler will check to see if the timeout occurred, and if so unset the modifier. If not, then the handler will ask the system to `restart` the pipeline with a new key code.

The `yield`, `resume`, and `restart` semantics are the core concept here and represent the bulk of the experiment.