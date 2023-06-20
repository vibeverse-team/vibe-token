import React from 'react';
import PropTypes from 'prop-types';

// CSS
import styles from './AuthModalBody.module.scss';

import { useProviders, useConnect } from '@connect2ic/react';

//Components
import Button from '../../Button/Button';
import Title from '../../Title';

const AuthModalBody = ({
  onClickConnectWalletButton,
}) => {

  const providers = useProviders();

  const { connect } = useConnect({
    onConnect: (data) => {
      console.log("Signed in.");
      console.log(data);
      onClickConnectWalletButton(data);
    },
    onDisconnect: () => {
      console.log("Signed out.")
    }
  })

  return (
    <div className="ml-8">  
    <div className={styles.authModalBody}>
      <h1 className="gradient-text">VIBE Token</h1>
      <button className="yellow-button">Test</button>
    </div>     
 
    <div className={styles.authModalBody}>
      <div className={styles.walletConnectContainer}>
        <Title type="secondary">Connect wallet</Title>
        {providers.map(wallet => (
          <div className="mb-2" key={wallet.meta.id}>
            <Button
              label={wallet.meta.name}
              onClick={() => connect(wallet.meta.id)}
            />
          </div>
        ))}
      </div>
    </div>
    </div>
  );
};

AuthModalBody.propTypes = {
  signInData: PropTypes.exact({
    username: PropTypes.string.isRequired,
    password: PropTypes.string.isRequired,
  }).isRequired,
  setSignInData: PropTypes.func.isRequired,
  onSubmitSignUpForm: PropTypes.func,
  onSubmitSignInForm: PropTypes.func,
  onClickConnectWalletButton: PropTypes.func,
};

export default AuthModalBody;
