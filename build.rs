fn main() {
    let doomgeneric_dir = "third_party/doomgeneric/doomgeneric";

    // Probe for SDL2 + SDL2_mixer to enable in-engine sound and music.
    // If the libraries are not installed, the game builds and runs silently.
    let sdl2 = pkg_config::Config::new()
        .atleast_version("2.0.0")
        .probe("sdl2");
    let sdl2_mixer = pkg_config::Config::new().probe("SDL2_mixer");
    let has_sound = sdl2.is_ok() && sdl2_mixer.is_ok();
    if !has_sound {
        eprintln!(
            "cargo:warning=SDL2/SDL2_mixer not found — building without audio (install libsdl2-dev libsdl2-mixer-dev to enable)"
        );
    }

    let doom_sources = [
        "dummy.c",
        "am_map.c",
        "doomdef.c",
        "doomstat.c",
        "dstrings.c",
        "d_event.c",
        "d_items.c",
        "d_iwad.c",
        "d_loop.c",
        "d_main.c",
        "d_mode.c",
        "d_net.c",
        "f_finale.c",
        "f_wipe.c",
        "g_game.c",
        "hu_lib.c",
        "hu_stuff.c",
        "info.c",
        "i_cdmus.c",
        "i_endoom.c",
        "i_joystick.c",
        "i_scale.c",
        "i_sound.c",
        "i_system.c",
        "i_timer.c",
        "memio.c",
        "m_argv.c",
        "m_bbox.c",
        "m_cheat.c",
        "m_config.c",
        "m_controls.c",
        "m_fixed.c",
        "m_menu.c",
        "m_misc.c",
        "m_random.c",
        "p_ceilng.c",
        "p_doors.c",
        "p_enemy.c",
        "p_floor.c",
        "p_inter.c",
        "p_lights.c",
        "p_map.c",
        "p_maputl.c",
        "p_mobj.c",
        "p_plats.c",
        "p_pspr.c",
        "p_saveg.c",
        "p_setup.c",
        "p_sight.c",
        "p_spec.c",
        "p_switch.c",
        "p_telept.c",
        "p_tick.c",
        "p_user.c",
        "r_bsp.c",
        "r_data.c",
        "r_draw.c",
        "r_main.c",
        "r_plane.c",
        "r_segs.c",
        "r_sky.c",
        "r_things.c",
        "sha1.c",
        "sounds.c",
        "statdump.c",
        "st_lib.c",
        "st_stuff.c",
        "s_sound.c",
        "tables.c",
        "v_video.c",
        "wi_stuff.c",
        "w_checksum.c",
        "w_file.c",
        "w_main.c",
        "w_wad.c",
        "z_zone.c",
        "w_file_stdc.c",
        "i_input.c",
        "i_video.c",
        "doomgeneric.c",
    ];

    let mut doom_build = cc::Build::new();
    doom_build
        .include(doomgeneric_dir)
        .flag_if_supported("-std=c99")
        .define("NORMALUNIX", None)
        .define("LINUX", None)
        .define("SNDSERV", None)
        .define("_DEFAULT_SOURCE", None)
        .define("DOOMGENERIC_RESX", "320")
        .define("DOOMGENERIC_RESY", "200");

    if has_sound {
        let sdl2_lib = sdl2.as_ref().unwrap();
        let sdl2_mixer_lib = sdl2_mixer.as_ref().unwrap();

        doom_build.define("FEATURE_SOUND", None);

        for path in &sdl2_lib.include_paths {
            doom_build.include(path);
        }
        for path in &sdl2_mixer_lib.include_paths {
            doom_build.include(path);
        }

        // SDL audio modules: SFX (PCM from WAD) + music (MUS→MIDI→SDL_mixer)
        for source in &["i_sdlsound.c", "i_sdlmusic.c", "mus2mid.c", "gusconf.c"] {
            doom_build.file(format!("{doomgeneric_dir}/{source}"));
        }
    }

    for source in &doom_sources {
        doom_build.file(format!("{doomgeneric_dir}/{source}"));
    }
    doom_build.compile("doomgeneric_core");

    #[cfg(target_family = "unix")]
    println!("cargo:rustc-link-lib=m");

    println!("cargo:rerun-if-changed=third_party/doomgeneric/doomgeneric");
}
