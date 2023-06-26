pub enum FnEnum {
    GetCommands,
    RelevantFiles,
}

impl FnEnum {
    pub fn get_function(&self) -> Function {
        match self {
            FnEnum::GetCommands => {
                let properties = [Property::new(
                    "commands",
                    "array",
                    "a list of terminal commands to be executed",
                    &[
                        ("type", "string"),
                        ("description", "a terminal command string"),
                    ],
                )];
                let perameters = Perameters::new("object", &properties, &["commands"]);
                Function::new(
                    "get_commands",
                    "get a list of terminal commands to run on mac os",
                    perameters,
                )
            }
            FnEnum::RelevantFiles => {
                let properties = [Property::new(
                    "files",
                    "array",
                    "a list of files to process",
                    &[("type", "string"), ("description", "a file path string")],
                )];
                let perameters = Perameters::new("object", &properties, &["files"]);
                Function::new(
                    "relevent_files",
                    "get a list of relevant files to the given code snippet",
                    perameters,
                )
            }
        }
    }
}

pub struct Function {
    pub name: String,
    pub description: String,
    pub perameters: Perameters,
}

impl Function {
    pub fn new(name: &str, description: &str, perameters: Perameters) -> Function {
        Function {
            name: name.to_string(),
            description: description.to_string(),
            perameters,
        }
    }
    pub fn render(&self) -> String {
        let name = format!("\"name\": \"{}\"", self.name);
        let description = format!("\"description\": \"{}\"", self.description);
        format!(
            "{{ {}, {}, {} }}",
            name,
            description,
            self.perameters.render(),
        )
    }
}

pub struct Perameters {
    pub type_dec: String,
    pub properties: Box<Vec<Property>>,
    pub required: Vec<String>,
}

impl Perameters {
    pub fn new(type_dec: &str, properties: &[Property], required: &[&str]) -> Perameters {
        let type_dec = type_dec.to_string();
        let props: Vec<Property> = properties.iter().cloned().collect();
        let required = required.into_iter().map(|s| s.to_string()).collect();
        Perameters {
            type_dec,
            properties: Box::new(props),
            required,
        }
    }
    pub fn render(&self) -> String {
        let type_dec = format!("\"type\": \"{}\"", self.type_dec);
        let required = format!("\"required\": [\"{}\"]", self.required.join(", "));
        let properties = self
            .properties
            .iter()
            .map(|p| p.render())
            .collect::<Vec<String>>()
            .join(",\n");
        format!(
            " \"parameters\": {{
                {},
                \"properties\": {{
                {}
                }},
                {}
            }}",
            type_dec, properties, required
        )
    }
}

#[derive(Clone)]
pub struct Property {
    name: String,
    return_value: String,
    items: Vec<(String, String)>,
    description: String,
}

impl Property {
    pub fn new(
        name: &str,
        return_value: &str,
        description: &str,
        items: &[(&str, &str)],
    ) -> Property {
        let name = name.to_string();
        let return_value = return_value.to_string();
        let description = description.to_string();
        let items = items
            .into_iter()
            .map(|&(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        Property {
            name,
            return_value,
            items,
            description,
        }
    }
    fn render_items(&self) -> String {
        let items: &Vec<String> = &self
            .items
            .clone()
            .into_iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
            .collect::<Vec<String>>();
        format!("\"items\": {{{}}}", items.join(", "))
    }
    pub fn render(&self) -> String {
        format!(
            "\"{}\": {{\"type\": \"{}\", {}, \"description\": \"{}\"}}",
            self.name,
            self.return_value,
            self.render_items(),
            self.description
        )
    }
}