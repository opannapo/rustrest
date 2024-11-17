mod package;
use package::log as custom_log;

fn main() {
    custom_log::init();

    log::info!("test");

    let mut v_origin: i32 = 10;
    log::info!("{}", v_origin);
    update_val(&mut v_origin);
    log::info!("{}", v_origin);
    update_val(&mut v_origin);
    log::info!("{}", v_origin);
}
fn update_val(v: &mut i32) {
    *v += 10;
    log::info!("{}", v);
}
