use std::collections::HashMap;

use bevy::prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut, Vec2};
use bevy_egui::{egui, EguiContext};
use enum_map::{enum_map, Enum, EnumMap};

use rose_file_readers::{IdFile, TsiFile, TsiSprite, TsiTexture, VfsIndex};

use crate::{ui::widgets::Dialog, VfsResource};

#[derive(Clone)]
pub struct UiSprite {
    pub texture_id: egui::TextureId,
    pub uv: egui::Rect,
    pub width: f32,
    pub height: f32,
}

impl UiSprite {
    pub fn draw(&self, ui: &mut egui::Ui, pos: egui::Pos2) {
        let rect = egui::Rect::from_min_size(pos, egui::vec2(self.width, self.height));
        let mut mesh = egui::epaint::Mesh::with_texture(self.texture_id);
        mesh.add_rect_with_uv(rect, self.uv, egui::Color32::WHITE);
        ui.painter().add(egui::epaint::Shape::mesh(mesh));
    }

    pub fn draw_stretched(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let mut mesh = egui::epaint::Mesh::with_texture(self.texture_id);
        mesh.add_rect_with_uv(rect, self.uv, egui::Color32::WHITE);
        ui.painter().add(egui::epaint::Shape::mesh(mesh));
    }
}

#[derive(Enum)]
pub enum UiSpriteSheetType {
    Ui,
    ExUi,
}

pub struct UiTexture {
    pub handle: Handle<Image>,
    pub texture_id: egui::TextureId,
    pub size: Option<Vec2>,
}

pub struct UiSpriteSheet {
    pub textures: Vec<TsiTexture>,
    pub sprites: Vec<TsiSprite>,
    pub loaded_textures: Vec<UiTexture>,
    pub sprites_by_name: IdFile,
}

pub struct UiResources {
    pub loaded_all_textures: bool,
    pub sprite_sheets: EnumMap<UiSpriteSheetType, UiSpriteSheet>,

    pub dialog_files: HashMap<String, Handle<Dialog>>,
    pub dialog_login: Handle<Dialog>,
    pub dialog_character_info: Handle<Dialog>,
    pub dialog_chatbox: Handle<Dialog>,
    pub dialog_select_avatar: Handle<Dialog>,
    pub dialog_create_avatar: Handle<Dialog>,
    pub dialog_game_menu: Handle<Dialog>,
    pub dialog_minimap: Handle<Dialog>,
    pub dialog_player_info: Handle<Dialog>,
    pub dialog_select_server: Handle<Dialog>,
    pub dialog_skill_list: Handle<Dialog>,
}

impl UiResources {
    pub fn get_sprite(&self, module_id: i32, sprite_name: &str) -> Option<UiSprite> {
        let sprite_sheet_type = match module_id {
            0 => UiSpriteSheetType::Ui,
            3 => UiSpriteSheetType::ExUi,
            _ => return None,
        };
        let sprite_sheet = &self.sprite_sheets[sprite_sheet_type];
        let sprite_index = sprite_sheet.sprites_by_name.get(sprite_name)?;

        let sprite = sprite_sheet.sprites.get(*sprite_index as usize)?;
        let texture = sprite_sheet
            .loaded_textures
            .get(sprite.texture_id as usize)?;
        let texture_size = texture.size?;

        Some(UiSprite {
            texture_id: texture.texture_id,
            uv: egui::Rect::from_min_max(
                egui::pos2(
                    (sprite.left as f32 + 0.5) / texture_size.x,
                    (sprite.top as f32 + 0.5) / texture_size.y,
                ),
                egui::pos2(
                    (sprite.right as f32 - 0.5) / texture_size.x,
                    (sprite.bottom as f32 - 0.5) / texture_size.y,
                ),
            ),
            width: (sprite.right - sprite.left) as f32,
            height: (sprite.bottom - sprite.top) as f32,
        })
    }
}

fn load_ui_spritesheet(
    vfs: &VfsIndex,
    asset_server: &AssetServer,
    egui_context: &mut EguiContext,
    tsi_path: &str,
    id_path: &str,
) -> Result<UiSpriteSheet, anyhow::Error> {
    let tsi_file = vfs.read_file::<TsiFile, _>(tsi_path)?;
    let id_file = vfs.read_file::<IdFile, _>(id_path)?;

    let mut loaded_textures = Vec::new();
    for tsi_texture in tsi_file.textures.iter() {
        let handle = asset_server.load(&format!("3DDATA/CONTROL/RES/{}", tsi_texture.filename));
        let texture_id = egui_context.add_image(handle.clone_weak());
        loaded_textures.push(UiTexture {
            handle,
            texture_id,
            size: None,
        });
    }

    Ok(UiSpriteSheet {
        textures: tsi_file.textures,
        sprites: tsi_file.sprites,
        loaded_textures,
        sprites_by_name: id_file,
    })
}

pub fn update_ui_resources(mut ui_resources: ResMut<UiResources>, images: Res<Assets<Image>>) {
    if ui_resources.loaded_all_textures {
        return;
    }

    let mut loaded_all = true;

    for (_, spritesheet) in ui_resources.sprite_sheets.iter_mut() {
        for texture in spritesheet.loaded_textures.iter_mut() {
            if texture.size.is_some() {
                continue;
            }

            if let Some(image) = images.get(&texture.handle) {
                texture.size = Some(image.size());
            } else {
                loaded_all = false;
            }
        }
    }

    ui_resources.loaded_all_textures = loaded_all;
}

pub fn load_ui_resources(
    mut commands: Commands,
    vfs_resource: Res<VfsResource>,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
) {
    let vfs = &vfs_resource.vfs;

    let dialog_filenames = [
        "DELIVERYSTORE.XML",
        "DLGADDFRIEND.XML",
        "DLGAVATA.XML",
        "DLGAVATARSTORE.XML",
        "DLGBANK.XML",
        "DLGCHAT.XML",
        "DLGCHATFILTER.XML",
        "DLGCHATROOM.XML",
        "DLGCLAN.XML",
        "DLGCLANREGNOTICE.XML",
        "DLGCOMM.XML",
        "DLGCREATEAVATAR.XML",
        "DLGDEAL.XML",
        "DLGDIALOG.XML",
        "DLGDIALOGEVENT.XML",
        "DLGEXCHANGE.XML",
        "DLGGOODS.XML",
        "DLGHELP.XML",
        "DLGINFO.XML",
        "DLGINPUTNAME.XML",
        "DLGITEM.XML",
        "DLGLOGIN.XML",
        "DLGMAKE.XML",
        "DLGMEMO.XML",
        "DLGMEMOVIEW.XML",
        "DLGMENU.XML",
        "DLGMINIMAP.XML",
        "DLGNINPUT.XML",
        "DLGNOTIFY.XML",
        "DLGOPTION.XML",
        "DLGORGANIZECLAN.XML",
        "DLGPARTY.XML",
        "DLGPARTYOPTION.XML",
        "DLGPRIVATECHAT.XML",
        "DLGPRIVATESTORE.XML",
        "DLGQUEST.XML",
        "DLGQUICKBAR.XML",
        "DLGRESTART.XML",
        "DLGSELAVATAR.XML",
        "DLGSELECTEVENT.XML",
        "DLGSELONLYSVR.XML",
        "DLGSELSVR.XML",
        "DLGSEPARATE.XML",
        "DLGSKILL.XML",
        "DLGSKILLTREE.XML",
        "DLGSTORE.XML",
        "DLGSYSTEM.XML",
        "DLGSYSTEMMSG.XML",
        "DLGUPGRADE.XML",
        "MSGBOX.XML",
        "SKILLTREE_DEALER.XML",
        "SKILLTREE_HOWKER.XML",
        "SKILLTREE_MUSE.XML",
        "SKILLTREE_SOLDIER.XML",
    ];

    let mut dialog_files = HashMap::new();
    for filename in dialog_filenames {
        dialog_files.insert(
            filename.to_string(),
            asset_server.load(&format!("3DDATA/CONTROL/XML/{}", filename)),
        );
    }

    commands.insert_resource(UiResources {
        loaded_all_textures: false,
        sprite_sheets: enum_map! {
            UiSpriteSheetType::Ui => load_ui_spritesheet(vfs, &asset_server, &mut egui_context, "3DDATA/CONTROL/RES/UI.TSI", "3DDATA/CONTROL/XML/UI_STRID.ID").expect("Failed to load UI sprite sheet"),
            UiSpriteSheetType::ExUi => load_ui_spritesheet(vfs, &asset_server, &mut egui_context,  "3DDATA/CONTROL/RES/EXUI.TSI", "3DDATA/CONTROL/XML/EXUI_STRID.ID").expect("Failed to load EXUI sprite sheet"),
        },
        dialog_character_info: dialog_files["DLGAVATA.XML"].clone(),
        dialog_chatbox: dialog_files["DLGCHAT.XML"].clone(),
        dialog_create_avatar: dialog_files[
            "DLGCREATEAVATAR.XML"].clone(),
        dialog_game_menu: dialog_files["DLGMENU.XML"].clone(),
        dialog_login: dialog_files["DLGLOGIN.XML"].clone(),
        dialog_minimap: dialog_files["DLGMINIMAP.XML"].clone(),
        dialog_player_info: dialog_files["DLGINFO.XML"].clone(),
        dialog_select_avatar: dialog_files[
            "DLGSELAVATAR.XML"].clone(),
        dialog_select_server: dialog_files["DLGSELSVR.XML"].clone(),
        dialog_skill_list: dialog_files["DLGSKILL.XML"].clone(),
        dialog_files,
    });
}
