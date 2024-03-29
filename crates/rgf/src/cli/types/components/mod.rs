use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::error::CommandError::NotCreated;
use crate::cli::error::CommandError::NotFound;
use crate::cli::result::CommandResult;
use crate::cli::types::components::args::ComponentsArgs;
use crate::cli::types::components::commands::ComponentsCommands;
use crate::client::types::components::queries::CreateComponentVariables;
use crate::client::InexorRgfClient;
use crate::table_model::types::component::Components;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn components(client: &Arc<InexorRgfClient>, component_args: ComponentsArgs) -> CommandResult {
    let Some(command) = component_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        ComponentsCommands::List => match client.types().components().get_all_components().await {
            Ok(Some(components)) => Ok(Components::from(components).into()),
            Ok(None) => Err(NoContent("No components found".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Get(args) => match client.types().components().get_component_by_type(args.clone()).await {
            Ok(Some(component)) => Ok(Components::from(component).into()),
            Ok(None) => Err(NotFound(format!("Component {}__{} not found", args.namespace, args.name))),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Create(args) => {
            let variables = CreateComponentVariables::builder()
                .namespace(args.ty.namespace)
                .name(args.ty.name)
                .description(args.description)
                // .properties(None)
                // .extensions(None)
                .build();
            match client.types().components().create_component_with_variables(variables).await {
                Ok(Some(component)) => Ok(Components::from(component).into()),
                Ok(None) => Err(NotCreated("Component wasn't created".to_string())),
                Err(e) => Err(e.into()),
            }
        }
    }
}

// fn print_component(component: Component) {
//     print_components(vec![component]);
// }
//
// fn print_components(components: Vec<Component>) {
//     let components: Vec<crate::table_model::types::component::Component> = components.into_iter().map(|p| p.into()).collect();
//     let table = Table::new(components)
//         .with(Style::extended())
//         .with(
//             Modify::new(Segment::new(1.., 0..2))
//                 .with(Width::increase(22).priority())
//                 .with(Width::truncate(25).suffix("...")),
//         )
//         .to_string();
//     println!("{}", table);
// }
