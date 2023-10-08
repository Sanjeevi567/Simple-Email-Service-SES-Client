use aws_apis::{
    load_credential_from_env, CredentInitialize, SesOps, SimpleMail, Simple_, TemplateMail,
    Template_,
};
use colored::Colorize;
use dotenv::dotenv;
use inquire::{
    ui::{Attributes, RenderConfig, StyleSheet, Styled},
    Confirm, Select, Text,
};
use reqwest::get;
use std::{env::var, fs::OpenOptions, io::Read};
#[tokio::main]
async fn main() {
    inquire::set_global_render_config(global_render_config());
    let operations: Vec<&str> = vec![
        "Verify the Credential\n",
        "Print Credentials Information\n",
        "AWS Simple Email Service(SES) Operations\n",
        "Quit the application\n",
    ];
    let mut credential = CredentInitialize::default();
    let mut ses_ops: SesOps = SesOps::build(credential.build());
    'main: loop {
        let choice = Select::new(
            "Select the option to execute the operation\n",
            operations.clone(),
        )
        .with_help_message(
            "Don't enclose data in quotation marks or add spaces around it in any operations",
        )
        .with_page_size(4)
        .prompt()
        .unwrap();

        match choice {
            "Verify the Credential\n" => {
                let choices = Confirm::new("Load the credentials from the configuration file or from environment variables\n")
                          .with_placeholder("Use 'Yes' to load from the environment and 'No' to load from environment variables\n")
                          .with_help_message("Without proper credentials, no operations can be executed successfully")
                          .prompt()
                          .unwrap();

                match choices {
                    true => {
                        let (credentials, region) = load_credential_from_env().await;
                        credential.update(
                            credentials.access_key_id(),
                            credentials.secret_access_key(),
                            region.as_deref(),
                        );
                        let config = credential.build();
                        ses_ops = SesOps::build(config.clone());
                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".blue().bold());
                    }
                    false => {
                        dotenv().ok();
                        let access_key = var("AWS_ACCESS_KEY_ID")
                        .expect("Ensure that the 'AWS_ACCESS_KEY_ID' environment variable is set, and its value is provided by AWS\n");
                        let secret_key = var("AWS_SECRET_ACCESS_KEY")
                        .expect("Ensure that the 'AWS_SECRET_ACCESS_KEY' environment variable is set, and its value is provided by AWS\n");
                        let region = var("AWS_DEFAULT_REGION")
                        .expect("Ensure that the 'AWS_DEFAULT_REGION' environment variable is set, and its value is provided by AWS\n");
                        credential.update(&access_key, &secret_key, Some(&region));
                        let config = credential.build();
                        ses_ops = SesOps::build(config.clone());
                        println!("{}\n","Please verify the credentials by printing the credential information before proceeding with any operations".red().bold());
                    }
                }
            }
            "Print Credentials Information\n" => {
                let confirm = Confirm::new(
                    "Are you sure you want to print credential information?\n",
                )
                .with_formatter(&|str| format!(".....{str}.....\n"))
                .with_placeholder(
                    "Type 'Yes' to view the credentials, or 'No' to not view the credentials\n",
                )
                .with_help_message("This is solely for verification purposes")
                .with_default(false)
                .prompt()
                .unwrap();

                match confirm {
                    true => {
                        println!("Here is your credential informations");
                        println!("{:#?}\n", credential.get_credentials());
                    }
                    false => {
                        println!("{}\n", "Sure...".green().bold())
                    }
                }
            }
            "AWS Simple Email Service(SES) Operations\n" => {
                let ses_operations = vec![
                    "Create a Contact List Name\n",
                    "Add an email to the list\n",
                    "Send a Single Simple Email\n",
                    "Send a Bulk of Simple Emails\n",
                    "Default Values\n",
                    "Create Email Template\n",
                    "Get Email Template\n",
                    "Get Email Template Variables\n",
                    "Send a Single Templated Email\n",
                    "Send a Bulk of Templated Emails\n",
                    "Retrieve emails from the provided list\n",
                    "Create Email Identity\n",
                    "Email Verification\n",
                    "Get Email Identities\n",
                    "Update Email Template\n",
                    "Delete Template\n",
                    "Delete Contact List Name\n",
                    "Common Errors\n",
                    "Return to the Main Menu\n",
                ];
                loop {
                    let email_choice = Select::new(
                        "Select the option to execute the operation\n",
                        ses_operations.clone(),
                    )
                    .with_help_message("Do not enclose it with quotation marks or add spaces")
                    .with_vim_mode(true)
                    .with_page_size(13)
                    .prompt()
                    .unwrap();

                    match email_choice {
                        "Create Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!("Please note that these template names are already available for your use:\n{:#?}",get_available_template_names);
                            let template_name = Text::new(
                                "Please provide the new template name for this template\n",
                            )
                            .with_placeholder(&placeholder_info)
                            .with_formatter(&|input| {
                                format!("Received Template Name Is: {input}\n")
                            })
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                            let subject_path =Text::new("Please provide the path to the subject data in HTML format to create Subject for Email Template\n")
                        .with_placeholder("The subject can contain template variables to personalize the email template's subject line\nDo not use apostrophes, spaces, or commas around template variables\n")
                        .with_help_message("An example subject template is available here https://tinyurl.com/4etkub75 ")
                        .with_formatter(&|input| format!("Received Subject Is: {input}\n"))
                        .prompt()
                        .unwrap();

                            let template_path = Text::new("Please provide the path for the template in HTML format to Create a HTML body for the Email Template\n")
                              .with_formatter(&|input| format!("Received Template Path Is: {input}\n"))
                              .with_placeholder("The HTML body can contain both template variables and HTML content\n")
                              .with_help_message("Example template is available at this location: https://tinyurl.com/rmxwfc5v")
                              .prompt()
                              .unwrap();

                            let text_path =Text::new("Please provide the path to the text body for the email template\n")
                        .with_placeholder("This section is optional, but it's essential to include for recipients who do not support HTML\n")
                        .with_formatter(&|input| format!("Received Text Body Is: {input}\n"))
                        .with_help_message("Example text body data is available here https://tinyurl.com/ycy4sbmn")
                        .prompt_skippable()
                        .unwrap()
                        .unwrap();
                            match (
                                template_name.is_empty(),
                                subject_path.is_empty(),
                                template_path.is_empty(),
                            ) {
                                (false, false, false) => {
                                    let mut reading_template_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&template_path)
                                        .expect(
                                            "Error opening the Template file path you specified\n",
                                        );
                                    let mut template_data = String::new();
                                    reading_template_data
                                        .read_to_string(&mut template_data)
                                        .expect("Error while reading data\n");
                                    let mut reading_subject_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&subject_path)
                                        .expect(
                                            "Error opening the Subject file path you specified\n",
                                        );
                                    let mut subject_data = String::new();
                                    reading_subject_data
                                        .read_to_string(&mut subject_data)
                                        .expect("Error while reading data\n");

                                    match text_path.is_empty() {
                                        false => {
                                            let mut reading_text_data = OpenOptions::new()
                                                         .read(true)
                                                         .write(true)
                                                         .open(&text_path)
                                                         .expect("Error opening the Text Body file path you specified\n");
                                            let mut text_data = String::new();
                                            reading_text_data
                                                .read_to_string(&mut text_data)
                                                .expect(
                                                    "Error opening the file path you specified\n",
                                                );

                                            ses_ops
                                                .create_email_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    Some(text_data),
                                                )
                                                .await;
                                        }
                                        true => {
                                            ses_ops
                                                .create_email_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                _ => {
                                    println!("{}\n", "Fields should not be left empty".red().bold())
                                }
                            }
                        }
                        "Update Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new(
                        "Please provide the template name to update the associated template\n",)
                    .with_placeholder(&placeholder_info)
                    .with_formatter(&|input| format!("Received Template Name Is: {input}\n"))
                    .prompt()
                    .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    let (current_subject, current_template_html, current_text) =
                                        ses_ops
                                            .get_template_subject_html_and_text(
                                                &template_name,
                                                false,
                                            )
                                            .await;
                                    let current_subject = format!(
                                        "Your current email template subject is:\n {}",
                                        current_subject
                                    );
                                    let subject_path =Text::new("Please provide the path to the subject data in JSON format to update\n")
                        .with_placeholder(&current_subject)
                        .with_formatter(&|input| format!("Received Subject Is: {input}\n"))
                        .prompt()
                        .unwrap();
                                    let current_template_variables = ses_ops
                                        .get_template_variables_of_subject_and_html_body(
                                            &current_subject,
                                            &current_template_html,
                                        );
                                    let current_template_variables = format!("These are the current template variables in the template named '{}'\n{}",template_name,current_template_variables.1.join("\n"));
                                    let template_path = Text::new("Please provide the path for the template in JSON format to update it with the old one\n")
                              .with_formatter(&|input| format!("Received Template Path Is: {input}\n"))
                              .with_placeholder(&current_template_variables)
                              .with_help_message("Example template is available at this location: https://tinyurl.com/4na92rph")
                              .prompt()
                              .unwrap();
                                    let current_text = format!(
                                        "Your current email template text is:\n{}\n",
                                        current_text
                                    );
                                    let text_path =Text::new("Please provide the path to the text body for the email template\n")
                        .with_placeholder(&current_text)
                        .with_help_message("This section is optional, but it's essential to include for recipients who do not support HTML")
                        .with_formatter(&|input| format!("Received Text Body Is: {input}\n"))
                        .prompt_skippable()
                        .unwrap()
                        .unwrap();
                                    let mut reading_template_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&template_path)
                                        .expect(
                                            "Error opening the Template file path you specified\n",
                                        );
                                    let mut template_data = String::new();
                                    reading_template_data
                                        .read_to_string(&mut template_data)
                                        .expect("Error while reading template data\n");
                                    let mut reading_subject_data = OpenOptions::new()
                                        .read(true)
                                        .write(true)
                                        .open(&subject_path)
                                        .expect(
                                            "Error opening the Subject file path you specified",
                                        );
                                    let mut subject_data = String::new();
                                    reading_subject_data
                                        .read_to_string(&mut subject_data)
                                        .expect("Error while reading subject data\n");

                                    match text_path.is_empty() {
                                        false => {
                                            let mut read_text_data = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(&text_path)
                            .expect("Error opening the Text Body file path you specified\n");
                                            let mut text = String::new();
                                            read_text_data
                                                .read_to_string(&mut text)
                                                .expect("Error While Reading to String ");
                                            ses_ops
                                                .update_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    Some(text),
                                                )
                                                .await;
                                        }
                                        true => {
                                            ses_ops
                                                .update_template(
                                                    &template_name,
                                                    &subject_data,
                                                    &template_data,
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Get Email Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new("Please provide the template name\n")
                                .with_placeholder(&placeholder_info)
                                .with_formatter(&|input| {
                                    format!("Received Template Name Is: {input}\n")
                                })
                                .prompt()
                                .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    ses_ops
                                        .get_template_subject_html_and_text(&template_name, true)
                                        .await;
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Get Email Template Variables\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Template Names in Your Credentials\n{:#?}",
                                get_available_template_names
                            );
                            let template_name = Text::new(
                                "Please provide the new template name for this template\n",
                            )
                            .with_placeholder(&placeholder_info)
                            .with_formatter(&|input| {
                                format!("Received Template Name Is: {input}\n")
                            })
                            .prompt_skippable()
                            .unwrap()
                            .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    let (subject_data, template_data, _) = ses_ops
                                        .get_template_subject_html_and_text(&template_name, false)
                                        .await;
                                    let (subject_variables, html_variables) = ses_ops
                                        .get_template_variables_of_subject_and_html_body(
                                            &subject_data,
                                            &template_data,
                                        );
                                    println!(
                                        "{}\n",
                                        "Subject Template Variables if any".yellow().bold()
                                    );
                                    subject_variables.into_iter().for_each(|variable| {
                                        println!("    {}", variable.green().bold());
                                    });
                                    println!("");
                                    println!(
                                        "{}\n",
                                        "HTML Template Variables if any".yellow().bold()
                                    );
                                    html_variables.into_iter().for_each(|variable| {
                                        println!("    {}", variable.green().bold());
                                    });
                                    println!("");
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }
                        "Delete Template\n" => {
                            let get_available_template_names = ses_ops.list_email_templates().await;
                            let placeholder_info = format!(
                                "Available Templates \n{:#?}",
                                get_available_template_names
                            );
                            let template_name =
                                Text::new("Please provide the template name for deletion\n")
                                    .with_placeholder(&placeholder_info)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            match template_name.is_empty() {
                                false => {
                                    ses_ops.delete_template(&template_name).await;
                                }
                                true => {
                                    println!("{}\n", "Template Name can't be empty".red().bold())
                                }
                            }
                        }

                        "Create a Contact List Name\n" => {
                            let lst_name = Text::new(
                                "Enter the list name to add to the AWS Simple Email Service\n",
                            )
                            .with_placeholder("The name should be unique\n")
                            .with_formatter(&|str| format!(".....{str}.....\n"))
                            .with_help_message("This is where the emails are stored")
                            .prompt()
                            .unwrap();
                            let description = Text::new("Small Description about the list name\n")
            .with_placeholder("Eg: A list named 'Zone Email Contacts' is used to add the emails\nof people in a specific area but can be skipped\n")
            .with_formatter(&|str| format!(".....{str}.....\n"))
            .prompt_skippable()
            .unwrap()
            .unwrap();
                            match (lst_name.is_empty(), description.is_empty()) {
                                (false, false) => {
                                    ses_ops
                                        .create_contact_list_name(&lst_name, Some(description))
                                        .await;
                                }
                                (false, true) => {
                                    ses_ops.create_contact_list_name(&lst_name, None).await;
                                }
                                _ => println!(
                                    "{}\n",
                                    "Contact Name Can't be empty..try again".red().bold()
                                ),
                            }
                        }
                        "Delete Contact List Name\n" => {
                            let get_available_contact_lists = ses_ops.list_contact_lists().await;
                            let contact_list_names = format!(
                                "Available Contact List Names:\n{:#?}\n",
                                get_available_contact_lists
                            );
                            let lst_name = Text::new("Enter the Contact List name to delete from AWS Simple Email Service\n")
            .with_placeholder(&contact_list_names)
            .with_formatter(&|str| format!(".....{str}.....\n"))
            .with_help_message("This is where the emails are stored")
            .prompt()
            .unwrap();
                            match lst_name.is_empty() {
                                false => {
                                    ses_ops.delete_contact_list_name(&lst_name).await;
                                }
                                true => println!(
                                    "{}\n",
                                    "Contact List Name can't be empty".red().bold()
                                ),
                            }
                        }

                        "Add an email to the list\n" => {
                            let get_contact_list_name = ses_ops.get_list_name();
                            let get_contact_list_name =
                                format!("Default contact list name: {}\n", get_contact_list_name);
                            let email = Text::new("Enter the email\n")
                                .with_placeholder(
                                    "Emails should be without quotation marks around them\n",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            let list_name =
                                Text::new("Enter the list name you want the email add in it\n")
                                    .with_placeholder(&get_contact_list_name)
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt_skippable()
                                    .unwrap()
                                    .unwrap();
                            let to_verified = Confirm::new("Would you like to send the verification email as well?\n")
                                   .with_formatter(&|str| format!(".....{str}.....\n"))
                                   .with_placeholder("Selecting 'Yes' means you want to receive a verification, while choosing 'No' means your email will be added to the list without verification\n")
                                   .prompt()
                                   .unwrap();

                            match (list_name.is_empty(), email.is_empty(), to_verified) {
                                (false, false, false) => {
                                    ses_ops
                                        .create_email_contact_without_verification(
                                            &email,
                                            Some(&list_name),
                                        )
                                        .await;
                                    println!("You must pass the email '{}' to the 'Create Email Identity' option before sending an email to this address\n",email.yellow().bold());
                                }
                                (false, false, true) => {
                                    ses_ops
                                        .create_email_contact_with_verification(
                                            &email,
                                            Some(&list_name),
                                        )
                                        .await;
                                }
                                (true, false, false) => {
                                    ses_ops
                                        .create_email_contact_without_verification(&email, None)
                                        .await;
                                    println!("You must pass the email '{}' to the 'Create Email Identity' option before sending an email to this address\n",email.yellow().bold());
                                }
                                (true, false, true) => {
                                    ses_ops
                                        .create_email_contact_with_verification(&email, None)
                                        .await;
                                }
                                _ => println!("{}\n", "No email is received".red().bold()),
                            }
                        }
                        "Create Email Identity\n" => {
                            let email = Text::new("Enter the email\n")
                                .with_placeholder(
                                    "Emails should be without quotation marks around them\n",
                                )
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt()
                                .unwrap();
                            match email.is_empty() {
                                false => {
                                    ses_ops.create_email_identity(&email).await;
                                }
                                true => println!("{}\n", "Email Can't be empty"),
                            }
                        }

                        "Email Verification\n" => {
                            let email_to_verify =
                                Text::new("Enter the email to check the identity\n")
                                    .with_placeholder("Only verified email can receive email\n")
                                    .with_formatter(&|str| format!(".....{str}.....\n"))
                                    .prompt()
                                    .unwrap();
                            match email_to_verify.is_empty() {
                                false => {
                                    let available_email_identies =
                                        ses_ops.retrieve_emails_from_list_email_identities().await;
                                    if available_email_identies.contains(&email_to_verify) {
                                        match ses_ops.is_email_verfied(&email_to_verify).await {
                                            true => {
                                                let email_to_verify =
                                                    email_to_verify.green().bold();
                                                println!("The email address {email_to_verify} has been verified\n");
                                                println!(" You can use it to receive messages or as a 'from' address\n");
                                            }
                                            false => {
                                                let email_to_verify =
                                                    email_to_verify.green().bold();
                                                println!("The email address {email_to_verify} is not verified\n");
                                                println!("Therefore, you can't use it to send emails ('from' address) or receive messages\n");
                                            }
                                        }
                                    } else {
                                        println!(
                                            "No identity was found for the email '{}'",
                                            email_to_verify
                                        );
                                        println!("{}\n","Please execute the 'create email identity' option before verifying this email".yellow().bold());
                                    }
                                }
                                true => {
                                    println!("{}\n", "The email can't be empty".red().bold())
                                }
                            }
                        }

                        "Retrieve emails from the provided list\n" => {
                            let get_contact_list_name = ses_ops.get_list_name();
                            let get_contact_list_name =
                                format!("Default contact list name: {}\n", get_contact_list_name);
                            let list_name = Text::new("Please enter the name of the list for which you'd like to receive these emails in PDF and text formats\n")
                               .with_placeholder(&get_contact_list_name)
                               .with_formatter(&|str| format!(".....{str}....."))
                               .prompt_skippable()
                               .unwrap()
                               .unwrap();
                            match list_name.is_empty() {
                                false => {
                                    ses_ops
                                        .writing_email_addresses_from_provided_list_as_text_pdf(
                                            Some(&list_name),
                                        )
                                        .await;
                                }
                                true => {
                                    ses_ops
                                        .writing_email_addresses_from_provided_list_as_text_pdf(
                                            None,
                                        )
                                        .await;
                                }
                            }
                        }
                        "Default Values\n" => {
                            let default_list_name = ses_ops.get_list_name().green().bold();
                            let default_template_name = ses_ops.get_template_name().green().bold();
                            let default_from_address = ses_ops.get_from_address().green().bold();
                            println!("Default Contact List Name: {default_list_name}\n");
                            println!("Default Template Name: {default_template_name}\n");
                            println!("Default from_address is: {default_from_address}\n");

                            println!("{}\n","Be sure to check the placeholder for default values, allowing you to skip using the default value".bright_blue());
                        }

                        "Send a Single Simple Email\n" => {
                            let email = Text::new("Enter the email..\n")
                        .with_formatter(&|str| format!(".....{str}....."))
                        .with_placeholder(
                            "The provided email should be verified through the 'Create Email Identity' option",
                        )
                        .prompt()
                        .unwrap();
                            let email_contacts =
                                ses_ops.retrieve_emails_from_list_email_identities().await;
                            match email.is_empty() {
                                false => {
                                    if email_contacts.contains(&email) {
                                        let subject = Text::new("Enter the subject of Email\n")
                                .with_placeholder(
                                    "Eg: For testing purposes, we have launched a new product",
                                )
                                .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();

                                        let defaul_from_address = ses_ops.get_from_address();

                                        let default_from_address =format!("Your 'from_address' needs to be verified, which is typically your email\nand the default 'from_address' is {}",defaul_from_address);

                                        let from_address = Text::new("Please enter the 'From' address, or press Enter to use the default 'From' address, if one is available in the placeholder\n")
                    .with_placeholder(&default_from_address)
                    .with_formatter(&|str| format!(".....{str}....."))
                    .prompt_skippable()
                    .unwrap()
                    .unwrap();
                                        let body_info = Confirm::new("You can either provide the email body from a local file path or any S3 object URLs can be passed, and they should be publicly accessible. Not all links provide the exact content we requested\n")
                .with_formatter(&|str| format!(".....{str}....."))
                .with_placeholder("Please respond with 'Yes' to provide a local file or 'No' to provide a S3 Object Url link\n")
                .prompt()
                .unwrap();
                                        let from_address = match from_address.is_empty() {
                                            true => None,
                                            false => Some(from_address.as_str()),
                                        };
                                        match (subject.is_empty(), body_info) {
                                            (false, true) => {
                                                let body_path = Text::new("Please provide the path to the body of a simple email content file\n")
                    .with_formatter(&|str| format!(".....{str}....."))
                    .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                    .with_help_message("You can download a example simple email content here https://tinyurl.com/mr22bh4f")
                    .prompt()
                    .unwrap();
                                                let mut reading_simple_data = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&body_path)
                    .expect("Error opening the simple email file path you specified");
                                                let mut body_data = String::new();
                                                reading_simple_data
                                                    .read_to_string(&mut body_data)
                                                    .expect("Error while reading to string\n");
                                                let simple_email =
                                                    SimpleMail::builder(&body_data, &subject)
                                                        .build();

                                                ses_ops
                        .send_mono_email(&email, Simple_(simple_email),from_address)
                        .await
                        .send()
                        .await
                        .map(|_|{
                            let colored_email = email.green().bold();
                            println!("A simple email has been successfully sent to '{}'\n{}\n",colored_email,"Please check your inbox to view it".yellow().bold())
                          })
                        .expect("Error while Sending Simple Email\n");
                                            }
                                            (false, false) => {
                                                let body_link = Text::new("Please provide the link to the body of a simple email content file\n")
                                           .with_formatter(&|str| format!(".....{str}.....\n"))
                                           .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                                           .with_help_message("Visit this link https://tinyurl.com/3bx4yz6v to obtain an S3 URL that contains the simple email content")
                                           .prompt()
                                         .unwrap();
                                                match get(&body_link).await{
                                 Ok(body) => {
                        let body_data = body.text().await.expect("Error while getting text data\n");
                        let x: &[_] = &['\n','\r',' ','\x1b','\u{20}','\u{7f}','\u{80}'];
                        let body_data = body_data.trim_matches(x);
                        let simple_email = SimpleMail::builder(
                          body_data,
                          &subject
                          )
                          .build();
                        
                         ses_ops
                        .send_mono_email(&email, Simple_(simple_email),from_address)
                        .await
                        .send()
                        .await
                        .map(|_|{
                            let colored_email = email.green().bold();
                            println!("A simple email has been successfully sent to '{}'\n{}\n",colored_email,"Please check your inbox to view it".yellow().bold())
                           })
                        .expect("Error While Sending Simple Email\n");

                    }
                    Err(_) => println!("{}\n","The provided link doesn't seem to be working. Could you please check the link and try again?".red().bold())
                }
                                            }
                                            _ => println!("{}\n", "Subject can't be empty"),
                                        }
                                    } else {
                                        println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                    }
                                }
                                true => {
                                    println!("{}\n", "Email can't be empty".red().bold());
                                }
                            }
                        }
                        "Get Email Identities\n" => {
                            ses_ops.writing_email_identies_details_as_text_pdf().await;
                            println!("{}\n","This option only returns the emails that are created either via the 'Create Email Identity' option or\nby choosing 'yes' in the 'Add an Email to the list' option when asked to send a verification email".yellow().bold());
                        }
                        "Send a Single Templated Email\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_template_name = ses_ops.get_template_name();
                            let default_template_name =
                                format!("Default template name is: {}", get_template_name);
                            let default_from_address =
                                format!("Default from_address is: {}", get_from_address);
                            let email = Text::new("Enter the email you wish to send\n")
                                .with_placeholder("The email must be verified")
                                .with_formatter(&|str| format!(".....{str}....."))
                                .prompt()
                                .unwrap();
                            let email_contacts =
                                ses_ops.retrieve_emails_from_list_email_identities().await;

                            match email.is_empty() {
                                false => {
                                    if email_contacts.contains(&email) {
                                        let template_name = Text::new(
                                    "Please enter the template name you want to use for the email\n",)
                                .with_placeholder(&default_template_name)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .with_help_message(
                                    "The template name must exist, and the variables should be specified as key-value pairs according to the template\n",
                                )
                                .prompt()
                                .unwrap();
                                        let from_address = Text::new("Enter the from address\n")
                                            .with_placeholder(&default_from_address)
                                            .with_formatter(&|str| format!(".....{str}.....\n"))
                                            .prompt_skippable()
                                            .unwrap()
                                            .unwrap();
                                        let placeholder_info = format!(
                                "The template variables should reflect the '{}' template",
                                template_name
                            );
                                        let template_path = Text::new(
                        "You can provide the path to the template data in JSON format\n",
                    )
                    .with_formatter(&|str| format!(".....{str}.....\n"))
                    .with_placeholder(&placeholder_info)
                    .prompt()
                    .unwrap();

                                        match (
                                            template_name.is_empty(),
                                            from_address.is_empty(),
                                            template_path.is_empty(),
                                        ) {
                                            (false, false, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");

                                                let email_content = TemplateMail::builder(
                                                    &template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        Some(&from_address),
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            (false, true, false) => {
                                                if email_contacts.contains(&email) {
                                                    let mut reading_template_data = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(&template_path)
                            .expect(
                                "Error opening the Template file path you specified",
                            );
                                                    let mut template_data = String::new();
                                                    reading_template_data
                                                        .read_to_string(&mut template_data)
                                                        .expect("Error while reading data\n");
                                                    let email_content = TemplateMail::builder(
                                                        &template_name,
                                                        &template_data,
                                                    )
                                                    .build();
                                                    ses_ops
                                                        .send_mono_email(
                                                            &email,
                                                            Template_(email_content),
                                                            None,
                                                        )
                                                        .await
                                                        .send()
                                                        .await
                                                        .map(|_| {
                                                            let colored_email =
                                                                email.green().bold();
                                                            println!(
                                                    "The template email is send to: {}\n",
                                                    colored_email
                                                )
                                                        })
                                                        .expect(
                                                            "Error while sending template mail\n",
                                                        );
                                                } else {
                                                    println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                                }
                                            }
                                            (true, true, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");
                                                let email_content = TemplateMail::builder(
                                                    &get_template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        None,
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            (true, false, false) => {
                                                let mut reading_template_data = OpenOptions::new()
                                                    .read(true)
                                                    .write(true)
                                                    .open(&template_path)
                                                    .expect(
                                                        "Error opening the file path you specified",
                                                    );
                                                let mut template_data = String::new();
                                                reading_template_data
                                                    .read_to_string(&mut template_data)
                                                    .expect("Error while reading data\n");
                                                let email_content = TemplateMail::builder(
                                                    &get_template_name,
                                                    &template_data,
                                                )
                                                .build();
                                                ses_ops
                                                    .send_mono_email(
                                                        &email,
                                                        Template_(email_content),
                                                        Some(&from_address),
                                                    )
                                                    .await
                                                    .send()
                                                    .await
                                                    .map(|_| {
                                                        let colored_email = email.green().bold();
                                                        println!(
                                                            "The template email is send to: {}\n",
                                                            colored_email
                                                        )
                                                    })
                                                    .expect("Error while sending template mail\n");
                                            }
                                            _ => {
                                                println!("{}\n","Please ensure that the fields are not empty, and then try again.".red().bold());
                                            }
                                        }
                                    } else {
                                        println!("The provided email '{}' has not been verified. Please execute the 'Create Email Identity' option to verify the email address, and then proceed with this one\n",email.yellow().bold());
                                    }
                                }
                                true => println!("{}\n", "Email can't be empty".red().bold()),
                            }
                        }
                        "Send a Bulk of Simple Emails\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_list_name = ses_ops.get_list_name();

                            let default_from_address =
                                format!("Default from_address is: {}\n", get_from_address);
                            let default_list_name =
                                format!("Default list name is: {}\n", get_list_name);

                            let list_name = Text::new("Please provide the name of the Contact List where all your verified emails are stored\n")
            .with_placeholder(&default_list_name)
            .with_formatter(&|input| format!("The Simple Email Content will be sent to each email address in the: {input} Contact List\n"))
            .prompt_skippable()
            .unwrap()
            .unwrap_or(ses_ops.get_list_name().into());
                            let body_info = Confirm::new("You can either provide the email body from a local file path or any S3 object URLs can be passed, and they should be publicly accessible. Not all links provide the exact content we requested\n")
        .with_formatter(&|str| format!(".....{str}....."))
        .with_placeholder("Please respond with 'Yes' to provide a local file or 'No' to provide a S3 Object Url link\n")
        .with_help_message("The body data is the same for all emails in the list")
        .prompt()
        .unwrap();
                            //println!("The emails in the provided list should be verified; otherwise, the operation may fail. You can choose to skip this step by providing empty input and then proceed with the 'Get Email Identities' option\n".yellow().bold());
                            let subject = Text::new("Please enter the subject content that all your subscribers should be aware of\n")
            .with_placeholder("The subject is the same for all emails\n")
            .with_formatter(&|str| format!(".....{str}.....\n"))
            .prompt()
            .unwrap();
                            let from_address = Text::new("Enter the from address\n")
                                .with_placeholder(&default_from_address)
                                .with_formatter(&|str| format!(".....{str}.....\n"))
                                .prompt_skippable()
                                .unwrap()
                                .unwrap();

                            match (subject.is_empty(), body_info) {
                                (false, true) => {
                                    let body_path = Text::new("Please provide the path to the body of a simple email content file\n")
                .with_formatter(&|str| format!(".....{str}....."))
                .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                .with_help_message("You can download a example simple email content here https://tinyurl.com/mr22bh4f")
                .prompt()
                .unwrap();
                                    let body_data = std::fs::read_to_string(&body_path).expect(
                                        "Error Opening the simple email file path you specified\n",
                                    );
                                    let simple_data = SimpleMail::builder(&body_data, &subject);

                                    match (list_name.is_empty(), from_address.is_empty()) {
                                        (false, false) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    Some(&from_address),
                                                    Some(&list_name),
                                                )
                                                .await;
                                        }
                                        (true, true) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    None,
                                                    None,
                                                )
                                                .await;
                                        }
                                        (false, true) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    None,
                                                    Some(&list_name),
                                                )
                                                .await;
                                        }
                                        (true, false) => {
                                            ses_ops
                                                .send_multi_email_with_simple(
                                                    simple_data,
                                                    Some(&from_address),
                                                    None,
                                                )
                                                .await;
                                        }
                                    }
                                }
                                (false, false) => {
                                    let body_link = Text::new("Please provide the link to the body of a simple email content file\n")
                .with_formatter(&|str| format!(".....{str}....."))
                .with_placeholder("Any file extension is acceptable as long as it can be read and contains only text content or an HTML body, without any template variables\n")
                .with_help_message("Visit this link https://tinyurl.com/3bx4yz6v to obtain an S3 URL that contains the simple email content")
                .prompt()
                .unwrap();
                                    match get(&body_link).await{
                    Ok(body) => {
                        let body_data = body.text().await.expect("Error while getting text data\n");
                        let x: &[_] = &['\n','\r',' ','\x1b','\u{20}','\u{7f}','\u{80}'];
                        let body_data = body_data.trim_matches(x);
                        let simple_data = SimpleMail::builder(&body_data, &subject);
                        match (list_name.is_empty(),from_address.is_empty()){
                            (false,false) => {
                                 ses_ops
                                .send_multi_email_with_simple(simple_data, Some(&from_address), Some(&list_name))
                                .await; 
                            }
                            (true,true) =>{
                                ses_ops
                                .send_multi_email_with_simple(simple_data, None, None)
                                .await;
                            }
                            (false,true) =>{
                                ses_ops
                                .send_multi_email_with_simple(simple_data, None, Some(&list_name))
                                .await;
                            }
                            (true,false) =>{
                               ses_ops
                                .send_multi_email_with_simple(simple_data, Some(&from_address), None)
                                .await;
                            }
                          }

                    }
                    Err(_) => println!("{}\n","The provided link doesn't seem to be working. Could you please check the link and try again?".red().bold())
                }
                                }
                                _ => {
                                    println!(
                                        "{}\n",
                                        "Email,subject or body can't be empty".red().bold()
                                    );
                                }
                            }
                        }

                        "Send a Bulk of Templated Emails\n" => {
                            let get_from_address = ses_ops.get_from_address();
                            let get_template_name = ses_ops.get_template_name();
                            let get_list_name = ses_ops.get_list_name();

                            use std::env::var;
                            match (var("TEMPLATE_NAME"), var("FROM_ADDRESS"), var("LIST_NAME")) {
                                (Ok(_), Ok(_), Ok(_)) => {
                                    println!(
                                        "Template Name: {}\nFrom Address: {}\nList Name: {}\n",
                                        get_template_name.green().bold(),
                                        get_list_name.green().bold(),
                                        get_from_address.green().bold()
                                    );
                                    ses_ops.send_bulk_templated_emails().await;
                                }
                                _ => {
                                    println!(
                                        "{}\n",
                                        "This information is for Debugging Purposes"
                                            .yellow()
                                            .bold()
                                    );
                                    println!(
                                        "Template Name: {}\nFrom Address: {}\nList Name: {}\n",
                                        get_template_name.green().bold(),
                                        get_list_name.green().bold(),
                                        get_from_address.green().bold()
                                    );
                                }
                            }
                        }
                        "Common Errors\n" => {
                            let possible_errors = include_str!("./assets/possible_errors.txt")
                                .yellow()
                                .italic()
                                .bold();
                            println!("{}\n", possible_errors);
                        }
                        "Return to the Main Menu\n" => continue 'main,

                        _ => {}
                    }
                }
            }
            "Quit the application\n" => {
                credential.empty();
                break 'main;
            }
            _other => {
                println!("This branch never reach..");
            }
        }
    }
}
fn global_render_config() -> RenderConfig {
    let mut config = RenderConfig::default()
        .with_prompt_prefix(Styled::new("⚙️").with_fg(inquire::ui::Color::DarkBlue))
        .with_text_input(StyleSheet::new().with_fg(inquire::ui::Color::LightGreen))
        .with_highlighted_option_prefix(Styled::new("👉"))
        .with_help_message(StyleSheet::new().with_fg(inquire::ui::Color::DarkYellow));
    config.answer = StyleSheet::new()
        .with_attr(Attributes::BOLD)
        .with_fg(inquire::ui::Color::DarkGreen);
    config
}
