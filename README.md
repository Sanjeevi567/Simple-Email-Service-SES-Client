### Simple Email Service<a name="ses"></a>

**Supported Operations in [SESV2](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_Operations.html) APIs:**

- [CreateContact](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_CreateContact.html) - This operation allows you to create a new email contact and add the contact to your contact list.

- [CreateContactList](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_CreateContactList.html) - This operation enables you to create a new contact list name to store all your emails for future email campaigns. Note that only one contact list per region is allowed.

- [CreateEmailIdentity](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_CreateEmailIdentity.html) - Use this operation to send a verification email to a given email address. This API sends a verification email since only verified emails can receive emails and be added to the contact list. You have two options when adding an email to the list: you can either add the email to the list and then send a verification email, or you can simply add the email to the list. If you choose the latter option, you must execute the "Create Email Identity" operation separately to send a verification email. Each option includes a help message at the end to guide you through the process, even if you haven't read the documentation.

- [CreateEmailTemplate](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_CreateEmailTemplate.html) - This operation creates an HTML and subject template, optionally allowing you to set a text-only body for recipients who don't support HTML. Both the HTML and subject parts can contain template variables to personalize the email. However, do not use HTML-like formatting in the subject part. You can find example HTML and subject body templates [here](https://github.com/Sanjuvi/Simple-Email-Service-SES-Client/blob/main/src/assets/template.html) and [here](https://github.com/Sanjuvi/Simple-Email-Service-SES-Client/blob/main/src/assets/subject.html) respectively, to help you create your own templates. You can have multiple email templates under the same credentials.

- [DeleteContact](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DeleteContact.html) - Use this operation to delete an email contact from the list. For deleting a verified identity, you should use the "Delete Email Identity" operation, which cannot be used directly at this time.

- [DeleteContactList](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DeleteContactList.html) - This operation deletes a contact list, including all the emails in that list if one is available.

- [DeleteEmailIdentity](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DeleteEmailIdentity.html) - Delete an email identity created using the "Delete Email Identity" option. This operation cannot be used directly.

- [DeleteEmailTemplate](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_DeleteEmailTemplate.html) - Delete the template associated with the given template name. The CLI option will download the email template to your local computer before deleting it.

- [GetEmailIdentity](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_GetEmailIdentity.html) - This operation is not consumed directly but is used to generate all email identities, both in text and PDF formats, in the "Get Email Identities" option.

- [GetEmailTemplate](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_GetEmailTemplate.html) - Download the email template associated with a given template name. The placeholder will show you all available template names in your credentials.

- [ListContactLists](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_ListContactLists.html) - This API is used for placeholder information and also within other options to prevent errors when providing a nonexistent contact list name without executing an operation.

- [ListContacts](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_ListContacts.html) - This API is used within different options to provide informative error messages. You can download all the emails in the given contact list by executing the "Retrieve Emails from the Provided List" option, which generates both a text and PDF file with the emails.

- [ListEmailIdentities](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_ListEmailIdentities.html) - This operation is not consumed directly but is used to prevent errors in other options. For example, it helps prevent the creation of email identities for the same email address, which would result in an error. This is checked before passing the email into the "Create Email Identity" or "Get Email Identity" operations.

- [ListEmailTemplates](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_ListEmailTemplates.html) - This operation is not consumed directly. Options starting with "List*" are used to avoid errors in API calls. The API response is used when providing a nonexistent template name without executing an operation, allowing you to continue using the application without crashing.

- [SendEmail](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_SendEmail.html) - You can use this operation to send a simple email without creating an email template, but the email must be verified. There is no need to use the [SendBulkEmail](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_SendBulkEmail.html) API to send multiple emails.

- [SendCustomVerificationEmail](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_SendCustomVerificationEmail.html) - This operation exists but is not used. It allows you to send a custom verification email. Due to the free trial version limitations, I cannot use and test both [CreateCustomVerificationEmailTemplate](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_CreateCustomVerificationEmailTemplate.html) and [SendCustomVerificationEmail](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_SendCustomVerificationEmail.html) to demonstrate their functionality.

- [UpdateEmailTemplate](https://docs.aws.amazon.com/ses/latest/APIReference-V2/API_UpdateEmailTemplate.html) - This operation updates an existing email template.


### Documentation:
