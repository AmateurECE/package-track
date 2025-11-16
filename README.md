# A Package Maintainer's Best Friend!

I maintain a few Yocto recipes that are used in connected devices. There's no
one to complain when the recipe is out of date except me, and I don't care to
remember to check the repositories for new versions.

This application is meant to be triggered by a cron job or systemd timer. When
it runs, it reaches out to remote repositories where open source projects are
maintained, and checks for the presence of new release tags. If there has been
a new tag published, it sends an email to the configured recipient with the
name of the component, the affected software project, the current version of
that dependency, and the new version. Currently, Git is the only supported
repository type.

`package-track` requires a PostgreSQL database. There is a `Containerfile` to
build an OCI image for running under Docker or Podman.

# Building

Locally:

```
cargo build
```

Container image. Note that `docker` can be used instead, if desired.

```
podman build -t package-track:latest .
```

# Running

`package-track` requires the following environment variables to be set:

* `MAIL_DOMAIN`: The sending email domain. This is used to set the `From:`
  header in the email, so it should be a domain that you have authority over.
* `MAIL_RECIPIENT`: Email address of the recipient.
* `MAIL_RELAY_HOST`: Hostname of an SMTP server to send mail to.
  `package-track` sends mail unencrypted on port 25, so it's recommended that
  this either be `localhost` or a host on non-public network.
* `DATABASE_URL`: URL of the PostgreSQL database. Must be of the form
  `postgres://user:password@hostname/database`.
