use std::mem::swap;
use bevy::prelude::*;

mod chip_map;

use chip_map::{ChipLib, ChipData, ChipType};
//use crate::chip_map::ChipType;

const GRID_WIDTH: i32 = 9;
const GRID_HEIGHT: i32 = 6;
const GRID_PADDING_X: i32 = -2;
const GRID_PADDING_Y: i32 = 2;
const IMAGE_SIZE: i32 = 32;
const CAM_SCALE: f32 = 0.65;
const GRID_SHIFT: i32 = 17;

const BASE_DECK: [i32;6] = [1,1,1,2,2,2];

fn main() {
    App::build()
        //Window
        .insert_resource(WindowDescriptor{
            title: "Grid Test".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.15,0.45,0.2)))

        //Prerequisites
        .add_plugins(DefaultPlugins)

        //Startup
        .add_startup_system(setup.system())
        .add_startup_system(chip_map::chip_setup.system())

        //Events
        .add_event::<ButtonEvent>()
        .add_event::<GridValueEvent>()
        .add_event::<HandEvent>()

        //Runtime
        .add_system(update_grid_image.system())
        .add_system(select_manager.system())
        .add_system(mouse_input.system())
        .add_system(update_grid_value.system())
        .add_system(deck_manager.system())

        .run();
}

struct Button {
    mode: ButtonMode,
    value: Option<i32>,
}

#[derive(Copy, Clone)]
enum ButtonMode {
    None,
    Grid,
    Inventory,
    Deck,
    Menu,
}

struct Deck{
    chips: Vec<i32>,
    library: Vec<i32>,
    discard: Vec<i32>,
}

struct ButtonEvent {
    mode: ButtonMode,
    value: Option<i32>,
}

struct GridEvent{

}

struct InventoryEvent {
    index: i32,
    from: i32,
}

struct DeckEvent{

}

struct ChipSelect{
    chip_index: i32,
}

struct GridValue{
    def: i32,
    off: i32,
    flat_grid: Vec<f32>,
    increase_grid: Vec<f32>,
    more_grid: Vec<f32>,
    effect_grid: Vec<f32>,
    type_grid: Vec<ChipType>,

}
 #[derive(Copy,Clone)]
struct GridChip {
    chip_index: i32,
    index: i32,
    row: i32,
    col: i32,
}

impl GridChip{
    fn new(index: i32, row: i32, col: i32) -> GridChip{
        GridChip{
            row,
            col,
            index,
            chip_index: 0,
        }
    }
}
struct HandEvent{
    index: i32,
    value: i32,
}
struct ValueBoard;
struct MainCamera;
struct HandInv{
    index: i32,
}
struct GridValueEvent;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let texture_handle = asset_server.load("atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0,32.0),4,1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut cam_bundle = OrthographicCameraBundle::new_2d();
    cam_bundle.orthographic_projection.scale = CAM_SCALE;
    cam_bundle.transform.translation.y = -40.0;
    commands.spawn_bundle(cam_bundle).insert(MainCamera);

    commands.spawn()
        .insert(Deck{
            chips: Vec::from(BASE_DECK),
            library: Vec::from(BASE_DECK),
            discard: Vec::new(),
        });

    for x_index in 0..GRID_WIDTH {
        let x_pos = (x_index * (IMAGE_SIZE + GRID_PADDING_X) - 120) as f32;
        for y_index in 0..GRID_HEIGHT {
            let shift = (x_index % 2) * GRID_SHIFT;
            let y_pos = (y_index * (IMAGE_SIZE + GRID_PADDING_Y) - 120 + shift) as f32;
            let index = x_index + y_index * GRID_WIDTH;
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle{
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x_pos,y_pos, 0.0)),
                    sprite: TextureAtlasSprite::new(0),
                    ..Default::default()
                })
                .insert(GridChip::new(index,y_index,x_index))
                .insert(Button{
                    mode: ButtonMode::Grid,
                    value: Some(index),
                });
        }
    }
    for x_shift in 0..5 {
        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new((-80 + x_shift* 40) as f32, -170.0, 0.0)),
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            })
            .insert(Button {
                mode: ButtonMode::Inventory,
                value: None,
            })
            .insert(HandInv{
                index: x_shift,
            });
    }

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-120.0,-170.0,0.0)),
            sprite: TextureAtlasSprite::new(3),
            ..Default::default()
        })
        .insert(Button {
            mode: ButtonMode::Deck,
            value: Some(1),
        });

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(170.0,0.0,0.0)),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(ChipSelect{
            chip_index:0,
        });

    let text_style = TextStyle{
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment{
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn()
        .insert(GridValue{
            def: 0,
            off: 0,
            flat_grid: vec![0.0; (GRID_WIDTH*GRID_HEIGHT) as usize],
            increase_grid: vec![1.0; (GRID_WIDTH*GRID_HEIGHT) as usize],
            more_grid: vec![1.0; (GRID_WIDTH*GRID_HEIGHT) as usize],
            effect_grid: vec![1.0; (GRID_WIDTH* GRID_HEIGHT) as usize],
            type_grid: vec![ChipType::Empty; (GRID_WIDTH* GRID_HEIGHT) as usize],
        });

    commands
        .spawn()
        .insert_bundle(Text2dBundle{
            text: Text{
                sections: vec![
                    TextSection{
                        value: "0".to_string(),
                        style: text_style.clone()
                    },
                    TextSection{
                        value: " : ".to_string(),
                        style: text_style.clone()
                    },
                    TextSection{
                        value: "0".to_string(),
                        style: text_style.clone()
                    }
                ],
                alignment: text_alignment.clone(),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0,100.0,0.0)),
            ..Default::default()
        })
        .insert(ValueBoard);
}

fn mouse_input(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    cam_query: Query<&Transform, With<MainCamera>>,
    mut button_query: Query<(&mut Button, &Transform)>,
    mut ev_button: EventWriter<ButtonEvent>,
){
    if !buttons.just_released(MouseButton::Left) {return}

    let window = windows.get_primary().unwrap();

    if let Some(m_pos) = window.cursor_position() {

        let cam = cam_query.single().unwrap();

        let size = Vec2::new(window.width(), window.height());
        let pos_wld = screen_to_world_pos(m_pos,size,cam.compute_matrix());

        let mut mode= ButtonMode::None;
        let mut value = None;

        button_query.iter_mut().for_each(|(mut button, trans)| {
            let b_pos = Vec2::new(trans.translation.x, trans.translation.y);
            let dist = b_pos.distance(pos_wld);
            if dist < (IMAGE_SIZE/2) as f32 {
                mode = button.mode;
                value = button.value;
                button.value = None;
            }
        });

        let button_data = ButtonEvent{
            mode,
            value,
        };

        ev_button.send(button_data);
    }
}

fn button_event_handler(
    mut ev_button: EventReader<ButtonEvent>,
    mut ev_inv: EventWriter<InventoryEvent>,
    mut ev_deck: EventWriter<DeckEvent>,
    mut ev_grid: EventWriter<GridEvent>,
){
    for ev in ev_button.iter() {
        let val = ev.value;
        match ev.mode {
            ButtonMode::Deck =>{

            }
            ButtonMode::Grid =>{

            }
            ButtonMode::Inventory =>{

            }
            _ => {}
        }
    }
}
/*
fn update_inv_image(
    mut ev_hand: EventReader<HandEvent>,
    inv_query: Query<(&mut TextureAtlasSprite, &HandInv)>
){
    for ev in ev_hand.iter() {

    }
}
*/
fn select_manager(
    mut ev_inv: EventReader<InventoryEvent>,
    mut select_query: Query<(&mut ChipSelect, &mut TextureAtlasSprite)>
){
    for ev in ev_button.iter() {
        match ev.mode {
            ButtonMode::Inventory =>{
                if let Some(i) = ev.value{
                        let (mut select, mut sprite) = select_query.single_mut().unwrap();
                        select.chip_index = i;
                        sprite.index = i as u32;
                }
            }
            _ =>{}
        }
    }
}

fn deck_manager(
    mut ev_deck: EventReader<DeckEvent>,
    mut deck_query: Query<&mut Deck>,
    mut inv_query: Query<(&mut Button, &HandInv)>,
    mut ev_hand: EventWriter<HandEvent>,
){
    for ev in ev_button.iter() {
        match ev.mode {
            ButtonMode::Deck =>{
                if let Some(val) = ev.value{
                    match val {
                        1 => {
                            if let Some((mut f_button, mut hand_inv)) = inv_query.iter_mut().filter(|(x,_)|{x.value == None}).min_by_key(|(_, x)| { x.index }) {

                                let mut deck = deck_query.single_mut().unwrap();

                                if let Some(new_card) = deck.library.pop() {
                                    println!("{:?} : {:?}", new_card, f_button.value);
                                    f_button.value = Some(new_card);
                                    ev_hand.send(HandEvent{
                                        value: new_card,
                                        index: hand_inv.index,
                                    });
                                } else {
                                    //Discard to new library
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}


fn screen_to_world_pos(
    m_pos: Vec2,
    size: Vec2,
    com_mat: Mat4,
) -> Vec2{
    let p = (m_pos - size / 2.0) * CAM_SCALE;
    let pos_wld = com_mat * p.extend(0.0).extend(1.0);
    Vec2::new(pos_wld.x, pos_wld.y)
}

fn update_grid_image(
    mut grid_query: Query<(&mut TextureAtlasSprite, &mut GridChip)>,
    mut ev_grid: EventReader<GridEvent>,
    mut ev_value: EventWriter<GridValueEvent>,
    select_query: Query<&ChipSelect>,
){
    for ev in ev_button.iter() {
        let mut changed = false;
        match ev.mode {
            ButtonMode::Grid =>{
                if let Some(i) = ev.value {
                    let select = select_query.single().unwrap();
                    let (mut sprite, mut chip) = grid_query.iter_mut().find(|(_, c)| { c.index == i }).unwrap();
                    chip.chip_index = select.chip_index;
                    sprite.index = select.chip_index as u32;
                    changed = true;
                }
            }
            _ => {}
        }
        if changed {
            ev_value.send(GridValueEvent);
        }
    }
}

fn update_grid_value(
    mut value_query: Query<&mut GridValue>,
    grid_query: Query<&GridChip>,
    lib_query: Query<&ChipLib>,
    mut board_query: Query<&mut Text, With<ValueBoard>>,
    mut ev_grid: EventReader<GridValueEvent>,
){
    for _ in ev_grid.iter() {
        let mut grid_value = value_query.single_mut().unwrap();

        grid_value.off = 0;
        grid_value.def = 0;

        grid_value.flat_grid = vec![0.0; (GRID_WIDTH * GRID_HEIGHT) as usize];
        grid_value.increase_grid = vec![1.0; (GRID_WIDTH * GRID_HEIGHT) as usize];
        grid_value.more_grid = vec![1.0; (GRID_WIDTH * GRID_HEIGHT) as usize];
        grid_value.type_grid = vec![ChipType::Empty; (GRID_WIDTH * GRID_HEIGHT) as usize];
        grid_value.effect_grid = vec![1.0; (GRID_WIDTH * GRID_HEIGHT) as usize];

        let chip_lib = lib_query.single().unwrap();
        let mut value_board = board_query.single_mut().unwrap();

        grid_query.iter().for_each(|grid_chip| {
            let data = chip_lib.get(grid_chip.chip_index);
            let index = grid_chip.index as usize;

            grid_value.flat_grid[index] += data.value as f32;
            grid_value.type_grid[index] = data.c_type;
            grid_value.effect_grid[index] = data.eff;

        });

        for grid_chip in grid_query.iter() {
            let data = chip_lib.get(grid_chip.chip_index);
            let index = grid_chip.index;

            match &data.targets {
                None => {}
                Some(t) => {
                    t.iter().for_each(|i| {
                        if chip_lib.range_check(index, i) {
                            let t_index = (index + *i) as usize;
                            if data.c_type == grid_value.type_grid[t_index] {
                                grid_value.increase_grid[t_index] += data.increase;
                                grid_value.more_grid[t_index] *= data.more + 1.0;
                                grid_value.flat_grid[t_index] += data.add;
                            }
                        }
                    });
                }
            }
        }

        grid_value.off = grid_value.flat_grid.iter()
            .zip(grid_value.increase_grid.iter())
            .zip(grid_value.more_grid.iter())
            .zip(grid_value.effect_grid.iter())
            .zip(grid_value.type_grid.iter())
            .filter(|(_, t)| **t == ChipType::Offensive)
            .map(|((((f, i), m), e), _)| {
                return (f * i * m * e).floor() as i32
            }).sum();

        grid_value.def = grid_value.flat_grid.iter()
            .zip(grid_value.increase_grid.iter())
            .zip(grid_value.more_grid.iter())
            .zip(grid_value.effect_grid.iter())
            .zip(grid_value.type_grid.iter())
            .filter(|(_, t)| **t == ChipType::Defence)
            .map(|((((f, i), m), e), _)| {
                return (f * i * m * e).floor() as i32
            }).sum();

        value_board.sections[0].value = grid_value.off.to_string();
        value_board.sections[2].value = grid_value.def.to_string();
    }
}