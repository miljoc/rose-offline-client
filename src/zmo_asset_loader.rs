use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    math::{Quat, Vec3},
    reflect::TypeUuid,
};
use bevy_inspector_egui::Inspectable;
use rose_file_readers::{RoseFile, ZmoChannel, ZmoFile};

#[derive(Default)]
pub struct ZmoAssetLoader;

#[derive(Debug, Clone, Default, Inspectable)]
pub struct ZmoAssetBone {
    translation: Vec<Vec3>,
    rotation: Vec<Quat>,
}

#[derive(Debug, Clone, TypeUuid, Inspectable)]
#[uuid = "120cb5ff-e72d-4730-9756-648d0001fdfa"]
pub struct ZmoAsset {
    num_frames: usize,
    fps: usize,
    bones: Vec<ZmoAssetBone>,
}

impl ZmoAsset {
    pub fn fps(&self) -> usize {
        self.fps
    }

    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    pub fn get_translation(&self, bone_id: usize, frame_id: usize) -> Option<Vec3> {
        self.bones
            .get(bone_id)
            .and_then(|x| x.translation.get(frame_id).cloned())
    }

    pub fn get_rotation(&self, bone_id: usize, frame_id: usize) -> Option<Quat> {
        self.bones
            .get(bone_id)
            .and_then(|x| x.rotation.get(frame_id).cloned())
    }
}

impl AssetLoader for ZmoAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            match <ZmoFile as RoseFile>::read(bytes.into(), &Default::default()) {
                Ok(zmo) => {
                    // First count how many transform channels there are
                    let mut max_bone_id = 0;
                    for (bone_id, _) in zmo.channels.iter() {
                        max_bone_id = max_bone_id.max(*bone_id);
                    }

                    let mut bones = vec![ZmoAssetBone::default(); (max_bone_id + 1) as usize];
                    for (bone_id, channel) in zmo.channels.iter() {
                        let bone_animation = &mut bones[*bone_id as usize];
                        match channel {
                            ZmoChannel::Position(positions) => {
                                bone_animation.translation = positions
                                    .iter()
                                    .map(|position| {
                                        Vec3::new(position.x, position.z, -position.y) / 100.0
                                    })
                                    .collect();
                            }
                            ZmoChannel::Rotation(rotations) => {
                                bone_animation.rotation = rotations
                                    .iter()
                                    .map(|rotation| {
                                        Quat::from_xyzw(
                                            rotation.x,
                                            rotation.z,
                                            -rotation.y,
                                            rotation.w,
                                        )
                                    })
                                    .collect();
                            }
                            _ => {}
                        }
                    }
                    load_context.set_default_asset(LoadedAsset::new(ZmoAsset {
                        num_frames: zmo.num_frames,
                        fps: zmo.fps,
                        bones,
                    }));
                    Ok(())
                }
                Err(error) => Err(error),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["zmo"]
    }
}