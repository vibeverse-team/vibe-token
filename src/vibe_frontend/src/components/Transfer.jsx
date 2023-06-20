import React, { useState } from "react";
import { Principal } from "@dfinity/principal";

function Transfer() {
  const [recipientId, setId] = useState('');
  const [amount, setAmount] = useState('');
  const [isDisabled, setDisable] = useState(false);
  const [feedback, setFeedback] = useState('');
  const [isHidden, setHidden] = useState(true);
  
  async function handleClick() {
    setHidden(true);
    setDisable(true);
    const recipient = await Principal.fromText(recipientId);
    const amountToTransfer = Number(amount);
    //const result = await token.transfer(recipient, amountToTransfer);
    setFeedback(result);
    setHidden(false);
    setDisable(false);
  }

  return (
    <div className="exchange-container">
      <div className="">
        <fieldset>
          <legend className="amount">To Account:</legend>
          <ul>
            <li>
              <input
                placeholder="Enter a Principal ID"
                type="text"
                id="transfer-to-id"
                value={recipientId}
                onChange={(e) => setId(e.target.value)}
              />
            </li>
          </ul>
        </fieldset>
        <fieldset>
          <legend className="amount">Amount:</legend>
          <ul>
            <li>
              <input
                placeholder="Enter a Principal ID"
                type="number"
                id="amount"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
              />
            </li>
          </ul>
        </fieldset>
        <p className="">
          <a>
          <span className="gradient-button buy-button" id="btn-transfer" onClick={handleClick} disabled={isDisabled} >
            Transfer
          </span>
          </a>
        </p>
        <p className="white_text" hidden={isHidden}>{feedback}</p>
      </div>
    </div>
  );
}

export default Transfer;
