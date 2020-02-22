use clap::*;
use manager::*;
use prettytable::*;
use std::fs::*;
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Inventory Managoat",
    author = "Joël Lupien <jojolepromain@gmail.com>",
    about = "Command line utility to manage your personal inventory."
)]
pub struct Manager {
    /// Uses the inventory with this name. The files will be loaded and saved using this prefix. Defaults to "inventory".
    #[structopt(name = "name", short, long, default_value = "inventory")]
    pub inventory_name: String,
    /// The directory to use to load and save the inventory files.
    /// Defaults to the default configuration directory of your user.
    #[structopt(short, long)]
    pub workdir: Option<PathBuf>,
    /// Enables printing of the data without creating pretty tables.
    #[structopt(short, long)]
    pub minimal: bool,
    /// The action to execute on the inventory.
    #[structopt(subcommand)]
    pub command: Command,
}

impl Manager {
    /// Assign a default working directory if none is specified.
    pub fn fix_workdir(&mut self) {
        if self.workdir.is_none() {
            self.workdir = Some(default_workdir());
        }
    }

    /// Executes the subcommand on the inventory instance.
    pub fn exec(&self, inventory: &mut Inventory) {
        match &self.command {
            Command::CreateType(cmd) => create_type(cmd, inventory),
            Command::ReadType(cmd) => read_type(cmd, inventory, self.minimal),
            Command::UpdateType(cmd) => update_type(cmd, inventory),
            Command::DeleteType(cmd) => delete_type(cmd, inventory),
            Command::CreateInstance(cmd) => create_instance(cmd, inventory),
            Command::ReadInstance(cmd) => read_instance(cmd, inventory, self.minimal),
            Command::UpdateInstance(cmd) => update_instance(cmd, inventory),
            Command::DeleteInstance(cmd) => delete_instance(cmd, inventory),
            Command::ListExpired => print_expired(inventory, self.minimal),
            Command::ListMissing => print_missing(inventory, self.minimal),
            Command::Use { type_id, quantity } => use_instance(*type_id, *quantity, inventory),
            Command::Trash { instance_id } => trash(*instance_id, inventory),
        }
    }
}

/// The list of possible subcommands.
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Create a new item type.
    #[structopt(name = "ct")]
    CreateType(CreateTypeCommand),
    /// Print one or multiple item type data.
    #[structopt(name = "rt")]
    ReadType(ReadTypeCommand),
    /// Modify the properties of an item type.
    #[structopt(name = "ut")]
    UpdateType(UpdateTypeCommand),
    /// Delete an item type.
    #[structopt(name = "dt")]
    DeleteType(DeleteTypeCommand),
    /// Create a new item instance.
    #[structopt(name = "ci")]
    CreateInstance(CreateInstanceCommand),
    /// Print one or multiple item instance data.
    #[structopt(name = "ri")]
    ReadInstance(ReadInstanceCommand),
    /// Modify the properties of an item instance.
    #[structopt(name = "ui")]
    UpdateInstance(UpdateInstanceCommand),
    /// Delete an item instance permanently and all records of it.
    #[structopt(name = "di")]
    DeleteInstance(DeleteInstanceCommand),
    /// List expired item instances.
    #[structopt(name = "list-expired")]
    ListExpired,
    /// List item types that do not have enough item instances
    /// to satisfy their minimum quantity.
    #[structopt(name = "list-missing")]
    ListMissing,
    /// Use some quantity from an item type.
    #[structopt(name = "use")]
    Use { 
        /// The type id from which to use the specified quantity.
        type_id: u32,
        /// The quantity to use. Defaults to 1.0.
        quantity: Option<f32>
    },
    /// Put an item instance in the trash, keeping a record of its existence.
    #[structopt(name = "trash")]
    Trash { 
        /// The instance id to put to the trash.
        instance_id: u32,
    },
}

#[derive(StructOpt, Debug)]
pub struct CreateTypeCommand {
    /// The name of the item type.
    name: String,
    /// The minimum quantity of this item type you want to have at all times.
    #[structopt(short, long, default_value = "0.0")]
    minimum_quantity: f32,
    /// The time to live of this item type once it is opened.
    #[structopt(short, long)]
    ttl: Option<humantime::Duration>,
    /// Whether this item is in the 'opened' state by default. For example fresh food.
    #[structopt(short, long)]
    open_by_default: Option<bool>,
}

#[derive(StructOpt, Debug)]
pub struct ReadTypeCommand {
    /// The id of the item type you want to view.
    #[structopt(short, long)]
    id: Option<u32>,
    /// The name of the item type you want to view.
    #[structopt(short, long)]
    name: Option<String>,
}

#[derive(StructOpt, Debug)]
pub struct UpdateTypeCommand {
    /// The id of the item type you want to edit.
    id: u32,
    /// Set the new name of this item type.
    #[structopt(short, long)]
    name: Option<String>,
    /// The minimum quantity of this item type you want to have at all times.
    #[structopt(short, long)]
    minimum_quantity: Option<f32>,
    /// The time to live of this item type once it is opened.
    #[structopt(short, long)]
    ttl: Option<Option<humantime::Duration>>,
    /// Whether this item is in the 'opened' state by default. For example fresh food.
    #[structopt(short, long)]
    open_by_default: Option<bool>,
}

#[derive(StructOpt, Debug)]
pub struct DeleteTypeCommand {
    /// The id of the item type you want to delete.
    id: u32,
}

#[derive(StructOpt, Debug)]
pub struct CreateInstanceCommand {
    /// The id of the item type associated with this new item instance.
    item_type: u32,
    /// The quantity of this item instance. The unit is specified in the item instance. Defaults to 1.0.
    #[structopt(short, long, default_value = "1.0")]
    quantity: f32,
    /// The model name of this item instance.
    #[structopt(short, long)]
    model: Option<String>,
    /// The serial number of this item instance.
    #[structopt(short, long)]
    serial: Option<String>,
    /// Extra data.
    #[structopt(long)]
    extra: Option<String>,
    /// The location where this item instance is stored.
    #[structopt(short, long)]
    location: Option<String>,
    /// The monetary value of this item instance.
    #[structopt(short, long)]
    value: Option<f32>,
    /// The date/time at which this item instance expires.
    #[structopt(short, long)]
    expires_at: Option<humantime::Timestamp>,
}

#[derive(StructOpt, Debug)]
pub struct ReadInstanceCommand {
    /// The id of the item instance.
    #[structopt(short, long)]
    id: Option<u32>,
    /// The type of the associated item type.
    #[structopt(short, long)]
    type_id: Option<u32>,
    /// The name of the associated item type.
    #[structopt(long)]
    type_name: Option<String>,
    /// List only item instances that are expired.
    #[structopt(short, long)]
    expired: bool,
}

#[derive(StructOpt, Debug)]
pub struct UpdateInstanceCommand {
    /// The id of the item type.
    id: u32,
    /// The quantity of this item instance. The unit is specified in the item instance.
    #[structopt(short, long)]
    quantity: Option<f32>,
    /// The model type of this item instance.
    #[structopt(short, long)]
    model: Option<String>,
    /// The serial number of this item instance.
    #[structopt(short, long)]
    serial: Option<String>,
    /// Extra data.
    #[structopt(long)]
    extra: Option<String>,
    /// The physical location of this item instance.
    #[structopt(short, long)]
    location: Option<String>,
    /// The monetary value of this item instance.
    #[structopt(short, long)]
    value: Option<f32>,
    /// The date/time at which the item instance will expire.
    #[structopt(short, long)]
    expires_at: Option<Option<humantime::Timestamp>>,
    /// The date/time at which this item instance was used for the first time.
    #[structopt(short, long)]
    opened_at: Option<Option<humantime::Timestamp>>,
}

#[derive(StructOpt, Debug)]
pub struct DeleteInstanceCommand {
    /// The id of the item instance.
    id: u32,
}

fn main() {
    let mut manager = Manager::from_args();
    manager.fix_workdir();
    let (mut inventory, types_path, instances_path) =
        load_inventory(&manager).expect("Failed to load the inventory file");
    manager.exec(&mut inventory);
    save_inventory(&inventory, types_path, instances_path)
        .expect("Failed to save data to inventory file.");
}

pub fn default_workdir() -> PathBuf {
    let mut dir =
        dirs::data_local_dir().expect("Failed to locate suitable folder to store inventory data.");
    dir.push("inventory_manager");
    dir
}

pub fn load_inventory<'a>(
    manager: &Manager,
) -> std::result::Result<(Inventory, PathBuf, PathBuf), std::io::Error> {
    let name = manager.inventory_name.clone();
    let workdir = manager
        .workdir
        .as_ref()
        .expect("Manager::fix_workdir wasn't called before this point.");
    //let verbosity = matches.occurrences_of("v");

    if metadata(workdir.clone()).is_err() {
        DirBuilder::new().recursive(true).create(workdir.clone())?;
    }

    let mut types_path = workdir.clone();
    types_path.push(format!("{}_types.json", name));
    let mut instances_path = workdir.clone();
    instances_path.push(format!("{}_instances.json", name));

    if let (Ok(types), Ok(instances)) = (read(&types_path), read(&instances_path)) {
        // deserialize
        let item_types =
            serde_json::from_reader(types.as_slice()).expect("Failed to deserialize types json");
        let item_instances = serde_json::from_reader(instances.as_slice())
            .expect("Failed to deserialize instances json");
        Ok((
            Inventory {
                item_types,
                item_instances,
            },
            types_path,
            instances_path,
        ))
    } else {
        Ok((Inventory::default(), types_path, instances_path))
    }
}

pub fn save_inventory(
    inventory: &Inventory,
    types_path: PathBuf,
    instances_path: PathBuf,
) -> std::result::Result<(), std::io::Error> {
    let types_file = File::create(types_path)?;
    let instances_file = File::create(instances_path)?;
    serde_json::to_writer_pretty(types_file, &inventory.item_types)?;
    serde_json::to_writer_pretty(instances_file, &inventory.item_instances)?;
    Ok(())
}

pub fn create_type<'a>(cmd: &CreateTypeCommand, inventory: &mut Inventory) {
    let mut new = ItemTypeBuilder::default();
    new.name(cmd.name.clone());
    new.minimum_quantity(cmd.minimum_quantity);
    new.ttl(cmd.ttl.map(|t| t.into()));
    new.opened_by_default(cmd.open_by_default.unwrap_or(false));
    inventory
        .add_item_type(new.build().unwrap())
        .expect("Failed to insert new item type");
}

pub fn read_type<'a>(cmd: &ReadTypeCommand, inventory: &Inventory, minimal: bool) {
    let res = if let Some(id) = &cmd.id {
        inventory.item_types.iter().find(|it| it.id == *id).map(|it| vec![it]).unwrap_or(vec![])
    } else if let Some(name) = &cmd.name {
        inventory.get_types_for_name(&name.to_string())
    } else {
        inventory.item_types.iter().collect::<Vec<_>>()
    };
    print_item_types(&res, minimal);
}

pub fn read_instance<'a>(cmd: &ReadInstanceCommand, inventory: &Inventory, minimal: bool) {
    let mut instances = if let Some(id) = cmd.id {
        inventory.item_instances.iter().filter(|ii| ii.id == id).collect::<Vec<_>>()
    } else if let Some(type_id) = cmd.type_id {
        inventory.get_instances_for_type(type_id.clone()).expect("Unknown type id specified")
    } else if let Some(type_name) = &cmd.type_name {
        let types = inventory.get_types_for_name(&type_name);
        let type_ids = types.iter().map(|t| t.id).collect::<Vec<_>>();
        inventory.item_instances.iter().filter(|ii| type_ids.contains(&ii.item_type)).collect::<Vec<_>>()
    } else {
        inventory.item_instances.iter().collect::<Vec<_>>()
    };
    if cmd.expired {
        let sys_time = SystemTime::now();
        instances.retain(|ii| {
            if let Some(exp) = ii.expires_at {
                exp >= sys_time
            } else {
                false
            }
        });
    }
    print_item_instances(&instances, inventory, minimal);
}

pub fn print_item_types(types: &Vec<&ItemType>, minimal: bool) {
    if minimal {
        types.iter().for_each(|it| println!("{}", it));
    } else {
        let mut table = Table::new();
        table.add_row(row!["id", "name", "min", "ttl", "open default"]);
        types.iter().for_each(|t| {
            table.add_row(row![
                t.id.to_string(),
                t.name.to_string(),
                t.minimum_quantity.to_string(),
                match t.ttl {
                    Some(ttl) => humantime::format_duration(ttl).to_string(),
                    None => "-".to_string(),
                },
                t.opened_by_default.to_string()
            ]);
        });
        table.printstd();
    }
}

pub fn print_item_instances(instances: &Vec<&ItemInstance>, inv: &Inventory, minimal: bool) {
    if minimal {
        instances.iter().for_each(|ii| println!("{}", ii));
    } else {
        let mut table = Table::new();
        table.add_row(row![
            "id",
            "(id)item type",
            "quantity",
            "model",
            "serial",
            "extra",
            "location",
            "value",
            "opened at",
            "expires at"
        ]);
        instances.iter().for_each(|t| {
            let item_type_str = format!(
                "({}){}",
                t.item_type.to_string(),
                inv.item_types
                    .iter()
                    .find(|ty| ty.id == t.item_type)
                    .expect("Failed to find item type for item instance")
                    .name
            )
            .to_string();
            table.add_row(row![
                t.id.to_string(),
                item_type_str,
                t.quantity.to_string(),
                match &t.model {
                    Some(model) => model.to_string(),
                    None => "-".to_string(),
                },
                match &t.serial {
                    Some(serial) => serial.to_string(),
                    None => "-".to_string(),
                },
                match &t.extra {
                    Some(extra) => extra.to_string(),
                    None => "-".to_string(),
                },
                match &t.location {
                    Some(location) => location.to_string(),
                    None => "-".to_string(),
                },
                match t.value {
                    Some(value) => value.to_string(),
                    None => "-".to_string(),
                },
                match t.opened_at {
                    Some(open) => humantime::format_rfc3339(open).to_string(),
                    None => "-".to_string(),
                },
                match t.expires_at {
                    Some(exp) => humantime::format_rfc3339(exp).to_string(),
                    None => "-".to_string(),
                },
            ]);
        });
        table.printstd();
    }
}

pub fn update_type<'a>(cmd: &UpdateTypeCommand, inventory: &mut Inventory) {
    if let Some(mut item_type) = inventory.item_types.iter_mut().find(|t| t.id == cmd.id) {
        if let Some(name) = &cmd.name {
            item_type.name = name.to_string();
        }
        if let Some(min) = cmd.minimum_quantity {
            item_type.minimum_quantity = min;
        }
        if let Some(ttl_opt) = cmd.ttl {
            item_type.ttl = ttl_opt.map(|t| t.into());
        }
        if let Some(open_by_default) = cmd.open_by_default {
            item_type.opened_by_default = open_by_default;
        }
    } else {
        eprintln!("Could not find an item type with the specified id");
    }
}

pub fn delete_type<'a>(cmd: &DeleteTypeCommand, inventory: &mut Inventory) {
    inventory.delete_item_type(cmd.id);
}

pub fn delete_instance<'a>(cmd: &DeleteInstanceCommand, inventory: &mut Inventory) {
    inventory.delete_item_instance(cmd.id).expect("Failed to delete item instance. Wrong id specified");
}

pub fn create_instance<'a>(cmd: &CreateInstanceCommand, inventory: &mut Inventory) {
    let mut new = ItemInstanceBuilder::default();

    new.item_type(cmd.item_type);
    new.model(cmd.model.clone());
    new.serial(cmd.serial.clone());
    new.extra(cmd.extra.clone());
    new.location(cmd.location.clone());
    new.value(cmd.value);
    new.quantity(cmd.quantity);
    new.expires_at(cmd.expires_at.clone().map(|t| t.into()));

    inventory
        .add_item_instance(new.build().unwrap())
        .expect("Failed to insert new item type");
}

pub fn update_instance<'a>(cmd: &UpdateInstanceCommand, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory.item_instances.iter_mut().find(|t| t.id == cmd.id) {
        if let Some(e) = cmd.quantity {
            item_instance.quantity = e;
        }
        if let Some(e) = &cmd.model {
            item_instance.model = Some(e.clone());
        }
        if let Some(e) = &cmd.serial {
            item_instance.serial = Some(e.clone());
        }
        if let Some(e) = &cmd.extra {
            item_instance.extra = Some(e.clone());
        }
        if let Some(e) = &cmd.location {
            item_instance.location = Some(e.clone());
        }
        if let Some(e) = &cmd.value {
            item_instance.value = Some(e.clone());
        }
        if let Some(e) = &cmd.expires_at {
            item_instance.expires_at = e.clone().map(|t| t.into());
        }
        if let Some(e) = &cmd.opened_at {
            item_instance.opened_at = e.clone().map(|t| t.into());
        }
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }
}

pub fn use_instance<'a>(type_id: u32, quantity: Option<f32>, inventory: &mut Inventory) {
    let mut remaining = 0.0;
    let mut trash_id = 0;
    let mut item_instances = inventory
        .item_instances
        .iter_mut()
        .filter(|t| t.id == type_id);

    let mut target = item_instances.find(|ii| ii.opened_at.is_some());
    if target.is_none() {
        target = item_instances.next();
    }
    if let Some(item_instance) = target {
        if let Some(e) = quantity {
            item_instance.quantity = item_instance.quantity - e;
            if item_instance.quantity < 0.0 {
                remaining = -item_instance.quantity;
                trash_id = item_instance.id;
                item_instance.quantity = 0.0;
            }
        } else {
            item_instance.quantity -= 1.0;
        }
        item_instance.removed_at = Some(SystemTime::now());
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }

    if remaining < -0.005 {
        trash(trash_id, inventory);
        use_instance(type_id, Some(remaining), inventory);
    }
}

pub fn open<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory.item_instances.iter_mut().find(|t| {
        t.id == matches
            .value_of("id")
            .unwrap()
            .parse::<u32>()
            .expect("Invalid id, expected unsigned integer")
    }) {
        item_instance.opened_at = Some(SystemTime::now().into());
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }
}

pub fn trash<'a>(instance_id: u32, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory
        .item_instances
        .iter_mut()
        .find(|t| t.id == instance_id)
    {
        item_instance.alive = false;
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }
}

pub fn print_missing<'a>(inventory: &mut Inventory, minimal: bool) {
    let types = inventory
        .item_types
        .iter()
        .filter(|t| {
            inventory.item_instances.iter().filter(|ii| ii.item_type == t.id).map(|ii| ii.quantity).fold(0.0, |accum, e| accum + e) < t.minimum_quantity
        }).collect::<Vec<_>>();
    print_item_types(&types, minimal);
}

pub fn print_expired<'a>(inventory: &mut Inventory, minimal: bool) {
    let v = inventory
        .item_instances
        .iter()
        .filter(|t| {
            if let Some(expiry) = t.expires_at {
                SystemTime::now() > expiry.into()
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    print_item_instances(&v, &inventory, minimal);
}
