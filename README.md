# vibe token

Repository containing the code of the vibe token and the token sale canister.

## Brief description

Vibe token will be used in the [vibeverse](http://vibeverse.xyz/) app as a token to pay fees when creating a collection or minting an nft. Since we want as much feedback as possible on vibeverse during the first month or two of the official deployment to production the token won't be required for using the app.

## Fee payment

As mentioned the vibe token will be used to pay fees when using [vibeverse](http://vibeverse.xyz/), but we did not mention earlier how these fees will be used. The plan is to have the fees distributed amongst all of the token holders. This means that whenever someone does something on vibeverse that requires fee payment everyone who owns some vibe tokens will get the equivalent portion of the fee.
This will be distributed based on the percentage of what the user holds from the total market cap.

## Prerequisets to local development

Before following the instructions we recommend following this documentation to set up all the prerequisets. Since the backend is written in rust make sure you have all of the necessities that are described here

## Local development
**Follow the following steps to locally deploy the frontend:**
```
npm i
```
```
npm run dev # The website will be accessible at http://localhost:3000/
# OR
dfx start --background
dfx deploy
```
**Follow the following steps to locally deploy the backend:**
```
# To compile the backend code:
cargo build
```
```
# To run all the backend tests:
cargo test
```
```
# To deploy
dfx start --background
dfx deploy
```
