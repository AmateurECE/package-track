use lettre::{
    Message,
    message::{Mailbox, SinglePart},
};

use crate::{component::Project, outdated::OutdatedComponent};

pub trait IntoMessage {
    fn into_message(
        self,
        recipient: Mailbox,
        sending_domain: &str,
    ) -> Result<Message, lettre::error::Error>;
}

impl IntoMessage for OutdatedComponent {
    fn into_message(
        self,
        recipient: Mailbox,
        sending_domain: &str,
    ) -> Result<Message, lettre::error::Error> {
        let Self {
            projects,
            latest_version,
            current_version,
            name,
        } = self;
        let subject = format!("package: {} {} is released!", name, latest_version);
        let containment = match (projects.first(), projects.get(1)) {
            (None, None) | (None, Some(_)) => panic!("Components must be owned by a project!"),
            (Some(Project { name, .. }), None) => format!("project {} contains", name),
            (Some(Project { name: one, .. }), Some(Project { name: two, .. }))
                if projects.len() > 2 =>
            {
                format!("projects {}, {} and others contain", one, two)
            }
            (Some(Project { name: one, .. }), Some(Project { name: two, .. })) => {
                format!("projects {} and {} contain", one, two)
            }
        };
        let contents = format!(
            "{} {}, but {} has just been released.",
            containment, current_version, latest_version
        );
        let singlepart = SinglePart::builder().body(contents);
        Message::builder()
            .from(format!("packager@{}", sending_domain).parse().unwrap())
            .to(recipient)
            .subject(subject)
            .singlepart(singlepart)
    }
}
