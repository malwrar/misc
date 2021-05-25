///

use std::error::Error;

use x11rb;
use x11rb::connection::{Connection, SequenceNumber};
use x11rb::cursor::Handle as CursorHandle;
use x11rb::errors::{ConnectionError, ReplyError, ReplyOrIdError};
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::resource_manager::Database;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    // Core x11 stuff
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id().unwrap();
    let gc_id = conn.generate_id().unwrap();
    let resource_db = Database::new_from_default(&conn).unwrap();
    let cursor_handle = CursorHandle::new(&conn, screen_num, &resource_db).unwrap();

    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY | EventMask::NO_EVENT)
        .background_pixel(screen.white_pixel)
        .win_gravity(Gravity::NORTH_WEST);
        // Just because, we set the cursor to "wait"
        //.cursor(cursor_handle.load_cursor(&conn, "wait").unwrap());

    let gc_aux = CreateGCAux::new().foreground(screen.black_pixel);

    let (mut width, mut height) = (100, 100);

    conn.create_window(
        screen.root_depth,
        win_id,
        screen.root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &win_aux,
    )
    .unwrap();

    conn.create_gc(gc_id, win_id, &gc_aux).unwrap();

    conn.map_window(win_id).unwrap();

    conn.flush().unwrap();





    TODO: figure out how to hook up to glx, draw window
    TODO: setup legion ECS thingy, ensure threading works





    loop {
        let event = conn.wait_for_event().unwrap();

        println!("{:?})", event);
    }
}

