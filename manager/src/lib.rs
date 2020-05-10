#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_builder;

use std::fmt;
use std::result::Result;
use std::time::{Duration, SystemTime};
use std::ops::Add;

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
    pub fn add_item_type(&mut self, mut item_type: ItemType) -> u32 {
        let free_id = self.free_type_id();
        item_type.id = free_id;
        self.item_types.push(item_type);
        free_id
    }

    pub fn add_item_instance(
        &mut self,
        mut item_instance: ItemInstance,
    ) -> Result<u32, InventoryError> {
        let free_id = self.free_instance_id();
        item_instance.id = free_id;
        if let Some(it) = self.item_types.iter().find(|it| it.id == item_instance.item_type) {
            if it.opened_by_default {
                item_instance.opened_at = Some(SystemTime::now());
                if let Some(ttl) = it.ttl {
                    item_instance.expires_at = Some(SystemTime::now().add(ttl.clone()));
                }
            }
        } else {
            return Err(InventoryError::UnknownItemType);
        }
        // TODO check the type "open by default" thingy
        item_instance.added_at = Some(SystemTime::now());
        self.item_instances.push(item_instance);
        Ok(free_id)
    }

    pub fn use_instance<'a>(&mut self, type_id: u32, quantity: Option<f32>) {
        let mut remaining = 0.0;
        let mut trash_id = 0;
        let mut item_instances = self
            .item_instances
            .iter_mut()
            .filter(|t| t.item_type == type_id && t.removed_at.is_none())
            .collect::<Vec<_>>();
    
        let mut target = item_instances.iter_mut().find(|ii| ii.opened_at.is_some());
        if target.is_none() {
            target = item_instances.first_mut();
        }
        if let Some(item_instance) = target {
            if let Some(e) = quantity {
                item_instance.quantity = item_instance.quantity - e;
                if item_instance.quantity < 0.0 {
                    remaining = item_instance.quantity;
                    trash_id = item_instance.id;
                    item_instance.quantity = 0.0;
                }
            } else {
                item_instance.quantity -= 1.0;
            }
            if item_instance.opened_at.is_none() {
                item_instance.opened_at = Some(SystemTime::now());
                let it = self.item_types.iter().find(|it| it.id == type_id).expect("No item type found with the specified id");
                if let Some(ttl) = it.ttl {
                    let candidate_exp = SystemTime::now().add(ttl);
                    let new_exp = if let Some(old) = item_instance.expires_at {
                        if old < candidate_exp {
                            old
                        } else {
                            candidate_exp
                        }
                    } else {
                        candidate_exp
                    };
                    item_instance.expires_at = Some(new_exp);
                }
            }
        } else {
            eprintln!("Could not find an item instance with the specified type id to use (or all items were used.)");
        }
    
        if remaining < -0.0005 {
            self.trash(trash_id);
            self.use_instance(type_id, Some(-remaining));
        }
    }

    pub fn trash<'a>(&mut self, instance_id: u32) {
        if let Some(mut item_instance) = self
            .item_instances
            .iter_mut()
            .find(|t| t.id == instance_id)
        {
            item_instance.removed_at = Some(SystemTime::now());
        } else {
            eprintln!("Could not find an item instance with the specified id to trash");
        }
    }

    pub fn delete_item_type(&mut self, id: u32) {
        self.item_types.retain(|t| t.id != id);
        self.item_instances.retain(|i| i.item_type != id);
    }

    pub fn delete_item_instance(&mut self, id: u32) -> Result<(), InventoryError> {
        if let Some(inst) = self.item_instances.iter_mut().find(|inst| inst.id == id) {
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
        self.item_types.iter().map(|it| it.id).max().unwrap_or(0) + 1
    }

    fn free_instance_id(&self) -> u32 {
        self.item_instances.iter().map(|ii| ii.id).max().unwrap_or(0) + 1
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

