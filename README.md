# Stripe with Discord Automation Integration

This project integrates Stripe with Discord automation. It uses a Rust SDK to listen to Stripe webhooks and then assigns roles in Discord and sends emails via a chosen email provider.

## Required Environment Variables
- `STRIPE_WEBHOOK_SECRET`
- `STRIPE_PRIVATE_API_KEY`
- `STRIPE_PUBLISH_KEY`
- `SUPABASE_URL`
- `SUPABASE_KEY`
- `RESEND_API_KEY`
- `SMTP_HOST`
- `SMTP_PORT`
- `SMTP_EMAIL_ADDRESS`
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_EMAIL`



## Automatic Emails
When automatic emails are enabled, you can choose between Resend, SMTP. Templates go in HTML format in `./email/templates`.

### Dynamically populating emails
You can use these pre-built placeholders that are extracted from the Stripe payment to customize and design your email template around these with no additional effort.

For placeholders:
- First name: `{{FirstName}}`
- Email: `{{Email}}`
- Full name: `{{FullName}}`
- Payment amount: `{{PaymentAmount}}`
- Purchase product name: `{{ProductName}}`

These are used to personalize emails and use payment-oriented references.

### Picking an email provider
Pass either `resend` or `smtp` in the email config


### Resend (Email option 1 - Serverless)
Resend is a free email provider that allows you to send 3k emails per month for free. You can sign up for an account [here](https://resend.io/).

Required environment variables for Resend:
- `resend_api_key`
- `resend_email`

Making `resend` your chosen email provider:
```yaml
email:
  Provider: resend
```

Setting the correct environment variables for Resend:
```env
RESEND_API_KEY=
RESEND_EMAIL=
```

### SMTP (Email option 2)
SMTP is a protocol that allows you to send emails from virtually any supported SMTP provider, You will need to provide the following environment variables for SMTP:
- `smtp_host`
- `smtp_port`
- `smtp_email_address`

Making `smtp` your chosen email provider:
```yaml
email:
  Provider: smtp
```

Setting the correct environment variables for SMTP:
```env
SMTP_HOST=
SMTP_PORT=
SMTP_EMAIL_ADDRESS=
```

### SES, Amazon Simple Email (Email option 3)
SES is a paid email provider that allows you to send emails from virtually any supported SMTP provider, You will need to provide the following environment variables for SMTP:
- `smtp_host`
- `smtp_port`
- `smtp_email_address`

Making `ses` your chosen email provider:
```yaml
email:
  Provider: ses
```

Setting the correct environment variables for SES:
```env
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
AWS_EMAIL=
```

## Databasing
In the `stripe_discord.yaml` file, you can choose between Sled and Supabase.

Supported databasing instances are:
- `sled` (local)
- `supabase` (online) 


### Sled (Db option 1)
Sled's local so there's no need for any keys or env variables, the downside is that you will need some type of disk retention to keep the database alive.

Making `sled` your chosen provider:
```yaml
db:
  Provider: sled
```

### Supabase (Db option 2)
Supabase's online so you worry about less with a tiny bit of added latency and they have a very generous free tier

Making `supabase` your chosen provider:
```yaml
db:
  Provider: supabase
```
If you do not have a supabase account/database set-up you can make a free account and use their free tier here: https://supabase.io/

If you need a Rust Supabase SDK, you can install my crate [here](https://crates.io/crates/supabase_rs)

Setting the correct environment variables for Supabase:
- `supabase_url`
- `supabase_key`

```env
SUPABASE_URL=
SUPABASE_KEY=
```

### Tests
You can run tests with `cargo test` to check if your configuration is correct.

## CLI
There is a CLI to add more organizations to your Stripe config.

