import React, { useState } from "react";
import { Principal } from "@dfinity/principal";
import { vibe_backend, canisterId } from "../../../declarations/vibe_backend";

function Balance() {
  const [inputValue, setInputValue] = useState('');
  const [balanceResult, setBalance] = useState('');
  const [cryptoSymbol, setSymbol] = useState('');
  const [isHidden, setHidden] = useState(true);
  
  async function handleClick() {
    console.log(inputValue);
    console.log(canisterId);
    const principal = Principal.fromText(inputValue);
    const balance = await vibe_backend.icrc1_balance_of({
      owner: principal,
      subaccount: []
    });

    console.log("Balance: " + balance);
    setBalance(balance.toString());

    const symbol = await vibe_backend.icrc1_symbol();
    console.log(symbol);
    setSymbol(symbol);

    setHidden(false);
  }

  return (
    <div className="exchange-container">
      <p>
        <input
          id="balance-principal-id"
          type="text"
          placeholder="Enter a Principal ID"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
        />
      </p>
      <p className="balance-text">
        <a>
        <span
          className="gradient-button buy-button"
          id="btn-request-balance"
          onClick={handleClick}
        >
          Check Balance
        </span>
        </a>
      </p>
      <p className="white_text" hidden={isHidden}> This account has a balance of {balanceResult} {cryptoSymbol}.</p>
    </div>
  );
}

export default Balance;
