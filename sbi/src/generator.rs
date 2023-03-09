//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[allow(unused)]
use openapiv3::*;

use crate::AnyOfHandler;

use super::schema::resolve_schema_component;
use super::utils::{get_dependent_refs_for_spec, get_references_for_schema};

#[derive(Debug, Clone)]
pub struct Generator {
    specs_dir: PathBuf,
    specs: HashMap<String, SpecModule>, // A HashMap of ModuleName -> Parsed Specs
    // TODO: May be we don't need the HashMap, simply BTreeSet is enough
    references: HashMap<String, BTreeSet<String>>, // A HashMap of FileName -> References
    aux_files: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct SpecModule {
    spec_file_name: String,
    spec: OpenAPI,
    _module: String,
    input: bool,
}

impl Generator {
    /// Create an Instance of the [`Generator`] from the given Directory.
    ///
    /// Expects the given path to be a directory, which will contain one or more OpenAPI
    /// specifications that will later be processed and the code is then generated for these
    /// specification.
    pub fn from_path<P: AsRef<Path> + std::cmp::Ord>(specs_dir: P) -> std::io::Result<Self> {
        let specs_dir: PathBuf = specs_dir.as_ref().into();

        if !specs_dir.metadata()?.is_dir() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: Not a directory", specs_dir.to_string_lossy()),
            ))
        } else {
            Ok(Self {
                specs_dir,
                specs: HashMap::new(),
                references: HashMap::new(),
                aux_files: None,
            })
        }
    }

    /// Generate Definitions from the files passed as `(file, module)` tuples.
    ///
    /// Additionally, `aux_files` are specified from where referred definitions are 'hand picked'
    /// so that all the definitions in the original set of `files` can be resolved.
    ///
    /// `schema_only` is a switch used for resolving references in the path "/components/schemas"
    /// only. (This is useful for example when generating data models.)
    pub fn generate<P>(
        &mut self,
        files_modules: &[(P, &str)],
        aux_files: &[&str],
        schema_only: bool,
        handlers: Option<Vec<AnyOfHandler>>,
    ) -> std::io::Result<()>
    where
        P: AsRef<Path> + std::cmp::Ord + std::fmt::Debug,
    {
        // First we 'simply parse' the specs
        for (entry, module_name) in files_modules {
            let spec = self.parse_spec_from_file(entry)?;
            let spec_file_name = entry.as_ref().to_str().unwrap().to_string();
            self.specs.insert(
                spec_file_name.clone(),
                SpecModule {
                    spec_file_name,
                    spec,
                    _module: module_name.to_string(),
                    input: true,
                },
            );
        }

        // We Now have collected unique references In all files.
        // Check if missing files if any?
        self.find_missing_files_if_any(aux_files, schema_only)?;

        let _ = self
            .aux_files
            .replace(aux_files.iter().map(|&x| x.to_string()).collect());

        self.all_component_schemas(&handlers)?;

        Ok(())
    }

    /// Generate Rust code for all the files in the directory (See also: [`Self::from_path`])
    ///
    /// module_name: Name of the module to be used for output
    /// schema_only: Flag selecting whether only `components/schemas` to be consiered for code
    /// generation
    pub fn generate_all(
        &mut self,
        module_name: &str,
        schema_only: bool,
        handlers: Option<Vec<AnyOfHandler>>,
    ) -> std::io::Result<()> {
        let mut input_set = BTreeSet::new();
        for entry in self.specs_dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "yaml" {
                let spec_path = path.file_name().unwrap();
                let spec_path_string = spec_path.to_str().unwrap().to_string();
                let spec = self.parse_spec_from_file(spec_path)?;
                let dependent_string = spec_path_string.clone();
                let spec_file_name = spec_path_string.clone();
                input_set.insert(dependent_string);
                self.specs.insert(
                    spec_path_string,
                    SpecModule {
                        spec_file_name,
                        spec,
                        _module: module_name.to_string(),
                        input: true,
                    },
                );
            }
        }

        // Find missing files if any
        self.find_missing_files_if_any(&[], schema_only)?;

        self.all_component_schemas(&handlers)?;

        Ok(())
    }

    /// Adds a handler function to handle `AnyOf` schemas
    ///
    /// Typically `anyOf`, `oneOf` and `allOf` components are where usually the quirks of the
    /// specifications are there. To deal with such situations, it's better to ask the users of the
    /// API to provide them.

    // Parses the OpenAPI specification from a Yaml File. Errors out on any error.
    fn parse_spec_from_file<P: AsRef<Path>>(&self, file: P) -> std::io::Result<OpenAPI> {
        let file_name = file.as_ref().to_string_lossy().to_string();
        eprintln!("generating for {}", file_name);

        let full_path = self.specs_dir.canonicalize()?.join(&file_name);
        let reader = std::fs::File::open(full_path)?;
        let spec: OpenAPI = serde_yaml::from_reader(reader).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Yaml Error: {}", e))
        })?;

        Ok(spec)
    }

    // Based on the input file set and auxilary file set, report if any files that are referenced
    // are missing, If they are missing, it's an error condition that is propagated to the User.
    fn find_missing_files_if_any(
        &mut self,
        aux_files: &[&str],
        schema_only: bool,
    ) -> std::io::Result<()> {
        // First get all references
        // Now we get All references that are used by any of the specs. This is a bit involved. If
        // we are generating 'models' only, we can get those for the `components/schemas`  only,
        for v in self.specs.values() {
            let references = get_dependent_refs_for_spec(&v.spec, schema_only);
            let reference_set = BTreeSet::from_iter(references.iter().map(|v| v.clone()));
            self.references
                .insert(v.spec_file_name.clone(), reference_set);
        }

        let mut missing_files = BTreeSet::new();
        for reference_set in self.references.values() {
            for reference in reference_set {
                let split = reference.split("#").collect::<Vec<&str>>();
                let (referred_file, _referred_ref) = (split[0], split[1]);
                // Reference starts with a file name
                if !referred_file.is_empty()
                    // Refererred file name is present in input reference
                    && self
                        .references
                        .keys()
                        .find(|&file_name| file_name == referred_file)
                        .is_none()
                    // Referred file is present in Aux Files.
                    && aux_files.iter().find(|&&f| f == referred_file).is_none()
                {
                    missing_files.insert(referred_file.clone());
                }
            }
        }

        if missing_files.is_empty() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Some of the Schema Objects cannot be resolved due to missing Spec Files: {}",
                    missing_files
                        .iter()
                        .cloned()
                        .collect::<Vec<&str>>()
                        .join(", ")
                ),
            ))
        }
    }

    // Get All the Component Schemas for which we will have to generate the structures.
    // The actual function that generates the code for schemas.
    fn all_component_schemas(
        &mut self,
        handlers: &Option<Vec<AnyOfHandler>>,
    ) -> std::io::Result<()> {
        // TODO: May be we will move this outside the function. But let's see
        fn get_component_schema_from_reference_in_spec(
            spec: &OpenAPI,
            reference: &str,
        ) -> (String, Option<ReferenceOr<Schema>>) {
            let components: _ = reference.rsplit("/").collect::<Vec<_>>();
            let component = components[0];
            let schemas = &spec.components.as_ref().unwrap().schemas;
            (component.to_string(), schemas.get(component).cloned())
        }

        if self.aux_files.is_some() {
            for aux_file in self.aux_files.as_ref().unwrap() {
                let spec = self.parse_spec_from_file(aux_file)?;
                self.specs.insert(
                    aux_file.to_string(),
                    SpecModule {
                        spec_file_name: aux_file.to_string(),
                        spec,
                        _module: "empty".to_string(),
                        input: false,
                    },
                );
            }
        };

        let mut unresolved_items = vec![];
        let mut all_references = BTreeSet::<(String, String)>::new();
        for (ref_file, reference_set) in &self.references {
            for reference in reference_set {
                let parts = reference.split('#').collect::<Vec<&str>>();
                let (reference_file, reference) = (parts[0], parts[1]);
                let reference_file = if reference_file.is_empty() {
                    ref_file
                } else {
                    reference_file
                };
                eprintln!(
                    "reference_file: {}, reference: {}",
                    reference_file, reference
                );
                all_references.insert((reference_file.to_string(), reference.to_string()));

                let spec_module = self.specs.get(reference_file).unwrap();
                let (component, schema) =
                    get_component_schema_from_reference_in_spec(&spec_module.spec, reference);
                match schema.unwrap() {
                    ReferenceOr::Reference { reference } => {
                        unresolved_items.push((component.to_string(), reference.to_string()))
                    }
                    ReferenceOr::Item(s) => {
                        let mut inner_schemas = vec![s];
                        let mut loop_count = 1;
                        // during every 'pass' of the loop, we may discover newer 'local'
                        // references (ie.  references within the same file). When no further
                        // references are discovered, we are done and we break the loop.
                        loop {
                            let mut inner_refs = vec![];
                            for schema in &inner_schemas {
                                let local_refs = get_references_for_schema(schema)
                                    .iter()
                                    //.filter(|r| r.starts_with('#'))
                                    .map(|r| r.to_string())
                                    .collect::<Vec<String>>();
                                inner_refs.extend(local_refs);
                            }
                            eprintln!("loop_count:{}, inner_refs: {:#?}", loop_count, inner_refs);

                            // We are done with inner schemas for this iteration of the loop, We
                            // may get more `inner_schemas` below.
                            inner_schemas.drain(..);
                            for inner in &inner_refs {
                                let parts = inner.split('#').collect::<Vec<_>>();
                                let (rfile, reference) = (parts[0], parts[1]);
                                let rfile = if rfile.is_empty() {
                                    reference_file
                                } else {
                                    rfile
                                };
                                all_references.insert((rfile.to_string(), reference.to_string()));

                                let spec_module = self.specs.get(rfile).unwrap();
                                let (_, schema) = get_component_schema_from_reference_in_spec(
                                    &spec_module.spec,
                                    &inner,
                                );
                                match schema.unwrap() {
                                    ReferenceOr::Item(s) => inner_schemas.push(s),
                                    _ => {}
                                }
                            }
                            if inner_schemas.is_empty() {
                                break;
                            }
                            loop_count += 1;
                        }
                    }
                }
            }
        }
        //eprintln!("all_references: {:#?}", all_references);

        let mut all_schemas = HashMap::new();
        for (file, reference) in all_references {
            let spec_module = self.specs.get(&file).unwrap();
            let spec = &spec_module.spec;
            let (component, schema) = get_component_schema_from_reference_in_spec(spec, &reference);
            eprintln!(
                "reference: {}, component: {}, file: {}",
                reference, component, file
            );
            let schema = schema.unwrap();
            match schema {
                ReferenceOr::Reference { .. } => {
                    unreachable!();
                }
                ReferenceOr::Item(ref s) => {
                    all_schemas.insert(component.clone(), s.clone());
                }
            }
        }

        for spec_module in self.specs.values() {
            if !spec_module.input {
                continue;
            }

            let spec = &spec_module.spec;
            let components = spec.components.as_ref().unwrap();
            for (component, schema) in &components.schemas {
                match schema {
                    ReferenceOr::Reference { .. } => {
                        unreachable!();
                    }
                    ReferenceOr::Item(ref s) => {
                        all_schemas.insert(component.clone(), s.clone());
                    }
                }
            }
        }

        eprintln!("all_schemas count: {:#?}", all_schemas.len());

        let mut resolved_items = vec![];
        for (c, s) in all_schemas {
            resolved_items.push(resolve_schema_component(&c, &s, handlers, false)?);
        }

        let mut code = resolved_items
            .iter()
            .map(|s| s.tokens.to_string())
            .collect::<Vec<_>>();

        let aux_code = resolved_items
            .into_iter()
            .map(|s| s.aux_tokens.to_string())
            .collect::<Vec<_>>();

        code.extend(aux_code);
        code.sort();
        let code = code.join("\n");

        let code = self.rustfmt_generated_code(&code)?;

        eprintln!("unresolved_items: {}", unresolved_items.len());
        println!("{}", code);

        Ok(())
    }

    fn rustfmt_generated_code(&self, code: &str) -> std::io::Result<String> {
        let rustfmt_binary = "rustfmt"; // TODO: Get from `env` , 'custom path' etc.
        let mut cmd = Command::new(rustfmt_binary);

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn()?;
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        let code = code.to_owned();
        let stdin_handle =
            ::std::thread::spawn(move || match child_stdin.write_all(code.as_bytes()) {
                Ok(_) => code,
                Err(_) => "write error in rustfmt".to_owned(),
            });

        let mut output = vec![];
        std::io::copy(&mut child_stdout, &mut output)?;

        let status = child.wait()?;

        match String::from_utf8(output) {
            Ok(formatted_output) => match status.code() {
                Some(0) => Ok(formatted_output),
                _ => todo!(),
            },
            _ => Ok(stdin_handle.join().unwrap()),
        }
    }
}
