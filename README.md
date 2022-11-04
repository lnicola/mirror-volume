# Introduction

This is a small utility to mirror the muted state (playback switch) and volume from one ALSA mixer control to another.
Developed to work around a Linux driver issue (speaker volume being controlled by the headphone one) on the Huawei Matebook 14s.

It will not do anything on other Matebook models.

# Building

Follow the instructions from https://rustup.rs/, clone the repository, run `cargo build --release`, you'll find the binary under `target/release`.

You'll need to install the ALSA development package (`libasound2-dev`, `alsa-lib-devel`, `alsa-lib` etc.) and a C linker (`binutils`).

# Configuration

The `Speaker` and `Headphone` controls of the `hw:0` card are used by default, but you can change them using the `MV_CARD_NAME`, `MV_SOURCE_SELEM` and `MV_TARGET_SELEM` environment variables.

# Installation

Copy the binary somewhere, run on login.
Also works as a `systemd` user service.

# Caveats

Not exactly production-ready.
Tested on Fedora 37, under PipeWire, on a single Matebook 14s.
