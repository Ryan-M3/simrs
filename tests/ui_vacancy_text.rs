#![cfg(feature = "graphics")]

use bevy::prelude::*;
use bevy::text::{Font, FontLoader};
use simrs::{Gregslist, Advert, VacancyText, VacancyTextPlugin};

fn dummy_ad() -> Advert {
    Advert { job: Entity::from_raw(0), role_index: 0, date_posted: 0.0 }
}

fn app_with_vacancy_ui(board: Gregslist) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::asset::AssetPlugin::default());
    app.init_asset::<Font>();
    app.init_asset_loader::<FontLoader>();
    app.world_mut().insert_resource(board);
    app.add_plugins(VacancyTextPlugin);
    app
}

fn read_vacancy_text(app: &mut App) -> String {
    let world = app.world_mut();
    let mut q = world.query_filtered::<&Text, With<VacancyText>>();
    let text = q.single(world).unwrap();
    text.0.clone()
}

#[test]
fn spawns_one_vacancy_text_on_startup() {
    let board = Gregslist::default();
    let mut app = app_with_vacancy_ui(board);
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<Entity, With<VacancyText>>();
    let count = q.iter(world).count();
    assert_eq!(count, 1);
}

#[test]
fn vacancy_text_reflects_gregslist_len() {
    let board = Gregslist::default();
    let mut app = app_with_vacancy_ui(board);
    app.update();
    let s0 = read_vacancy_text(&mut app);
    assert_eq!(s0, "Open roles: 0");
    {
        let mut g = app.world_mut().resource_mut::<Gregslist>();
        g.ads.push(dummy_ad());
        g.ads.push(dummy_ad());
        g.ads.push(dummy_ad());
    }
    app.update();
    let s1 = read_vacancy_text(&mut app);
    assert_eq!(s1, "Open roles: 3");
}

#[test]
fn vacancy_text_updates_after_hiring_drains_board() {
    let mut board = Gregslist::default();
    board.ads.push(dummy_ad());
    board.ads.push(dummy_ad());
    let mut app = app_with_vacancy_ui(board);
    app.update();
    {
        let mut g = app.world_mut().resource_mut::<Gregslist>();
        g.ads.clear();
    }
    app.update();
    let s1 = read_vacancy_text(&mut app);
    assert_eq!(s1, "Open roles: 0");
}
