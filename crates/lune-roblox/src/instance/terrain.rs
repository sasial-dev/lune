use mlua::prelude::*;
use rbx_dom_weak::types::{MaterialColors, TerrainMaterials, Variant};

use crate::{
    datatypes::types::{Color3, EnumItem},
    shared::classes::{add_class_restricted_method, add_class_restricted_method_mut},
};

use super::Instance;

pub const CLASS_NAME: &str = "Terrain";

pub fn add_methods<M: LuaUserDataMethods<Instance>>(methods: &mut M) {
    add_class_restricted_method(
        methods,
        CLASS_NAME,
        "GetMaterialColor",
        terrain_get_material_color,
    );

    add_class_restricted_method_mut(
        methods,
        CLASS_NAME,
        "SetMaterialColor",
        terrain_set_material_color,
    );
}

fn get_or_create_material_colors(instance: &Instance) -> MaterialColors {
    if let Some(Variant::MaterialColors(inner)) = instance.get_property("MaterialColors") {
        inner
    } else {
        MaterialColors::default()
    }
}

/**
    Returns the color of the given terrain material.

    ### See Also
    * [`GetMaterialColor`](https://create.roblox.com/docs/reference/engine/classes/Terrain#GetMaterialColor)
      on the Roblox Developer Hub
*/
fn terrain_get_material_color(_: &Lua, this: &Instance, material: EnumItem) -> LuaResult<Color3> {
    let material_colors = get_or_create_material_colors(this);

    if &material.parent.desc.name != "Material" {
        return Err(LuaError::RuntimeError(format!(
            "Expected Enum.Material, got Enum.{}",
            &material.parent.desc.name
        )));
    }

    let terrain_material = material
        .name
        .parse::<TerrainMaterials>()
        .map_err(|err| LuaError::RuntimeError(err.to_string()))?;

    Ok(material_colors.get_color(terrain_material).into())
}

/**
    Sets the color of the given terrain material.

    ### See Also
    * [`SetMaterialColor`](https://create.roblox.com/docs/reference/engine/classes/Terrain#SetMaterialColor)
      on the Roblox Developer Hub
*/
fn terrain_set_material_color(
    _: &Lua,
    this: &mut Instance,
    args: (EnumItem, Color3),
) -> LuaResult<()> {
    let mut material_colors = get_or_create_material_colors(this);
    let material = args.0;
    let color = args.1;

    if &material.parent.desc.name != "Material" {
        return Err(LuaError::RuntimeError(format!(
            "Expected Enum.Material, got Enum.{}",
            &material.parent.desc.name
        )));
    }

    let terrain_material = material
        .name
        .parse::<TerrainMaterials>()
        .map_err(|err| LuaError::RuntimeError(err.to_string()))?;

    material_colors.set_color(terrain_material, color.into());
    this.set_property("MaterialColors", Variant::MaterialColors(material_colors));
    Ok(())
}
