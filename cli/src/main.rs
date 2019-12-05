use clap::*;
use manager::*;
use prettytable::*;
use std::fs::*;
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Inventory Manager", author = "Joël Lupien <jojolepromain@gmail.com>", about = "Command line utility to manage your personal inventory.")]
pub struct Manager {
    /// Uses the inventory with this name. The files will be loaded and saved using this prefix. Defaults to \"inventory\".
    #[structopt(name = "name", short, long, default_value = "default")]
    pub inventory_name: String,
    // TODO: this doesn't actually call default_workdir?
    /// The directory to use to load and save the inventory files.
    #[structopt(short, long, default_value = "default_workdir()")]
    pub workdir: PathBuf,
    #[structopt(short, long)]
    pub minimal: bool,
    #[structopt(short, long)]
    pub fields: Option<Vec<String>>,
    #[structopt(short, long)]
    pub interactive: bool,
    #[structopt(short, long)]
    pub quiet: bool,
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(subcommand)]
    pub crud: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "ct")]
    CreateType {
        /// The name of the item type.
        name: String,
        /// The minimum quantity of this item type you want to have at all times.
        #[structopt(short, long)]
        minimum_quantity: Option<f32>,
        /// The time to live of this item type once it is opened.
        #[structopt(short, long)]
        ttl: Option<humantime::Duration>,
        /// Whether this item is in the 'opened' state by default. For example fresh food.
        #[structopt(short, long)]
        open_by_default: Option<bool>,
    },
    #[structopt(name = "rt")]
    ReadType {
        /// The id of the item type you want to view.
        #[structopt(short, long)]
        id: Option<u32>,
        // TODO
    },
    #[structopt(name = "ut")]
    UpdateType {
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
        ttl: Option<humantime::Duration>,
        /// Whether this item is in the 'opened' state by default. For example fresh food.
        #[structopt(short, long)]
        open_by_default: Option<bool>,
    },
    #[structopt(name = "dt")]
    DeleteType {
        /// The id of the item type you want to delete.
        id: u32,
    },
    #[structopt(name = "ci")]
    CreateInstance {
        item_type: u32,
        /// The quantity of this item instance. The unit is specified in the item instance. Defaults to 1.0.
        #[structopt(short, long, default_value = "1.0")]
        quantity: f32,
        #[structopt(short, long)]
        model: Option<String>,
        #[structopt(short, long)]
        serial: Option<String>,
        #[structopt(long)]
        extra: Option<String>,
        #[structopt(short, long)]
        location: Option<String>,
        #[structopt(short, long)]
        value: Option<f32>,
        #[structopt(short, long)]
        expires_at: Option<humantime::Timestamp>,
    },
    #[structopt(name = "ri")]
    ReadInstance {
        /// The id of the item type.
        id: Option<u32>,
        // TODO
    },
    #[structopt(name = "ui")]
    UpdateInstance {
        /// The id of the item type.
        id: u32,
        /// The quantity of this item instance. The unit is specified in the item instance.
        #[structopt(short, long)]
        quantity: Option<f32>,
        #[structopt(short, long)]
        model: Option<String>,
        #[structopt(short, long)]
        serial: Option<String>,
        #[structopt(long)]
        extra: Option<String>,
        #[structopt(short, long)]
        location: Option<String>,
        #[structopt(short, long)]
        value: Option<f32>,
        #[structopt(short, long)]
        expires_at: Option<Option<humantime::Timestamp>>,
        #[structopt(short, long)]
        opened_at: Option<Option<humantime::Timestamp>>,
    },
    #[structopt(name = "di")]
    DeleteInstance {
        /// The id of the item type.
        id: u32,
    },
}

fn main() {
    Manager::from_args();
    return;
    let matches = App::new("Inventory Manager")
        .version("0.1.0")
        .author("Joël Lupien <jojolepromain@gmail.com>")
        .about("Command line utility to manage your personal inventory.")
        .arg(
            Arg::with_name("inventory name")
                .short("i")
                .long("inventory")
                .help("Uses the inventory with this name. The files will be loaded and saved using this prefix. Defaults to \"inventory\"")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("workdir")
                .help("The directory to use to load and save the inventory files.")
                .short("w")
                .long("workdir")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Verbose mode"),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates an item type.")
                .arg(Arg::with_name("name").required(true).help("The name of the item type.").takes_value(true))
                .arg(Arg::with_name("minimum quantity").long("min").help("The minimum quantity of this item type you want to have at all times.").takes_value(true))
                .arg(Arg::with_name("ttl").long("ttl").help("The time to live of this item type once it is opened.").takes_value(true))
                .arg(Arg::with_name("open by default").long("open-by-default").takes_value(true).value_names(&["true/false"]).help("Whether this item is in the 'opened' state by default. For example fresh food."))
        )
        .subcommand(
            SubCommand::with_name("view")
                .about("Shows item types.")
                .arg(Arg::with_name("name").help("Filters by the name of the item type.").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("change")
                .about("Change an item type.")
                .arg(Arg::with_name("id").help("The id of the item type you want to edit.").required(true).takes_value(true))
                .arg(Arg::with_name("name").short("n").long("name").help("Set the new name of this item type.").takes_value(true))
                .arg(Arg::with_name("minimum quantity").long("min").help("The minimum quantity of this item type you want to have at all times.").takes_value(true))
                .arg(Arg::with_name("ttl").long("ttl").help("The time to live of this item type once it is opened.").takes_value(true))
                .arg(Arg::with_name("open by default").long("open-by-default").takes_value(true).value_names(&["true/false"]).help("Whether this item is in the 'opened' state by default. For example fresh food."))
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete an item type and all associated item instances.")
                .arg(Arg::with_name("id").help("The id of the item type you want to delete.").required(true).takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Create an item instance.")
                .arg(Arg::with_name("item type").required(true).help("The id of the item type.").takes_value(true))
                .arg(Arg::with_name("quantity").short("q").long("quantity").help("The quantity of this item instance. The unit is unspecified. Defaults to 1.0").takes_value(true))
                .arg(Arg::with_name("model").long("model").takes_value(true))
                .arg(Arg::with_name("serial").long("serial").takes_value(true))
                .arg(Arg::with_name("extra").long("extra").takes_value(true))
                .arg(Arg::with_name("location").long("location").takes_value(true))
                .arg(Arg::with_name("value").long("value").takes_value(true))
                .arg(Arg::with_name("expires at").long("expires-at").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit an item instance.")
                .arg(Arg::with_name("id").required(true).help("The id of the item instance.").takes_value(true))
                .arg(Arg::with_name("quantity").short("q").long("quantity").help("The quantity of this item instance. The unit is unspecified. Defaults to 1.0").takes_value(true))
                .arg(Arg::with_name("model").long("model").takes_value(true))
                .arg(Arg::with_name("serial").long("serial").takes_value(true))
                .arg(Arg::with_name("extra").long("extra").takes_value(true))
                .arg(Arg::with_name("location").long("location").takes_value(true))
                .arg(Arg::with_name("value").long("value").takes_value(true))
                .arg(Arg::with_name("expires at").long("expires-at").takes_value(true))
                .arg(Arg::with_name("opened at").long("opened-at").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("use")
                .about("Use an item instance quantity (and open it if it was not already.) If no quantity remains, the item instance will automagically be trashed.")
                .arg(Arg::with_name("id").required(true).help("The id of the item instance.").takes_value(true))
                .arg(Arg::with_name("quantity").short("q").long("quantity").help("The quantity to use of this item instance. The unit is unspecified. Defaults to 1.0").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("open")
                .about("Open an item instance if it was not already.")
                .arg(Arg::with_name("id").required(true).help("The id of the item instance.").takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("trash")
                .about("Puts an item instance in the trash.")
                .arg(Arg::with_name("id").required(true).help("The id of the item instance.").takes_value(true))
        )
        // ------------------------
        .subcommand(
            SubCommand::with_name("list-missing")
                .about("Lists item types that don't have enough item instances to fill the specified minimal quantity.")
        )
        .subcommand(
            SubCommand::with_name("list-expired")
                .about("Lists item instances with a non-zero quantity that have expired but have not yet been trashed.")
        )
        .get_matches();
    let (mut inventory, types_path, instances_path) =
        load_inventory(&matches).expect("Failed to load the inventory file");
    exec_subcommand(&matches, &mut inventory);
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
    matches: &ArgMatches<'a>,
) -> std::result::Result<(Inventory, PathBuf, PathBuf), std::io::Error> {
    let name = matches
        .value_of("inventory name")
        .unwrap_or("default")
        .to_string();
    let workdir = matches
        .value_of("workdir")
        .map(|s| PathBuf::from(s))
        .unwrap_or(default_workdir());
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
        //let item_types = csv::ReaderBuilder::new().delimiter(';' as u8).from_reader(types.as_slice()).deserialize().map(|entry| entry.expect("failed to deserialize entry")).collect::<Vec<_>>();
        let item_types =
            serde_json::from_reader(types.as_slice()).expect("Failed to deserialize type json");
        //let item_instances = csv::ReaderBuilder::new().delimiter(';' as u8).from_reader(instances.as_slice()).deserialize().map(|entry| entry.expect("failed to deserialize entry")).collect::<Vec<_>>();
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

pub fn exec_subcommand<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let (subcommand, Some(matches)) = matches.subcommand() {
        match subcommand {
            "create" => create(matches, inventory),
            "view" => view(matches, inventory),
            "change" => change(matches, inventory),
            "delete" => delete(matches, inventory),
            "add" => add(matches, inventory),
            "edit" => edit(matches, inventory),
            "use" => use_instance(matches, inventory),
            "open" => open(matches, inventory),
            "trash" => trash(matches, inventory),
            "list-missing" => print_missing(matches, inventory),
            "list-expired" => print_expired(matches, inventory),
            _ => unreachable!(),
        }
    }
}

pub fn create<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    let mut new = ItemTypeBuilder::default();
    new.name(matches.value_of("name").unwrap().to_string());
    new.minimum_quantity(
        matches
            .value_of("minimum quantity")
            .map(|min| min.parse().expect("Failed to parse minimum quantity")),
    );
    new.ttl(matches.value_of("ttl").map(|ttl| {
        ttl.parse::<humantime::Duration>()
            .expect("Failed to parse ttl duration")
            .into()
    }));
    new.opened_by_default(
        matches
            .value_of("open by default")
            .map(|open| {
                open.parse()
                    .expect("Failed to parse open by default boolean value")
            })
            .unwrap_or(false),
    );
    inventory
        .add_item_type(new.build().unwrap())
        .expect("Failed to insert new item type");
}

pub fn view<'a>(matches: &ArgMatches<'a>, inventory: &Inventory) {
    let res = if let Some(name) = matches.value_of("name") {
        inventory.get_types_for_name(&name.to_string())
    } else {
        inventory.item_types.iter().collect::<Vec<_>>()
    };
    print_item_types(&res);
}

pub fn print_item_types(types: &Vec<&ItemType>) {
    let mut table = Table::new();
    table.add_row(row!["id", "name", "min", "ttl", "open default"]);
    types.iter().for_each(|t| {
        table.add_row(row![
            t.id.to_string(),
            t.name.to_string(),
            match t.minimum_quantity {
                Some(qty) => qty.to_string(),
                None => "-".to_string(),
            },
            match t.ttl {
                Some(ttl) => humantime::format_duration(ttl).to_string(),
                None => "-".to_string(),
            },
            t.opened_by_default.to_string()
        ]);
    });
    table.printstd();
}

pub fn print_item_instances(instances: &Vec<&ItemInstance>, inv: &Inventory) {
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

pub fn change<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let Some(mut item_type) = inventory.item_types.iter_mut().find(|t| {
        t.id == matches
            .value_of("id")
            .unwrap()
            .parse::<u32>()
            .expect("Invalid id, expected unsigned integer")
    }) {
        if let Some(name) = matches.value_of("name") {
            item_type.name = name.to_string();
        }
        if let Some(min) = matches.value_of("minimum quantity") {
            if min.is_empty() {
                item_type.minimum_quantity = None;
            } else {
                item_type.minimum_quantity =
                    Some(min.parse().expect(
                        "Invalid minimum quantity, expected unsigned integer or empty string",
                    ));
            }
        }
        if let Some(ttl) = matches.value_of("ttl") {
            if ttl.is_empty() {
                item_type.ttl = None;
            } else {
                item_type.ttl = Some(ttl.parse::<humantime::Duration>().expect("Failed to parse ttl duration: Expected empty string or humantime-compatible duration.").into());
            }
        }
        if let Some(open) = matches.value_of("open by default") {
            item_type.opened_by_default = open
                .parse()
                .expect("Failed to parse open by default: Expected true or false");
        }
    } else {
        eprintln!("Could not find an item type with the specified id");
    }
}

pub fn delete<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    inventory.delete_item_type(
        matches
            .value_of("id")
            .unwrap()
            .parse()
            .expect("Failed to parse id: Expected unsigned integer"),
    );
}

pub fn add<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    let mut new = ItemInstanceBuilder::default();

    new.item_type(
        matches
            .value_of("item type")
            .unwrap()
            .parse()
            .expect("Failed to parse item id: Expected unsigned integer"),
    );
    new.model(matches.value_of("model").map(|e| e.to_string()));
    new.serial(matches.value_of("serial").map(|e| e.to_string()));
    new.extra(matches.value_of("extra").map(|e| e.to_string()));
    new.location(matches.value_of("location").map(|e| e.to_string()));
    new.value(matches.value_of("value").map(|e| {
        e.parse()
            .expect("Failed to parse value: Expected floating point number")
    }));
    new.quantity(
        matches
            .value_of("quantity")
            .map(|e| {
                e.parse()
                    .expect("Failed to parse quantity: Expected floating point number")
            })
            .unwrap_or(1.0),
    );
    new.expires_at(matches.value_of("expires at").map(|e| {
        e.parse::<humantime::Timestamp>()
            .expect("Failed to parse expires at: Invalid humantime-compatible timestamp format")
            .into()
    }));

    inventory
        .add_item_instance(new.build().unwrap())
        .expect("Failed to insert new item type");
}

pub fn edit<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory.item_instances.iter_mut().find(|t| {
        t.id == matches
            .value_of("id")
            .unwrap()
            .parse::<u32>()
            .expect("Invalid id, expected unsigned integer")
    }) {
        if let Some(e) = matches.value_of("quantity") {
            if e.is_empty() {
                item_instance.quantity = 0.0;
            } else {
                item_instance.quantity = e
                    .parse()
                    .expect("Invalid quantity, expected floating point number or empty string");
            }
        }
        if let Some(e) = matches.value_of("model") {
            if e.is_empty() {
                item_instance.model = None;
            } else {
                item_instance.model = Some(e.to_string());
            }
        }
        if let Some(e) = matches.value_of("serial") {
            if e.is_empty() {
                item_instance.serial = None;
            } else {
                item_instance.serial = Some(e.to_string());
            }
        }
        if let Some(e) = matches.value_of("extra") {
            if e.is_empty() {
                item_instance.extra = None;
            } else {
                item_instance.extra = Some(e.to_string());
            }
        }
        if let Some(e) = matches.value_of("location") {
            if e.is_empty() {
                item_instance.location = None;
            } else {
                item_instance.location = Some(e.to_string());
            }
        }
        if let Some(e) = matches.value_of("value") {
            if e.is_empty() {
                item_instance.value = None;
            } else {
                item_instance.value = Some(
                    e.parse()
                        .expect("Invalid value, expected floating point number or empty string"),
                );
            }
        }
        if let Some(e) = matches.value_of("expires-at") {
            if e.is_empty() {
                item_instance.expires_at = None;
            } else {
                item_instance.expires_at = Some(e.parse::<humantime::Timestamp>().expect("Failed to parse timestamp: Expected empty string or humantime-compatible duration.").into());
            }
        }
        if let Some(e) = matches.value_of("opened-at") {
            if e.is_empty() {
                item_instance.opened_at = None;
            } else {
                item_instance.opened_at = Some(e.parse::<humantime::Timestamp>().expect("Failed to parse timestamp: Expected empty string or humantime-compatible duration.").into());
            }
        }
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }
}

pub fn use_instance<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory.item_instances.iter_mut().find(|t| {
        t.id == matches
            .value_of("id")
            .unwrap()
            .parse::<u32>()
            .expect("Invalid id, expected unsigned integer")
    }) {
        if let Some(e) = matches.value_of("quantity") {
            if e.is_empty() {
                item_instance.quantity -= 1.0;
            } else {
                item_instance.quantity = item_instance.quantity
                    - e.parse::<f32>()
                        .expect("Invalid quantity, expected floating point number or empty string");
                if item_instance.quantity < 0.0 {
                    item_instance.quantity = 0.0;
                }
            }
        }
    } else {
        eprintln!("Could not find an item instance with the specified id");
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

pub fn trash<'a>(matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    if let Some(mut item_instance) = inventory.item_instances.iter_mut().find(|t| {
        t.id == matches
            .value_of("id")
            .unwrap()
            .parse::<u32>()
            .expect("Invalid id, expected unsigned integer")
    }) {
        item_instance.alive = false;
    } else {
        eprintln!("Could not find an item instance with the specified id");
    }
}

pub fn print_missing<'a>(_matches: &ArgMatches<'a>, inventory: &mut Inventory) {
    let v = inventory
        .item_instances
        .iter()
        .filter(|t| {
            if let Some(min) = inventory
                .item_types
                .iter()
                .find(|it| it.id == t.item_type)
                .expect("Failed to find item type for item instance")
                .minimum_quantity
            {
                t.quantity < min
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    print_item_instances(&v, &inventory);
}

pub fn print_expired<'a>(_matches: &ArgMatches<'a>, inventory: &mut Inventory) {
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
    print_item_instances(&v, &inventory);
}

