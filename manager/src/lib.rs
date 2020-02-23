#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;

use std::fmt;
use std::result::Result;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct ItemType {
    #[builder(setter(skip))]
    pub id: u32,
    pub name: String,
    #[builder(default)]
    pub minimum_quantity: f32,
    #[builder(default)]
    pub ttl: Option<Duration>,
    #[builder(default)]
    pub opened_by_default: bool,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{}",
            self.id,
            self.name,
            self.minimum_quantity,
            self.ttl
                .map(|ttl| humantime::format_duration(ttl).to_string())
                .unwrap_or("".to_string()),
            self.opened_by_default
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct ItemInstance {
    #[builder(setter(skip))]
    pub id: u32,
    pub item_type: u32,
    #[builder(default = "1.0")]
    pub quantity: f32,
    #[builder(default = "true")]
    pub alive: bool,
    #[builder(default)]
    pub model: Option<String>,
    #[builder(default)]
    pub serial: Option<String>,
    #[builder(default)]
    pub extra: Option<String>,
    #[builder(default)]
    pub location: Option<String>,
    #[builder(default)]
    pub value: Option<f32>,
    #[builder(default)]
    pub opened_at: Option<SystemTime>,
    #[builder(default)]
    pub expires_at: Option<SystemTime>,
    #[builder(setter(skip))]
    pub added_at: Option<SystemTime>,
    #[builder(setter(skip))]
    pub removed_at: Option<SystemTime>,
}

impl fmt::Display for ItemInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{};{};{};{};{}",
            self.id,
            self.item_type,
            self.quantity,
            conv(&self.model),
            conv(&self.serial),
            conv(&self.extra),
            conv(&self.location),
            conv(&self.value),
            self.opened_at
                .map(|t| humantime::format_rfc3339(t).to_string())
                .unwrap_or("".to_string()),
            self.expires_at
                .map(|t| humantime::format_rfc3339(t).to_string())
                .unwrap_or("".to_string())
        )
    }
}

pub fn conv<T: ToString>(s: &Option<T>) -> String {
    s.as_ref().map(|m| m.to_string()).unwrap_or_default()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UseState {
    New,
    Used,
}

impl Default for UseState {
    fn default() -> Self {
        UseState::New
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub item_types: Vec<ItemType>,
    pub item_instances: Vec<ItemInstance>,
}

impl Inventory {
    pub fn add_item_type(&mut self, mut item_type: ItemType) -> Result<(), InventoryError> {
        let free_id = self.free_type_id();
        item_type.id = free_id;
        self.item_types.push(item_type);
        Ok(())
    }

    pub fn add_item_instance(
        &mut self,
        mut item_instance: ItemInstance,
    ) -> Result<(), InventoryError> {
        let free_id = self.free_instance_id();
        item_instance.id = free_id;
        if !self.has_item_type(item_instance.item_type) {
            return Err(InventoryError::UnknownItemType);
        }
        // TODO check the type "open by default" thingy
        item_instance.added_at = Some(SystemTime::now());
        self.item_instances.push(item_instance);
        Ok(())
    }

    pub fn delete_item_type(&mut self, id: u32) {
        self.item_types.retain(|t| t.id != id);
        self.item_instances.retain(|i| i.item_type != id);
    }

    pub fn delete_item_instance(&mut self, id: u32) -> Result<(), InventoryError> {
        if let Some(inst) = self.item_instances.iter_mut().find(|inst| inst.id == id) {
            inst.alive = false;
            inst.removed_at = Some(SystemTime::now());
            Ok(())
        } else {
            Err(InventoryError::UnknownItemInstance)
        }
    }

    pub fn get_instances_for_type(&self, id: u32) -> Result<Vec<&ItemInstance>, InventoryError> {
        if !self.has_item_type(id) {
            return Err(InventoryError::UnknownItemType);
        }
        Ok(self
            .item_instances
            .iter()
            .filter(|inst| inst.item_type == id && inst.removed_at.is_none())
            .collect::<Vec<_>>())
    }

    fn has_item_type(&self, id: u32) -> bool {
        self.item_types.iter().any(|ty| ty.id == id)
    }

    fn free_type_id(&self) -> u32 {
        self.item_types
            .iter()
            .fold(0, |accum, ty| if ty.id > accum { ty.id } else { accum })
            + 1
    }

    fn free_instance_id(&self) -> u32 {
        self.item_instances
            .iter()
            .fold(0, |accum, ty| if ty.id > accum { ty.id } else { accum })
            + 1
    }

    pub fn get_types_for_name(&self, name: &String) -> Vec<&ItemType> {
        self.item_types
            .iter()
            .filter(|t| t.name.to_lowercase().contains(&name.to_lowercase()))
            .collect::<Vec<_>>()
    }

    pub fn quantity_for_type(&self, type_id: u32) -> f32 {
        self.item_instances
            .iter()
            .filter(|ii| ii.item_type == type_id && ii.removed_at.is_none())
            .map(|ii| ii.quantity)
            .fold(0.0, |accum, e| accum + e)
    }
}

#[derive(Debug, Clone)]
pub enum InventoryError {
    UnknownItemType,
    UnknownItemInstance,
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn write_inventory() {
        let mut inv = Inventory::default();
        let ty = ItemTypeBuilder::default()
            .id(1)
            .name("thonk type, still the type".to_string())
            .build()
            .unwrap();
        let is = ItemInstanceBuilder::default()
            .item_type(ty.id)
            .model(Some("some composit,e model".to_string()))
            .build()
            .unwrap();
        inv.item_types.push(ty);
        inv.item_instances.push(is);
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        wtr.serialize(inv.item_types).unwrap();
        wtr.flush().unwrap();
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        wtr.serialize(inv.item_instances).unwrap();
        wtr.flush().unwrap();
    }
}
