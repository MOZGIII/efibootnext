# efibootnext

A library and a cli for manipulating the BootNext UEFI variable and listing
possible boot options.

The aim of this project is to provide heuristics required to support operational
implementation of abstractions around the UEFI boot process management.
Normally systems expose all the required functionality, but some (Windows 10 in
particular) are lacking the kernel API that's needed for the correct
implementation. Thankfully, we can implement workarounds, like the heuristics
this crate provides.
