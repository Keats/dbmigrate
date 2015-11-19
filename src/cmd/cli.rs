// #![cfg_attr(feature = "dev", allow(unstable_features))]
// #![cfg_attr(feature = "dev", feature(plugin))]
// #![cfg_attr(feature = "dev", plugin(clippy))]

// #[macro_use]
// extern crate clap;
// extern crate itertools;

// use std::process::exit;

// mod files;


// fn main() {
//     let matches = clap_app!(myapp =>
//         (@setting SubcommandRequiredElseHelp)
//         (version: &crate_version!()[..])
//         (author: "Vincent Prouillet <vincent@wearewizards.io>")
//         (about: "Handles migrations for databases")
//         (@arg url: -u --url +takes_value "Sets the URL of the database to use")
//         (@arg path: -p --path +takes_value "Sets the folder containing the migrations")
//         (@subcommand create =>
//             (about: "Creates two migration files (up and down) with the given slug")
//             (@arg slug: +required "Sets the name of the migration")
//         )
//         (@subcommand sync =>
//             (about: "Apply all non-applied migrations")
//         )
//         (@subcommand rollback =>
//             (about: "Rollback the current migration to the previous one")
//         )
//         (@subcommand status =>
//             (about: "See list of migrations and which ones are applied")
//         )
//         (@subcommand goto =>
//             (about: "Go to a specific migration, rollbacking/migrating on its way")
//             (@arg migration_number: +required "Which migration to go to")
//         )
//     ).get_matches();

//     // TODO(vincent): take url and path from env?
//     let url = match matches.value_of("url") {
//         Some(url) => url,
//         None => {
//             println!("Couldn't find a database url.");
//             println!("Run migrate --help for more information.");
//             exit(1);
//         }
//     };

//     let path = match matches.value_of("path") {
//         Some(path) => path,
//         None => {
//             println!("Couldn't find the migrations path.");
//             println!("Run migrate --help for more information.");
//             exit(1);
//         }
//     };

//     println!("Value for url: {}", url);
//     println!("Value for path: {}", path);
//     // files::read_migration_files(path).unwrap();
// }
