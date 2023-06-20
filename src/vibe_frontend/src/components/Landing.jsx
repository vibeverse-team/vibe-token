import React, { useState } from "react";

import { motion } from "framer-motion";
import EarthCanvas from "../canvas/Earth";
import { slideIn } from "../utils/motion";

import vibe from "../../assets/vibe.png"
import dfinity from "../../assets/dfinity.png"

import Faucet from "./Faucet";
import Balance from "./Balance";
import Transfer from "./Transfer";
import RequireWeb3Auth from "./Web3Authorization/RequireWeb3Auth/RequireWeb3Auth";

import useTokenSale from "../hooks/useTokenSale";

function LandingWrapper() {
    return (
        <RequireWeb3Auth>
            <Landing />
        </RequireWeb3Auth>
    )
}

function Landing({provider}) {
    const [icpAmount, setIcpAmount] = useState(0);
    const [vibeAmount, setVibeAmount] = useState(0);

    const onIcpAmountChange = (e) => {
        setIcpAmount(e.target.value);
        setVibeAmount(e.target.value * 10);
    }

    const {buy} = useTokenSale();

    const handleBuy = async () => {
        console.log(provider);
        const res = await buy(provider, icpAmount);
        console.log(res);
    }

    return (
        <>
        <div className="landing">
            <h1 className="gradient-text">VIBE Token</h1>
        </div>
        <div className="animation_planet">
            <motion.div
                    variants={slideIn("right", "tween", 0.2, 1)}
                    className="planet"
                    // className='xl:flex-1 xl:h-full md:h-[1100px] h-[700px]'
            >
                <EarthCanvas />
            </motion.div>
            <a href="#exchange">
                <span className="gradient-button">👉 BUY 👈</span>
            </a>
        </div>
        <div className="landing" id="exchange">
            <div className="exchange-container">
            <div className="exchange-icp">
                <img  src={dfinity} alt="Dfinity Logo" height="50px" width="100px"/>
                <input
                placeholder="ICP"
                type="text"
                id="exchange-icp"
                onChange={onIcpAmountChange}
                value={icpAmount}
                />
            </div>
            <div className="exchange-vibe">
                <img src={vibe} alt="Vibe Logo" height="100px" width="100px"/>
                <input
                placeholder="VIBE"
                type="text"
                id="exchange-icp"
                readOnly
                value={vibeAmount}
                />
            </div>
            </div>
        </div>
        <a className="buy-token" onClick={handleBuy}>
            <span className="gradient-button buy-button">👉 BUY 👈</span>
        </a>
        <div className="last-section">
            <Faucet />
            <Balance />
        </div>
        <div className="last-section">
            <Transfer />
        </div>
        </>
    )
}

export default LandingWrapper;
