/* eslint-disable use-isnan */
/* eslint-disable eqeqeq */
import { createActor as createIcoActor, canisterId as icoCanisterId } from "../../../declarations/token_sale/index";
import { createActor as createTokenActor, canisterId as tokenCanisterId } from "../../../declarations/vibe_backend/index";
import { idlFactory } from "../../../declarations/token_sale/token_sale.did.js";
import { idlFactory as wicpIdlFactory } from "../../../declarations/wicp/wicp.did.js";
import { Principal } from "@dfinity/principal";

const WICP = "utozz-siaaa-aaaam-qaaxq-cai";

const useTokenSale = () => {
    const getSymbol = async () => {
        console.log("Getting the symbol");

        const actor = createTokenActor(tokenCanisterId);
        return await actor.icrc1_symbol();
    }

    const getBalance = async (rawPrincipal) => {
        console.log("Getting balance.");
        const actor = createTokenActor(tokenCanisterId);
        try {
            const principal = Principal.from(rawPrincipal);
            const account = {
                owner: principal,
                subaccount: []
            };
            console.log(account);
            console.log("Principal: " + principal);
            return await actor.icrc1_balance_of(account);
        }catch(e) {
            console.error(e);
            throw e;
        }
    }

    const approve = async (provider, amount) => {
        console.log("Approving: $" + amount + "ICP");
        try {
            const customActor = (await provider.activeProvider.createActor(WICP, wicpIdlFactory, {dev: false})).value;
            console.log(customActor);
            const principal = Principal.from(icoCanisterId);
            await customActor.approve(principal, amount);
        }catch(e) {
            console.error(e);
            throw e;
        }
    }

    const buy = async (provider, amount) => {
        console.log("Buying VIBE for $" + amount + "ICP");
        console.log(provider);

        amount = amount * Math.pow(10, 8);
        console.log(amount);

        const customActor = (await provider.activeProvider.createActor(icoCanisterId, idlFactory, {dev: false})).value;
        console.log(customActor);
        try {
            await approve(provider, amount/10);
            return await customActor.buy(amount);
        }catch(e) {
            console.error(e);
            throw e;
        }
    }

    return { approve, buy, getBalance, getSymbol }
};

export default useTokenSale;
