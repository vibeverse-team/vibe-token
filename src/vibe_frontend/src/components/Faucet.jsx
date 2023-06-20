import React, { useState } from "react";

function Faucet() {
  const [isDisabled, setDisabled] = useState(false);
  const [buttonText, setText] = useState("Claim my VIBE Tokens")

  async function handleClick(event) {
    setDisabled(true)
    //const result = await token.payOut();
    setText(result);
  }

  return (
    <div className="">
      <h2 className="gradient-vibe">
        You just gained VIP Access to Vibeverse!
      </h2>
      {/* <label className="gradient-vibe-2">Get your free Vibeverse tokens here! Claim 1000 VIBE coins to your account.</label> */}
      <p className="">
        <a>
        <span className="gradient-button" disabled={isDisabled} id="btn-payout" onClick={handleClick}>
          {buttonText}
        </span>
        </a>
      </p>
    </div>
  );
}

export default Faucet;